use crate::core::{Result, Error};
use crate::runtime::{VM, Value, Instruction};
use std::collections::{HashMap, HashSet};

/// Debugger for VM inspection and control
pub struct Debugger {
    vm: VM,
    breakpoints: HashSet<usize>,
    watchpoints: HashMap<String, WatchCondition>,
    call_stack: Vec<StackFrame>,
    state: DebuggerState,
}

/// Represents the current state of the debugger
#[derive(Clone, Debug, PartialEq)]
pub enum DebuggerState {
    Running,
    Paused,
    Stepping,
    StepOver,
    StepOut,
    Terminated,
}

/// Represents a stack frame
#[derive(Clone, Debug)]
pub struct StackFrame {
    pub function: String,
    pub pc: usize,
    pub locals: HashMap<String, Value>,
}

/// Watch condition for variables
pub enum WatchCondition {
    Changed,
    Equals(Value),
    GreaterThan(Value),
    LessThan(Value),
    Custom(Box<dyn Fn(&Value) -> bool>),
}

impl Debugger {
    pub fn new(vm: VM) -> Self {
        Debugger {
            vm,
            breakpoints: HashSet::new(),
            watchpoints: HashMap::new(),
            call_stack: Vec::new(),
            state: DebuggerState::Paused,
        }
    }
    
    // Execution Control
    
    pub fn run(&mut self) -> Result<()> {
        self.state = DebuggerState::Running;
        
        while self.state == DebuggerState::Running {
            if !self.step()? {
                self.state = DebuggerState::Terminated;
                break;
            }
            
            // Check breakpoints
            if self.breakpoints.contains(&self.vm.pc) {
                self.state = DebuggerState::Paused;
                break;
            }
            
            // Check watchpoints
            if self.check_watchpoints()? {
                self.state = DebuggerState::Paused;
                break;
            }
        }
        
        Ok(())
    }
    
    pub fn step(&mut self) -> Result<bool> {
        if self.vm.pc >= self.vm.program.len() {
            return Ok(false);
        }
        
        // Execute single instruction
        let instruction = &self.vm.program[self.vm.pc];
        self.vm.context.gas_used += instruction.gas_cost();
        instruction.execute(&mut self.vm)?;
        
        // Update call stack
        match instruction {
            Instruction::Call(_) => {
                self.call_stack.push(self.current_frame()?);
            }
            Instruction::Return => {
                self.call_stack.pop();
            }
            _ => {}
        }
        
        self.vm.pc += 1;
        Ok(true)
    }
    
    pub fn step_over(&mut self) -> Result<()> {
        let current_depth = self.call_stack.len();
        self.state = DebuggerState::StepOver;
        
        while self.state == DebuggerState::StepOver {
            if !self.step()? {
                self.state = DebuggerState::Terminated;
                break;
            }
            
            if self.call_stack.len() <= current_depth {
                self.state = DebuggerState::Paused;
                break;
            }
        }
        
        Ok(())
    }
    
    pub fn step_out(&mut self) -> Result<()> {
        let target_depth = self.call_stack.len().saturating_sub(1);
        self.state = DebuggerState::StepOut;
        
        while self.state == DebuggerState::StepOut {
            if !self.step()? {
                self.state = DebuggerState::Terminated;
                break;
            }
            
            if self.call_stack.len() <= target_depth {
                self.state = DebuggerState::Paused;
                break;
            }
        }
        
        Ok(())
    }
    
    // Breakpoint Management
    
    pub fn add_breakpoint(&mut self, pc: usize) {
        self.breakpoints.insert(pc);
    }
    
    pub fn remove_breakpoint(&mut self, pc: usize) {
        self.breakpoints.remove(&pc);
    }
    
    pub fn add_watchpoint(&mut self, name: String, condition: WatchCondition) {
        self.watchpoints.insert(name, condition);
    }
    
    pub fn remove_watchpoint(&mut self, name: &str) {
        self.watchpoints.remove(name);
    }
    
    // State Inspection
    
    pub fn get_stack_trace(&self) -> Vec<StackFrame> {
        self.call_stack.clone()
    }
    
    pub fn get_local_variables(&self) -> Result<HashMap<String, Value>> {
        self.current_frame().map(|frame| frame.locals)
    }
    
    pub fn get_value(&self, name: &str) -> Result<Option<Value>> {
        self.current_frame()
            .map(|frame| frame.locals.get(name).cloned())
    }
    
    pub fn inspect_memory(&self, address: usize, size: usize) -> Result<Vec<u8>> {
        if address + size > self.vm.memory.size() {
            return Err(Error::MemoryAccessViolation);
        }
        
        Ok(self.vm.memory.read_bytes(address, size))
    }
    
    // Helper Methods
    
    fn current_frame(&self) -> Result<StackFrame> {
        self.call_stack.last()
            .cloned()
            .ok_or(Error::NoActiveFrame)
    }
    
    fn check_watchpoints(&self) -> Result<bool> {
        for (name, condition) in &self.watchpoints {
            if let Some(value) = self.get_value(name)? {
                match condition {
                    WatchCondition::Changed => {
                        // Compare with previous value
                    }
                    WatchCondition::Equals(target) => {
                        if &value == target {
                            return Ok(true);
                        }
                    }
                    WatchCondition::GreaterThan(target) => {
                        // Compare values
                    }
                    WatchCondition::LessThan(target) => {
                        // Compare values
                    }
                    WatchCondition::Custom(f) => {
                        if f(&value) {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        Ok(false)
    }
}

// Debug information
pub struct DebugInfo {
    pub source_map: HashMap<usize, SourceLocation>,
    pub variable_map: HashMap<String, VariableInfo>,
}

pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

pub struct VariableInfo {
    pub ty: String,
    pub stack_offset: usize,
    pub scope_start: usize,
    pub scope_end: usize,
} 