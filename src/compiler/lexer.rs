use logos::Logos;
use std::error::Error;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Keywords
    #[token("contract")]
    Contract,
    
    #[token("state")]
    State,
    
    #[token("event")]
    Event,
    
    #[token("pure")]
    Pure,
    
    #[token("mut")]
    Mut,
    
    #[token("fn")]
    Fn,
    
    #[token("let")]
    Let,
    
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    #[token("while")]
    While,
    
    #[token("return")]
    Return,
    
    #[token("ensure")]
    Ensure,
    
    #[token("emit")]
    Emit,
    
    // Blockchain-specific keywords
    #[token("@no_reentry")]
    NoReentry,
    
    // Types
    #[token("Address")]
    Address,
    
    #[token("u256")]
    U256,
    
    #[token("Map")]
    Map,
    
    #[token("Result")]
    Result,
    
    // Symbols
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token("<")]
    LAngle,
    
    #[token(">")]
    RAngle,
    
    #[token("[")]
    LBracket,
    
    #[token("]")]
    RBracket,
    
    #[token(":")]
    Colon,
    
    #[token(";")]
    Semicolon,
    
    #[token(",")]
    Comma,
    
    #[token(".")]
    Dot,
    
    #[token("=")]
    Assign,
    
    #[token("->")]
    Arrow,
    
    // Operators
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("+=")]
    PlusAssign,
    
    #[token("-=")]
    MinusAssign,
    
    #[token(">=")]
    GreaterEqual,
    
    #[token("<=")]
    LessEqual,
    
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    #[regex("[0-9]+")]
    Number,
    
    #[regex(r#""[^"]*""#)]
    String,
    
    // Comments and whitespace
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Comment,
    
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    
    // Error
    #[error]
    Error,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let lexer = Token::lexer(input);
    let tokens: Vec<Token> = lexer.collect();
    
    // Check for lexical errors
    if tokens.iter().any(|t| matches!(t, Token::Error)) {
        return Err("Lexical error: Invalid token found".into());
    }
    
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "contract TokenContract { fn transfer() -> Result }";
        let tokens = tokenize(input).unwrap();
        assert!(tokens.contains(&Token::Contract));
        assert!(tokens.contains(&Token::Fn));
        assert!(tokens.contains(&Token::Arrow));
        assert!(tokens.contains(&Token::Result));
    }

    #[test]
    fn test_blockchain_specific() {
        let input = "@no_reentry { Address }";
        let tokens = tokenize(input).unwrap();
        assert!(tokens.contains(&Token::NoReentry));
        assert!(tokens.contains(&Token::Address));
    }
} 