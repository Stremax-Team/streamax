use crate::vm::{VM, Value, Contract};
use crate::stdlib::core::{Error, Result};
use crate::testing::blockchain::{BlockchainTestEnvironment, Account};
use std::collections::HashMap;

/// Contract test configuration
#[derive(Debug, Clone)]
pub struct ContractTestConfig {
    pub mock_external_calls: bool,
    pub record_events: bool,
    pub trace_execution: bool,
    pub coverage_enabled: bool,
}

impl Default for ContractTestConfig {
    fn default() -> Self {
        ContractTestConfig {
            mock_external_calls: true,
            record_events: true,
            trace_execution: false,
            coverage_enabled: true,
        }
    }
}

/// Contract test environment
pub struct ContractTestEnvironment {
    config: ContractTestConfig,
    env: BlockchainTestEnvironment,
    deployed_contracts: HashMap<String, DeployedContract>,
    mocks: HashMap<[u8; 20], MockContract>,
    coverage: Coverage,
}

#[derive(Debug)]
pub struct DeployedContract {
    pub name: String,
    pub address: [u8; 20],
    pub abi: ContractABI,
    pub source_map: SourceMap,
}

#[derive(Debug)]
pub struct MockContract {
    pub address: [u8; 20],
    pub expectations: Vec<MockExpectation>,
    pub calls: Vec<MockCall>,
}

#[derive(Debug)]
pub struct MockExpectation {
    pub method: String,
    pub args: Vec<Value>,
    pub return_value: Value,
    pub times: Option<usize>,
}

#[derive(Debug)]
pub struct MockCall {
    pub method: String,
    pub args: Vec<Value>,
    pub timestamp: u64,
}

#[derive(Debug, Default)]
pub struct Coverage {
    pub lines: HashMap<String, HashSet<usize>>,
    pub branches: HashMap<String, HashSet<usize>>,
    pub functions: HashMap<String, HashSet<String>>,
}

#[derive(Debug)]
pub struct ContractABI {
    pub constructor: Option<ABIFunction>,
    pub functions: HashMap<String, ABIFunction>,
    pub events: HashMap<String, ABIEvent>,
}

#[derive(Debug)]
pub struct ABIFunction {
    pub name: String,
    pub inputs: Vec<ABIParameter>,
    pub outputs: Vec<ABIParameter>,
    pub state_mutability: StateMutability,
}

#[derive(Debug)]
pub struct ABIEvent {
    pub name: String,
    pub inputs: Vec<ABIParameter>,
    pub anonymous: bool,
}

#[derive(Debug)]
pub struct ABIParameter {
    pub name: String,
    pub type_name: String,
    pub components: Option<Vec<ABIParameter>>,
    pub indexed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Debug)]
pub struct SourceMap {
    pub source_files: HashMap<String, String>,
    pub line_mappings: HashMap<usize, (String, usize)>,
    pub function_mappings: HashMap<String, (String, Range)>,
}

#[derive(Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl ContractTestEnvironment {
    pub fn new(config: ContractTestConfig, blockchain_env: BlockchainTestEnvironment) -> Self {
        ContractTestEnvironment {
            config,
            env: blockchain_env,
            deployed_contracts: HashMap::new(),
            mocks: HashMap::new(),
            coverage: Coverage::default(),
        }
    }

    pub fn deploy_contract(&mut self, name: &str, contract: Contract) -> Result<[u8; 20]> {
        // Deploy the contract
        let sender = self.env.get_current_block().number % 10;
        let mut sender_address = [0; 20];
        sender_address[19] = sender as u8;
        
        let address = self.env.deploy_contract(contract.clone(), sender_address)?;
        
        // Store contract info
        self.deployed_contracts.insert(name.to_string(), DeployedContract {
            name: name.to_string(),
            address,
            abi: contract.abi,
            source_map: contract.source_map,
        });
        
        Ok(address)
    }

    pub fn mock_contract<F>(&mut self, address: [u8; 20], setup: F) -> Result<()> 
    where
        F: FnOnce(&mut MockBuilder) -> Result<()>
    {
        let mut builder = MockBuilder::new(address);
        setup(&mut builder)?;
        
        self.mocks.insert(address, builder.build());
        Ok(())
    }

    pub fn call_contract(&mut self, name: &str, method: &str, args: &[Value]) -> Result<Value> {
        let contract = self.deployed_contracts.get(name)
            .ok_or(Error::ContractNotFound)?;
            
        // Check if method exists in ABI
        let abi_function = contract.abi.functions.get(method)
            .ok_or(Error::MethodNotFound)?;
            
        // Validate arguments against ABI
        self.validate_arguments(abi_function, args)?;
        
        // Execute call
        let sender = self.env.get_current_block().number % 10;
        let mut sender_address = [0; 20];
        sender_address[19] = sender as u8;
        
        let result = self.env.call_contract(
            contract.address,
            method,
            args,
            sender_address
        )?;
        
        // Update coverage if enabled
        if self.config.coverage_enabled {
            self.update_coverage(name, method)?;
        }
        
        Ok(result)
    }

    pub fn assert_event_emitted(&self, name: &str, event: &str, args: &[Value]) -> Result<()> {
        let contract = self.deployed_contracts.get(name)
            .ok_or(Error::ContractNotFound)?;
            
        // Check if event exists in ABI
        let abi_event = contract.abi.events.get(event)
            .ok_or(Error::EventNotFound)?;
            
        // Check events
        for emitted in self.env.get_events() {
            if self.event_matches(emitted, abi_event, args) {
                return Ok(());
            }
        }
        
        Err(Error::EventNotFound)
    }

    pub fn get_coverage_report(&self) -> Coverage {
        self.coverage.clone()
    }

    // Private helper methods

    fn validate_arguments(&self, function: &ABIFunction, args: &[Value]) -> Result<()> {
        if args.len() != function.inputs.len() {
            return Err(Error::InvalidArgumentCount);
        }
        
        for (arg, param) in args.iter().zip(function.inputs.iter()) {
            if !self.type_matches(arg, &param.type_name) {
                return Err(Error::TypeMismatch);
            }
        }
        
        Ok(())
    }

    fn type_matches(&self, value: &Value, type_name: &str) -> bool {
        match (value, type_name) {
            (Value::Int(_), "int256") => true,
            (Value::Uint(_), "uint256") => true,
            (Value::Address(_), "address") => true,
            (Value::Bool(_), "bool") => true,
            (Value::Bytes(_), "bytes") => true,
            (Value::String(_), "string") => true,
            _ => false,
        }
    }

    fn event_matches(&self, emitted: &Event, abi: &ABIEvent, args: &[Value]) -> bool {
        if emitted.name != abi.name {
            return false;
        }
        
        if args.len() != abi.inputs.len() {
            return false;
        }
        
        args.iter().zip(abi.inputs.iter()).all(|(arg, param)| {
            self.type_matches(arg, &param.type_name)
        })
    }

    fn update_coverage(&mut self, contract: &str, function: &str) -> Result<()> {
        if let Some(contract_info) = self.deployed_contracts.get(contract) {
            // Update function coverage
            self.coverage.functions
                .entry(contract.to_string())
                .or_default()
                .insert(function.to_string());
                
            // Update line coverage if we have source mapping
            if let Some(range) = contract_info.source_map.function_mappings.get(function) {
                let lines = self.coverage.lines
                    .entry(contract.to_string())
                    .or_default();
                    
                for line in range.start..=range.end {
                    lines.insert(line);
                }
            }
        }
        
        Ok(())
    }
}

pub struct MockBuilder {
    address: [u8; 20],
    expectations: Vec<MockExpectation>,
}

impl MockBuilder {
    fn new(address: [u8; 20]) -> Self {
        MockBuilder {
            address,
            expectations: Vec::new(),
        }
    }
    
    pub fn expect_call(&mut self, method: &str, args: Vec<Value>, return_value: Value) -> &mut Self {
        self.expectations.push(MockExpectation {
            method: method.to_string(),
            args,
            return_value,
            times: None,
        });
        self
    }
    
    pub fn times(&mut self, count: usize) -> &mut Self {
        if let Some(expectation) = self.expectations.last_mut() {
            expectation.times = Some(count);
        }
        self
    }
    
    fn build(self) -> MockContract {
        MockContract {
            address: self.address,
            expectations: self.expectations,
            calls: Vec::new(),
        }
    }
} 