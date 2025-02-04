use crate::core::{Result, Error};

/// Represents the virtual machine state
pub struct VM {
    pub stack: Vec<Value>,
    pub memory: Memory,
    pub context: ExecutionContext,
}

/// Represents a value in the VM
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
}

/// Represents the VM memory
pub struct Memory {
    heap: Vec<u8>,
    allocations: Vec<(usize, usize)>, // (offset, size)
}

/// Represents the execution context
pub struct ExecutionContext {
    pub gas_limit: u64,
    pub gas_used: u64,
    pub depth: u32,
    pub caller: Option<[u8; 32]>,
}

impl VM {
    pub fn new(gas_limit: u64) -> Self {
        VM {
            stack: Vec::new(),
            memory: Memory {
                heap: Vec::new(),
                allocations: Vec::new(),
            },
            context: ExecutionContext {
                gas_limit,
                gas_used: 0,
                depth: 0,
                caller: None,
            },
        }
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<()> {
        // TODO: Implement bytecode execution
        Ok(())
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }
}

/// Trait for implementing custom instructions
pub trait Instruction {
    fn execute(&self, vm: &mut VM) -> Result<()>;
    fn gas_cost(&self) -> u64;
} 