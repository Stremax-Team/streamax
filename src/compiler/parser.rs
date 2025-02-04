use crate::ast::*;
use crate::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEOF,
    InvalidExpression,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();
        
        while self.tokens.peek().is_some() {
            if let Some(contract) = self.parse_contract()? {
                program.add_contract(contract);
            }
        }
        
        Ok(program)
    }

    fn parse_contract(&mut self) -> Result<Option<Contract>, ParseError> {
        if !matches!(self.tokens.peek(), Some(Token::Contract)) {
            return Ok(None);
        }
        
        self.consume(Token::Contract)?;
        let name = self.parse_identifier()?;
        self.consume(Token::LBrace)?;
        
        let mut contract = Contract::new(name);
        
        while !matches!(self.tokens.peek(), Some(Token::RBrace)) {
            match self.tokens.peek() {
                Some(Token::State) => {
                    contract.state_vars.push(self.parse_state_var()?);
                }
                Some(Token::Event) => {
                    contract.events.push(self.parse_event()?);
                }
                Some(Token::Pure) | Some(Token::Mut) | Some(Token::Fn) => {
                    contract.functions.push(self.parse_function()?);
                }
                _ => return Err(ParseError::UnexpectedToken("Expected state, event, or function declaration".into())),
            }
        }
        
        self.consume(Token::RBrace)?;
        Ok(Some(contract))
    }

    fn parse_state_var(&mut self) -> Result<StateVar, ParseError> {
        self.consume(Token::State)?;
        let name = self.parse_identifier()?;
        self.consume(Token::Colon)?;
        let type_info = self.parse_type()?;
        self.consume(Token::Semicolon)?;
        
        Ok(StateVar {
            name,
            type_info,
            visibility: Visibility::Public, // Default visibility
        })
    }

    fn parse_event(&mut self) -> Result<Event, ParseError> {
        self.consume(Token::Event)?;
        let name = self.parse_identifier()?;
        self.consume(Token::LParen)?;
        
        let mut parameters = Vec::new();
        while !matches!(self.tokens.peek(), Some(Token::RParen)) {
            if !parameters.is_empty() {
                self.consume(Token::Comma)?;
            }
            parameters.push(self.parse_parameter()?);
        }
        
        self.consume(Token::RParen)?;
        self.consume(Token::Semicolon)?;
        
        Ok(Event { name, parameters })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        let is_pure = matches!(self.tokens.peek(), Some(Token::Pure));
        if is_pure {
            self.consume(Token::Pure)?;
        }
        
        self.consume(Token::Fn)?;
        let name = self.parse_identifier()?;
        
        self.consume(Token::LParen)?;
        let mut parameters = Vec::new();
        while !matches!(self.tokens.peek(), Some(Token::RParen)) {
            if !parameters.is_empty() {
                self.consume(Token::Comma)?;
            }
            parameters.push(self.parse_parameter()?);
        }
        self.consume(Token::RParen)?;
        
        let return_type = if matches!(self.tokens.peek(), Some(Token::Arrow)) {
            self.consume(Token::Arrow)?;
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        Ok(Function {
            name,
            parameters,
            return_type,
            body,
            modifiers: Vec::new(),
            is_pure,
        })
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.consume(Token::LBrace)?;
        let mut block = Block::new();
        
        while !matches!(self.tokens.peek(), Some(Token::RBrace)) {
            block.add_statement(self.parse_statement()?);
        }
        
        self.consume(Token::RBrace)?;
        Ok(block)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.tokens.peek() {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::Emit) => self.parse_emit_statement(),
            Some(Token::Ensure) => self.parse_ensure_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.tokens.next() {
            Some(Token::Address) => Ok(Type::Address),
            Some(Token::U256) => Ok(Type::U256),
            Some(Token::Map) => {
                self.consume(Token::LAngle)?;
                let key_type = Box::new(self.parse_type()?);
                self.consume(Token::Comma)?;
                let value_type = Box::new(self.parse_type()?);
                self.consume(Token::RAngle)?;
                Ok(Type::Map { key_type, value_type })
            }
            Some(Token::Result) => {
                self.consume(Token::LAngle)?;
                let ok_type = Box::new(self.parse_type()?);
                self.consume(Token::Comma)?;
                let err_type = Box::new(self.parse_type()?);
                self.consume(Token::RAngle)?;
                Ok(Type::Result { ok_type, err_type })
            }
            Some(Token::Identifier) => Ok(Type::Custom(self.parse_identifier()?)),
            _ => Err(ParseError::UnexpectedToken("Expected type".into())),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        match self.tokens.next() {
            Some(Token::Identifier) => Ok("identifier".to_string()), // In a real implementation, we'd get the actual identifier text
            _ => Err(ParseError::UnexpectedToken("Expected identifier".into())),
        }
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParseError> {
        match self.tokens.next() {
            Some(token) if token == expected => Ok(()),
            Some(_) => Err(ParseError::UnexpectedToken(format!("Expected {:?}", expected))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse_simple_contract() {
        let input = r#"
            contract TokenContract {
                state balance: Map<Address, u256>;
                
                event Transfer(from: Address, to: Address, amount: u256);
                
                pure fn get_balance(owner: Address) -> u256 {
                    return balance[owner];
                }
            }
        "#;
        
        let tokens = tokenize(input).unwrap();
        let program = parse(tokens).unwrap();
        
        assert_eq!(program.contracts.len(), 1);
        let contract = &program.contracts[0];
        assert_eq!(contract.name, "TokenContract");
        assert_eq!(contract.state_vars.len(), 1);
        assert_eq!(contract.events.len(), 1);
        assert_eq!(contract.functions.len(), 1);
    }
} 