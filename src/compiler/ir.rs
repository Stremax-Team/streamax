use std::collections::HashMap;
use crate::ast;

#[derive(Debug, Clone)]
pub struct Program {
    pub contracts: Vec<Contract>,
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub storage: Vec<StorageSlot>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct StorageSlot {
    pub name: String,
    pub slot: u32,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Instruction>,
    pub locals: Vec<Local>,
    pub is_pure: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct Local {
    pub name: String,
    pub ty: Type,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U256,
    Address,
    Bool,
    String,
    Array(Box<Type>),
    Map { key: Box<Type>, value: Box<Type> },
    Void,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack operations
    Push(Value),
    Pop,
    Dup(u8),
    Swap(u8),
    
    // Memory operations
    Load(u32),  // Load from local variable
    Store(u32), // Store to local variable
    SLoad(u32), // Load from storage
    SStore(u32), // Store to storage
    
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    
    // Comparison
    Eq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    
    // Control flow
    Jump(Label),
    JumpIf(Label),
    Label(Label),
    Call(String, u8), // Function name and number of arguments
    Return,
    
    // Blockchain specific
    EmitEvent(String, u8), // Event name and number of arguments
    NoReentry(Label, Label), // Start and end labels
    
    // Memory management
    Alloc(Type),
    Free,
}

#[derive(Debug, Clone)]
pub enum Value {
    U256(u64), // Simplified for example
    Address([u8; 20]),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub String);

pub struct IRBuilder {
    current_contract: Option<Contract>,
    current_function: Option<Function>,
    label_counter: u32,
    local_counter: u32,
    storage_counter: u32,
}

impl IRBuilder {
    pub fn new() -> Self {
        IRBuilder {
            current_contract: None,
            current_function: None,
            label_counter: 0,
            local_counter: 0,
            storage_counter: 0,
        }
    }

    pub fn build(mut self, ast: &ast::Program) -> Program {
        let mut contracts = Vec::new();
        
        for ast_contract in &ast.contracts {
            self.current_contract = Some(Contract {
                name: ast_contract.name.clone(),
                storage: Vec::new(),
                functions: Vec::new(),
            });
            
            // Convert state variables to storage slots
            for var in &ast_contract.state_vars {
                let slot = StorageSlot {
                    name: var.name.clone(),
                    slot: self.storage_counter,
                    ty: self.convert_type(&var.type_info),
                };
                self.storage_counter += 1;
                self.current_contract.as_mut().unwrap().storage.push(slot);
            }
            
            // Convert functions
            for ast_fn in &ast_contract.functions {
                self.current_function = Some(Function {
                    name: ast_fn.name.clone(),
                    params: ast_fn.parameters.iter()
                        .map(|p| Parameter {
                            name: p.name.clone(),
                            ty: self.convert_type(&p.type_info),
                        })
                        .collect(),
                    return_type: ast_fn.return_type.as_ref()
                        .map(|t| self.convert_type(t)),
                    body: Vec::new(),
                    locals: Vec::new(),
                    is_pure: ast_fn.is_pure,
                });
                
                // Convert function body
                let body = self.convert_block(&ast_fn.body);
                self.current_function.as_mut().unwrap().body = body;
                
                self.current_contract.as_mut().unwrap()
                    .functions.push(self.current_function.take().unwrap());
            }
            
            contracts.push(self.current_contract.take().unwrap());
        }
        
        Program { contracts }
    }

    fn convert_type(&self, ast_type: &ast::Type) -> Type {
        match ast_type {
            ast::Type::U256 => Type::U256,
            ast::Type::Address => Type::Address,
            ast::Type::Bool => Type::Bool,
            ast::Type::String => Type::String,
            ast::Type::Array(t) => Type::Array(Box::new(self.convert_type(t))),
            ast::Type::Map { key_type, value_type } => Type::Map {
                key: Box::new(self.convert_type(key_type)),
                value: Box::new(self.convert_type(value_type)),
            },
            _ => Type::Void, // Handle other cases
        }
    }

    fn convert_block(&mut self, block: &ast::Block) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        
        for stmt in &block.statements {
            instructions.extend(self.convert_statement(stmt));
        }
        
        instructions
    }

    fn convert_statement(&mut self, stmt: &ast::Statement) -> Vec<Instruction> {
        match stmt {
            ast::Statement::Let { name, type_info: _, value } => {
                let mut instructions = self.convert_expression(value);
                let local_index = self.local_counter;
                self.local_counter += 1;
                
                if let Some(ref mut function) = self.current_function {
                    function.locals.push(Local {
                        name: name.clone(),
                        ty: Type::U256, // Infer type from value
                        index: local_index,
                    });
                }
                
                instructions.push(Instruction::Store(local_index));
                instructions
            }
            ast::Statement::Assignment { target, value } => {
                let mut instructions = self.convert_expression(value);
                instructions.extend(self.convert_assignment_target(target));
                instructions
            }
            ast::Statement::Return(Some(expr)) => {
                let mut instructions = self.convert_expression(expr);
                instructions.push(Instruction::Return);
                instructions
            }
            ast::Statement::Return(None) => {
                vec![Instruction::Return]
            }
            ast::Statement::If { condition, then_block, else_block } => {
                let mut instructions = Vec::new();
                let then_label = self.new_label("then");
                let else_label = self.new_label("else");
                let end_label = self.new_label("endif");
                
                // Condition
                instructions.extend(self.convert_expression(condition));
                instructions.push(Instruction::JumpIf(then_label.clone()));
                
                // Else block
                if let Some(else_block) = else_block {
                    instructions.extend(self.convert_block(else_block));
                }
                instructions.push(Instruction::Jump(end_label.clone()));
                
                // Then block
                instructions.push(Instruction::Label(then_label));
                instructions.extend(self.convert_block(then_block));
                instructions.push(Instruction::Label(end_label));
                
                instructions
            }
            _ => Vec::new(), // Handle other cases
        }
    }

    fn convert_expression(&mut self, expr: &ast::Expression) -> Vec<Instruction> {
        match expr {
            ast::Expression::NumberLiteral(n) => {
                vec![Instruction::Push(Value::U256(n.parse().unwrap_or(0)))]
            }
            ast::Expression::Identifier(name) => {
                // Check if it's a local variable
                if let Some(ref function) = self.current_function {
                    if let Some(local) = function.locals.iter().find(|l| l.name == *name) {
                        return vec![Instruction::Load(local.index)];
                    }
                }
                
                // Check if it's a storage variable
                if let Some(ref contract) = self.current_contract {
                    if let Some(slot) = contract.storage.iter().find(|s| s.name == *name) {
                        return vec![Instruction::SLoad(slot.slot)];
                    }
                }
                
                Vec::new()
            }
            ast::Expression::Binary { left, operator, right } => {
                let mut instructions = self.convert_expression(left);
                instructions.extend(self.convert_expression(right));
                
                instructions.push(match operator {
                    ast::BinaryOp::Add => Instruction::Add,
                    ast::BinaryOp::Sub => Instruction::Sub,
                    ast::BinaryOp::Mul => Instruction::Mul,
                    ast::BinaryOp::Div => Instruction::Div,
                    ast::BinaryOp::Eq => Instruction::Eq,
                    ast::BinaryOp::Lt => Instruction::Lt,
                    ast::BinaryOp::Gt => Instruction::Gt,
                    ast::BinaryOp::LtEq => Instruction::LtEq,
                    ast::BinaryOp::GtEq => Instruction::GtEq,
                    _ => return Vec::new(), // Handle other cases
                });
                
                instructions
            }
            _ => Vec::new(), // Handle other cases
        }
    }

    fn convert_assignment_target(&self, target: &ast::Expression) -> Vec<Instruction> {
        match target {
            ast::Expression::Identifier(name) => {
                // Check if it's a local variable
                if let Some(ref function) = self.current_function {
                    if let Some(local) = function.locals.iter().find(|l| l.name == *name) {
                        return vec![Instruction::Store(local.index)];
                    }
                }
                
                // Check if it's a storage variable
                if let Some(ref contract) = self.current_contract {
                    if let Some(slot) = contract.storage.iter().find(|s| s.name == *name) {
                        return vec![Instruction::SStore(slot.slot)];
                    }
                }
                
                Vec::new()
            }
            _ => Vec::new(), // Handle other cases
        }
    }

    fn new_label(&mut self, prefix: &str) -> Label {
        let label = Label(format!("{}{}", prefix, self.label_counter));
        self.label_counter += 1;
        label
    }
}

// Optimization passes
pub fn optimize(program: Program, level: u8) -> Program {
    let mut optimized = program;
    
    if level >= 1 {
        optimized = constant_folding(optimized);
    }
    
    if level >= 2 {
        optimized = dead_code_elimination(optimized);
    }
    
    optimized
}

fn constant_folding(program: Program) -> Program {
    // Implement constant folding optimization
    program
}

fn dead_code_elimination(program: Program) -> Program {
    // Implement dead code elimination
    program
}

pub fn lower(ast: ast::Program) -> Result<Program, String> {
    let builder = IRBuilder::new();
    Ok(builder.build(&ast))
} 