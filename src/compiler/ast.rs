use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Program {
    pub contracts: Vec<Contract>,
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub state_vars: Vec<StateVar>,
    pub events: Vec<Event>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct StateVar {
    pub name: String,
    pub type_info: Type,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub modifiers: Vec<Modifier>,
    pub is_pure: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_info: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Address,
    U256,
    Bool,
    String,
    Map { key_type: Box<Type>, value_type: Box<Type> },
    Array(Box<Type>),
    Result { ok_type: Box<Type>, err_type: Box<Type> },
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        type_info: Option<Type>,
        value: Expression,
    },
    Assignment {
        target: Expression,
        value: Expression,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    If {
        condition: Expression,
        then_block: Block,
        else_block: Option<Block>,
    },
    While {
        condition: Expression,
        block: Block,
    },
    Return(Option<Expression>),
    Emit {
        event: String,
        arguments: Vec<Expression>,
    },
    Ensure {
        condition: Expression,
        message: String,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    BoolLiteral(bool),
    AddressLiteral(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        member: String,
    },
    IndexAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Modifier {
    NoReentry,
    Payable,
    View,
    Custom(String),
}

// Helper functions for AST construction
impl Program {
    pub fn new() -> Self {
        Program { contracts: Vec::new() }
    }

    pub fn add_contract(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }
}

impl Contract {
    pub fn new(name: String) -> Self {
        Contract {
            name,
            state_vars: Vec::new(),
            events: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl Block {
    pub fn new() -> Self {
        Block { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
} 