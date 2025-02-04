use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug)]
pub enum TypeError {
    TypeMismatch {
        expected: Type,
        found: Type,
    },
    UndefinedVariable(String),
    UndefinedFunction(String),
    UndefinedType(String),
    InvalidOperation {
        op: String,
        type_name: String,
    },
    ReentrancyVulnerability(String),
    StateModificationInPureFunction,
    InvalidEventEmission(String),
}

pub struct TypeChecker {
    variables: HashMap<String, Type>,
    functions: HashMap<String, FunctionSignature>,
    events: HashMap<String, Vec<Parameter>>,
    current_function: Option<String>,
    is_pure_context: bool,
}

#[derive(Clone)]
struct FunctionSignature {
    parameters: Vec<Parameter>,
    return_type: Option<Type>,
    is_pure: bool,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            variables: HashMap::new(),
            functions: HashMap::new(),
            events: HashMap::new(),
            current_function: None,
            is_pure_context: false,
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), TypeError> {
        // First pass: collect all declarations
        for contract in &program.contracts {
            self.collect_declarations(contract)?;
        }

        // Second pass: check implementations
        for contract in &program.contracts {
            self.check_contract(contract)?;
        }

        Ok(())
    }

    fn collect_declarations(&mut self, contract: &Contract) -> Result<(), TypeError> {
        // Collect state variables
        for var in &contract.state_vars {
            self.variables.insert(var.name.clone(), var.type_info.clone());
        }

        // Collect events
        for event in &contract.events {
            self.events.insert(event.name.clone(), event.parameters.clone());
        }

        // Collect function signatures
        for function in &contract.functions {
            self.functions.insert(
                function.name.clone(),
                FunctionSignature {
                    parameters: function.parameters.clone(),
                    return_type: function.return_type.clone(),
                    is_pure: function.is_pure,
                },
            );
        }

        Ok(())
    }

    fn check_contract(&mut self, contract: &Contract) -> Result<(), TypeError> {
        // Check state variables
        for var in &contract.state_vars {
            self.check_type(&var.type_info)?;
        }

        // Check events
        for event in &contract.events {
            for param in &event.parameters {
                self.check_type(&param.type_info)?;
            }
        }

        // Check functions
        for function in &contract.functions {
            self.check_function(function)?;
        }

        Ok(())
    }

    fn check_function(&mut self, function: &Function) -> Result<(), TypeError> {
        self.current_function = Some(function.name.clone());
        self.is_pure_context = function.is_pure;

        // Create new scope for function parameters
        let outer_vars = self.variables.clone();
        
        // Add parameters to scope
        for param in &function.parameters {
            self.variables.insert(param.name.clone(), param.type_info.clone());
        }

        // Check return type if present
        if let Some(ref return_type) = function.return_type {
            self.check_type(return_type)?;
        }

        // Check function body
        self.check_block(&function.body)?;

        // Restore outer scope
        self.variables = outer_vars;
        self.current_function = None;
        self.is_pure_context = false;

        Ok(())
    }

    fn check_block(&mut self, block: &Block) -> Result<(), TypeError> {
        for statement in &block.statements {
            self.check_statement(statement)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), TypeError> {
        match statement {
            Statement::Let { name, type_info, value } => {
                let value_type = self.check_expression(value)?;
                if let Some(ref declared_type) = type_info {
                    if !self.types_match(declared_type, &value_type) {
                        return Err(TypeError::TypeMismatch {
                            expected: declared_type.clone(),
                            found: value_type,
                        });
                    }
                }
                self.variables.insert(name.clone(), value_type);
            }
            Statement::Assignment { target, value } => {
                let target_type = self.check_expression(target)?;
                let value_type = self.check_expression(value)?;
                if !self.types_match(&target_type, &value_type) {
                    return Err(TypeError::TypeMismatch {
                        expected: target_type,
                        found: value_type,
                    });
                }
            }
            Statement::If { condition, then_block, else_block } => {
                let condition_type = self.check_expression(condition)?;
                if !matches!(condition_type, Type::Bool) {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Bool,
                        found: condition_type,
                    });
                }
                self.check_block(then_block)?;
                if let Some(else_block) = else_block {
                    self.check_block(else_block)?;
                }
            }
            Statement::While { condition, block } => {
                let condition_type = self.check_expression(condition)?;
                if !matches!(condition_type, Type::Bool) {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Bool,
                        found: condition_type,
                    });
                }
                self.check_block(block)?;
            }
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    let expr_type = self.check_expression(expr)?;
                    if let Some(current_fn) = &self.current_function {
                        if let Some(fn_sig) = self.functions.get(current_fn) {
                            if let Some(ref return_type) = fn_sig.return_type {
                                if !self.types_match(return_type, &expr_type) {
                                    return Err(TypeError::TypeMismatch {
                                        expected: return_type.clone(),
                                        found: expr_type,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            Statement::Emit { event, arguments } => {
                if self.is_pure_context {
                    return Err(TypeError::StateModificationInPureFunction);
                }
                if let Some(event_params) = self.events.get(event) {
                    if arguments.len() != event_params.len() {
                        return Err(TypeError::InvalidEventEmission(
                            format!("Wrong number of arguments for event {}", event)
                        ));
                    }
                    for (arg, param) in arguments.iter().zip(event_params) {
                        let arg_type = self.check_expression(arg)?;
                        if !self.types_match(&param.type_info, &arg_type) {
                            return Err(TypeError::TypeMismatch {
                                expected: param.type_info.clone(),
                                found: arg_type,
                            });
                        }
                    }
                } else {
                    return Err(TypeError::InvalidEventEmission(
                        format!("Undefined event {}", event)
                    ));
                }
            }
            Statement::Ensure { condition, message: _ } => {
                let condition_type = self.check_expression(condition)?;
                if !matches!(condition_type, Type::Bool) {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Bool,
                        found: condition_type,
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn check_expression(&self, expr: &Expression) -> Result<Type, TypeError> {
        match expr {
            Expression::Identifier(name) => {
                self.variables.get(name)
                    .cloned()
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))
            }
            Expression::NumberLiteral(_) => Ok(Type::U256),
            Expression::StringLiteral(_) => Ok(Type::String),
            Expression::BoolLiteral(_) => Ok(Type::Bool),
            Expression::AddressLiteral(_) => Ok(Type::Address),
            Expression::Binary { left, operator, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                self.check_binary_operation(operator, &left_type, &right_type)
            }
            Expression::Unary { operator, operand } => {
                let operand_type = self.check_expression(operand)?;
                self.check_unary_operation(operator, &operand_type)
            }
            Expression::FunctionCall { function, arguments } => {
                self.check_function_call(function, arguments)
            }
            Expression::MemberAccess { object, member } => {
                self.check_member_access(object, member)
            }
            Expression::IndexAccess { array, index } => {
                self.check_index_access(array, index)
            }
        }
    }

    fn check_type(&self, type_info: &Type) -> Result<(), TypeError> {
        match type_info {
            Type::Map { key_type, value_type } => {
                self.check_type(key_type)?;
                self.check_type(value_type)
            }
            Type::Array(element_type) => {
                self.check_type(element_type)
            }
            Type::Result { ok_type, err_type } => {
                self.check_type(ok_type)?;
                self.check_type(err_type)
            }
            Type::Custom(name) => {
                if !self.is_known_type(name) {
                    return Err(TypeError::UndefinedType(name.clone()));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn types_match(&self, expected: &Type, found: &Type) -> bool {
        match (expected, found) {
            (Type::Map { key_type: k1, value_type: v1 },
             Type::Map { key_type: k2, value_type: v2 }) => {
                self.types_match(k1, k2) && self.types_match(v1, v2)
            }
            (Type::Array(t1), Type::Array(t2)) => {
                self.types_match(t1, t2)
            }
            (Type::Result { ok_type: ok1, err_type: err1 },
             Type::Result { ok_type: ok2, err_type: err2 }) => {
                self.types_match(ok1, ok2) && self.types_match(err1, err2)
            }
            _ => expected == found,
        }
    }

    fn is_known_type(&self, name: &str) -> bool {
        // Add custom type checking logic here
        matches!(name, "Address" | "u256" | "bool" | "string")
    }

    fn check_binary_operation(
        &self,
        op: &BinaryOp,
        left_type: &Type,
        right_type: &Type,
    ) -> Result<Type, TypeError> {
        match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                if !matches!(left_type, Type::U256) || !matches!(right_type, Type::U256) {
                    return Err(TypeError::InvalidOperation {
                        op: format!("{:?}", op),
                        type_name: format!("{:?}", left_type),
                    });
                }
                Ok(Type::U256)
            }
            BinaryOp::Eq | BinaryOp::NotEq => {
                if !self.types_match(left_type, right_type) {
                    return Err(TypeError::TypeMismatch {
                        expected: left_type.clone(),
                        found: right_type.clone(),
                    });
                }
                Ok(Type::Bool)
            }
            BinaryOp::Lt | BinaryOp::Gt | BinaryOp::LtEq | BinaryOp::GtEq => {
                if !matches!(left_type, Type::U256) || !matches!(right_type, Type::U256) {
                    return Err(TypeError::InvalidOperation {
                        op: format!("{:?}", op),
                        type_name: format!("{:?}", left_type),
                    });
                }
                Ok(Type::Bool)
            }
            BinaryOp::And | BinaryOp::Or => {
                if !matches!(left_type, Type::Bool) || !matches!(right_type, Type::Bool) {
                    return Err(TypeError::InvalidOperation {
                        op: format!("{:?}", op),
                        type_name: format!("{:?}", left_type),
                    });
                }
                Ok(Type::Bool)
            }
        }
    }

    fn check_unary_operation(
        &self,
        op: &UnaryOp,
        operand_type: &Type,
    ) -> Result<Type, TypeError> {
        match op {
            UnaryOp::Not => {
                if !matches!(operand_type, Type::Bool) {
                    return Err(TypeError::InvalidOperation {
                        op: "!".to_string(),
                        type_name: format!("{:?}", operand_type),
                    });
                }
                Ok(Type::Bool)
            }
            UnaryOp::Neg => {
                if !matches!(operand_type, Type::U256) {
                    return Err(TypeError::InvalidOperation {
                        op: "-".to_string(),
                        type_name: format!("{:?}", operand_type),
                    });
                }
                Ok(Type::U256)
            }
        }
    }

    fn check_function_call(
        &self,
        function: &Expression,
        arguments: &[Expression],
    ) -> Result<Type, TypeError> {
        if let Expression::Identifier(name) = function {
            if let Some(signature) = self.functions.get(name) {
                if arguments.len() != signature.parameters.len() {
                    return Err(TypeError::InvalidOperation {
                        op: format!("call to {}", name),
                        type_name: "wrong number of arguments".to_string(),
                    });
                }

                for (arg, param) in arguments.iter().zip(&signature.parameters) {
                    let arg_type = self.check_expression(arg)?;
                    if !self.types_match(&param.type_info, &arg_type) {
                        return Err(TypeError::TypeMismatch {
                            expected: param.type_info.clone(),
                            found: arg_type,
                        });
                    }
                }

                Ok(signature.return_type.clone().unwrap_or(Type::U256))
            } else {
                Err(TypeError::UndefinedFunction(name.clone()))
            }
        } else {
            Err(TypeError::InvalidOperation {
                op: "call".to_string(),
                type_name: "not a function".to_string(),
            })
        }
    }

    fn check_member_access(
        &self,
        object: &Expression,
        member: &str,
    ) -> Result<Type, TypeError> {
        let object_type = self.check_expression(object)?;
        match object_type {
            Type::Map { value_type, .. } => Ok(*value_type),
            _ => Err(TypeError::InvalidOperation {
                op: format!("member access {}", member),
                type_name: format!("{:?}", object_type),
            }),
        }
    }

    fn check_index_access(
        &self,
        array: &Expression,
        index: &Expression,
    ) -> Result<Type, TypeError> {
        let array_type = self.check_expression(array)?;
        let index_type = self.check_expression(index)?;

        match array_type {
            Type::Map { key_type, value_type } => {
                if !self.types_match(&key_type, &index_type) {
                    return Err(TypeError::TypeMismatch {
                        expected: *key_type,
                        found: index_type,
                    });
                }
                Ok(*value_type)
            }
            Type::Array(element_type) => {
                if !matches!(index_type, Type::U256) {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::U256,
                        found: index_type,
                    });
                }
                Ok(*element_type)
            }
            _ => Err(TypeError::InvalidOperation {
                op: "index access".to_string(),
                type_name: format!("{:?}", array_type),
            }),
        }
    }
}

pub fn check(program: Program) -> Result<Program, TypeError> {
    let mut checker = TypeChecker::new();
    checker.check(&program)?;
    Ok(program)
} 