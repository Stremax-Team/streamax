use crate::core::{Result, Error};
use super::instructions::Instruction;
use super::memory::Memory;
use super::context::ExecutionContext;
use std::collections::HashMap;

/// Enhanced VM state with support for new language features
pub struct VM {
    // Basic VM state
    pub stack: Vec<Value>,
    pub memory: Memory,
    pub context: ExecutionContext,
    
    // Program state
    pub program: Vec<Instruction>,
    pub pc: usize,
    
    // Actor system
    pub mailbox: Vec<Message>,
    pub actors: HashMap<ActorId, Actor>,
    
    // Resource management
    pub resources: HashMap<ResourceId, ResourceState>,
    pub permissions: HashMap<PermissionId, Permission>,
    
    // Contract state
    pub storage: HashMap<Vec<u8>, Vec<u8>>,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Address(Address),
    Resource(ResourceId),
    Permission(PermissionId),
}

pub struct Actor {
    id: ActorId,
    state: ActorState,
    mailbox: Vec<Message>,
}

pub struct Message {
    sender: ActorId,
    receiver: ActorId,
    payload: Vec<u8>,
}

pub struct ResourceState {
    owner: Option<ActorId>,
    data: Vec<u8>,
}

pub struct Permission {
    granted_to: ActorId,
    resource: ResourceId,
    capabilities: Vec<Capability>,
}

impl VM {
    pub fn new(program: Vec<Instruction>, gas_limit: u64) -> Self {
        VM {
            stack: Vec::new(),
            memory: Memory::new(),
            context: ExecutionContext::new(gas_limit),
            program,
            pc: 0,
            mailbox: Vec::new(),
            actors: HashMap::new(),
            resources: HashMap::new(),
            permissions: HashMap::new(),
            storage: HashMap::new(),
            events: Vec::new(),
        }
    }
    
    pub fn execute(&mut self) -> Result<()> {
        while self.pc < self.program.len() {
            // Check gas
            if self.context.gas_used >= self.context.gas_limit {
                return Err(Error::OutOfGas);
            }
            
            // Execute instruction
            let instruction = &self.program[self.pc];
            self.context.gas_used += instruction.gas_cost();
            instruction.execute(self)?;
            
            self.pc += 1;
        }
        Ok(())
    }
    
    // Actor System Methods
    pub fn send_message(&mut self, msg: Message) -> Result<()> {
        if let Some(actor) = self.actors.get_mut(&msg.receiver) {
            actor.mailbox.push(msg);
            Ok(())
        } else {
            Err(Error::ActorNotFound)
        }
    }
    
    pub fn process_messages(&mut self) -> Result<()> {
        for actor in self.actors.values_mut() {
            while let Some(msg) = actor.mailbox.pop() {
                self.handle_message(actor.id, msg)?;
            }
        }
        Ok(())
    }
    
    // Resource Management
    pub fn acquire_resource(&mut self, id: ResourceId) -> Result<()> {
        let resource = self.resources.get_mut(&id)
            .ok_or(Error::ResourceNotFound)?;
            
        if resource.owner.is_some() {
            return Err(Error::ResourceUnavailable);
        }
        
        resource.owner = Some(self.context.current_actor);
        Ok(())
    }
    
    pub fn release_resource(&mut self, id: ResourceId) -> Result<()> {
        let resource = self.resources.get_mut(&id)
            .ok_or(Error::ResourceNotFound)?;
            
        if resource.owner != Some(self.context.current_actor) {
            return Err(Error::PermissionDenied);
        }
        
        resource.owner = None;
        Ok(())
    }
    
    // Permission System
    pub fn check_permission(&self, id: PermissionId) -> Result<bool> {
        let permission = self.permissions.get(&id)
            .ok_or(Error::PermissionNotFound)?;
            
        Ok(permission.granted_to == self.context.current_actor)
    }
    
    // Contract Methods
    pub fn call_contract(&mut self, address: Address) -> Result<()> {
        // Save current context
        let caller_context = self.context.clone();
        
        // Create new context for contract call
        self.context = ExecutionContext::new(self.context.gas_limit - self.context.gas_used);
        self.context.caller = Some(caller_context.current_actor);
        
        // Execute contract
        let contract = self.load_contract(address)?;
        self.execute_contract(contract)?;
        
        // Restore context
        self.context = caller_context;
        Ok(())
    }
    
    pub fn emit_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

// Helper types
type ActorId = u32;
type ResourceId = u32;
type PermissionId = u32;
type Address = [u8; 32];

#[derive(Clone)]
pub struct Event {
    pub topic: Vec<u8>,
    pub data: Vec<u8>,
} 