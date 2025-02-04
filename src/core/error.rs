use std::fmt;
use std::error::Error as StdError;

/// Core error type for the language
#[derive(Debug)]
pub enum Error {
    // System errors
    OutOfMemory,
    OutOfGas,
    StackOverflow,
    InvalidOperation,
    
    // Module errors
    ModuleNotFound(String),
    ModuleLoadError(String),
    SymbolNotFound(String),
    
    // Type errors
    TypeError(String),
    UnificationError(String),
    
    // Runtime errors
    RuntimeError(String),
    AssertionFailed(String),
    
    // IO errors
    IoError(std::io::Error),
    
    // Contract errors
    ContractError(String),
    
    // Serialization errors
    SerializationError(String),
    DeserializationError(String),
    
    // Crypto errors
    CryptoError(String),
    
    // Custom errors
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::OutOfGas => write!(f, "Out of gas"),
            Error::StackOverflow => write!(f, "Stack overflow"),
            Error::InvalidOperation => write!(f, "Invalid operation"),
            Error::ModuleNotFound(name) => write!(f, "Module not found: {}", name),
            Error::ModuleLoadError(msg) => write!(f, "Failed to load module: {}", msg),
            Error::SymbolNotFound(name) => write!(f, "Symbol not found: {}", name),
            Error::TypeError(msg) => write!(f, "Type error: {}", msg),
            Error::UnificationError(msg) => write!(f, "Type unification error: {}", msg),
            Error::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            Error::AssertionFailed(msg) => write!(f, "Assertion failed: {}", msg),
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::ContractError(msg) => write!(f, "Contract error: {}", msg),
            Error::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Error::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            Error::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

/// Result type alias for operations that can fail
pub type Result<T> = std::result::Result<T, Error>;

// Helper macros for error handling
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err(crate::core::Error::Custom($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(crate::core::Error::Custom(format!($fmt, $($arg)*)))
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $msg:literal $(,)?) => {
        if !($cond) {
            bail!($msg);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
} 