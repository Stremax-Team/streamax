use crate::core::{Result, Error};
use super::{VM, Value};

/// Represents an instruction in the VM
#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack Operations
    Push(Value),
    Pop,
    Dup(u8),
    Swap(u8),
    
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    
    // Logical
    And,
    Or,
    Xor,
    Not,
    
    // Control Flow
    Jump(u32),
    JumpIf(u32),
    Call(u32),
    Return,
    
    // Memory
    Load(u32),
    Store(u32),
    Alloc(u32),
    Free(u32),
    
    // Blockchain
    GetBalance(u32),
    Transfer(u32),
    EmitEvent(u32),
    
    // Contract
    CallContract(u32),
    CreateContract(u32),
    
    // Actor
    SendMessage(u32),
    ReceiveMessage,
    
    // Resource Management
    AcquireResource(u32),
    ReleaseResource(u32),
    
    // Permissions
    CheckPermission(u32),
    GrantPermission(u32),
    RevokePermission(u32),
}

impl Instruction {
    pub fn execute(&self, vm: &mut VM) -> Result<()> {
        match self {
            Instruction::Push(value) => vm.push(value.clone()),
            Instruction::Pop => { vm.pop()?; Ok(()) },
            
            Instruction::Add => {
                let b = vm.pop()?;
                let a = vm.pop()?;
                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => vm.push(Value::Int(a + b)),
                    _ => Err(Error::TypeMismatch),
                }
            },
            
            Instruction::CallContract(addr) => {
                // Check gas
                if vm.context.gas_used >= vm.context.gas_limit {
                    return Err(Error::OutOfGas);
                }
                
                // Check permissions
                if !vm.has_permission(*addr) {
                    return Err(Error::PermissionDenied);
                }
                
                // Perform call
                vm.call_contract(*addr)
            },
            
            Instruction::AcquireResource(id) => {
                // Check if resource is available
                if !vm.is_resource_available(*id) {
                    return Err(Error::ResourceUnavailable);
                }
                
                // Mark resource as acquired
                vm.acquire_resource(*id)
            },
            
            // Implement other instructions...
            _ => Ok(()),
        }
    }
    
    pub fn gas_cost(&self) -> u64 {
        match self {
            Instruction::Push(_) => 1,
            Instruction::Pop => 1,
            Instruction::Add => 1,
            Instruction::Sub => 1,
            Instruction::Mul => 5,
            Instruction::Div => 5,
            Instruction::CallContract(_) => 100,
            Instruction::CreateContract(_) => 1000,
            Instruction::EmitEvent(_) => 100,
            Instruction::Store(_) => 20,
            Instruction::Load(_) => 20,
            _ => 1,
        }
    }
} 