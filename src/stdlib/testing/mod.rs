use crate::core::{Result, Error};
use crate::runtime::VM;
use std::collections::HashMap;

/// Test runner for executing test cases
pub struct TestRunner {
    tests: Vec<TestCase>,
    mocks: HashMap<String, Box<dyn Mock>>,
    fixtures: HashMap<String, Box<dyn Fixture>>,
}

/// Represents a single test case
pub struct TestCase {
    name: String,
    test_fn: Box<dyn Fn(&mut VM) -> Result<()>>,
    properties: Vec<Box<dyn Property>>,
}

/// Trait for property-based testing
pub trait Property {
    fn check(&self, vm: &mut VM) -> Result<bool>;
    fn shrink(&self) -> Option<Box<dyn Property>>;
}

/// Trait for mocking dependencies
pub trait Mock {
    fn call(&self, name: &str, args: &[Value]) -> Result<Value>;
    fn verify_calls(&self) -> Result<()>;
}

/// Trait for test fixtures
pub trait Fixture {
    fn setup(&mut self) -> Result<()>;
    fn teardown(&mut self) -> Result<()>;
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {
            tests: Vec::new(),
            mocks: HashMap::new(),
            fixtures: HashMap::new(),
        }
    }
    
    pub fn add_test<F>(&mut self, name: &str, test_fn: F)
    where
        F: Fn(&mut VM) -> Result<()> + 'static,
    {
        self.tests.push(TestCase {
            name: name.to_string(),
            test_fn: Box::new(test_fn),
            properties: Vec::new(),
        });
    }
    
    pub fn add_property<P: Property + 'static>(&mut self, name: &str, property: P) {
        if let Some(test) = self.tests.iter_mut().find(|t| t.name == name) {
            test.properties.push(Box::new(property));
        }
    }
    
    pub fn add_mock<M: Mock + 'static>(&mut self, name: &str, mock: M) {
        self.mocks.insert(name.to_string(), Box::new(mock));
    }
    
    pub fn run(&mut self) -> TestResults {
        let mut results = TestResults::new();
        
        for test in &self.tests {
            // Setup fixtures
            for fixture in self.fixtures.values_mut() {
                if let Err(e) = fixture.setup() {
                    results.add_failure(&test.name, e);
                    continue;
                }
            }
            
            // Run test
            let mut vm = VM::new(Vec::new(), 1_000_000);
            match (test.test_fn)(&mut vm) {
                Ok(()) => {
                    results.add_success(&test.name);
                    
                    // Check properties
                    for property in &test.properties {
                        match property.check(&mut vm) {
                            Ok(true) => results.add_property_success(&test.name),
                            Ok(false) => {
                                if let Some(shrunk) = property.shrink() {
                                    results.add_property_failure(&test.name, "Property failed with minimal counterexample");
                                } else {
                                    results.add_property_failure(&test.name, "Property failed");
                                }
                            }
                            Err(e) => results.add_property_failure(&test.name, &format!("Property check failed: {}", e)),
                        }
                    }
                }
                Err(e) => results.add_failure(&test.name, e),
            }
            
            // Verify mocks
            for mock in self.mocks.values() {
                if let Err(e) = mock.verify_calls() {
                    results.add_failure(&test.name, e);
                }
            }
            
            // Teardown fixtures
            for fixture in self.fixtures.values_mut() {
                if let Err(e) = fixture.teardown() {
                    results.add_failure(&test.name, e);
                }
            }
        }
        
        results
    }
}

/// Test results collector
pub struct TestResults {
    successes: Vec<String>,
    failures: Vec<(String, Error)>,
    property_successes: Vec<String>,
    property_failures: Vec<(String, String)>,
}

impl TestResults {
    pub fn new() -> Self {
        TestResults {
            successes: Vec::new(),
            failures: Vec::new(),
            property_successes: Vec::new(),
            property_failures: Vec::new(),
        }
    }
    
    pub fn add_success(&mut self, name: &str) {
        self.successes.push(name.to_string());
    }
    
    pub fn add_failure(&mut self, name: &str, error: Error) {
        self.failures.push((name.to_string(), error));
    }
    
    pub fn add_property_success(&mut self, name: &str) {
        self.property_successes.push(name.to_string());
    }
    
    pub fn add_property_failure(&mut self, name: &str, reason: &str) {
        self.property_failures.push((name.to_string(), reason.to_string()));
    }
    
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!("Test Results:\n"));
        summary.push_str(&format!("Successes: {}\n", self.successes.len()));
        summary.push_str(&format!("Failures: {}\n", self.failures.len()));
        summary.push_str(&format!("Property Successes: {}\n", self.property_successes.len()));
        summary.push_str(&format!("Property Failures: {}\n", self.property_failures.len()));
        summary
    }
}

// Example property-based test
pub struct NonNegativeBalance {
    address: Address,
}

impl Property for NonNegativeBalance {
    fn check(&self, vm: &mut VM) -> Result<bool> {
        // Check that balance is never negative
        let balance = vm.get_balance(&self.address)?;
        Ok(balance >= 0)
    }
    
    fn shrink(&self) -> Option<Box<dyn Property>> {
        None // No shrinking for this property
    }
} 