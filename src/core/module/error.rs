use std::fmt;
use std::error::Error as StdError;
use crate::core::{ModuleId, Symbol};

#[derive(Debug, Clone)]
pub enum ModuleError {
    /// Module not found in search paths
    NotFound(ModuleId),
    
    /// Error parsing module contents
    ParseError {
        module: ModuleId,
        message: String,
    },
    
    /// Circular dependency detected
    CircularDependency {
        path: Vec<ModuleId>,
    },
    
    /// Invalid or undefined symbol
    InvalidSymbol {
        module: ModuleId,
        symbol: Symbol,
        reason: String,
    },
    
    /// Visibility violation
    VisibilityViolation {
        module: ModuleId,
        symbol: Symbol,
        required: String,
        actual: String,
    },
    
    /// Type error
    TypeError {
        module: ModuleId,
        message: String,
    },
    
    /// IO error during module operations
    IoError {
        module: Option<ModuleId>,
        source: std::io::Error,
    },
    
    /// Version mismatch between modules
    VersionMismatch {
        module: ModuleId,
        required: String,
        actual: String,
    },
    
    /// Cache error
    CacheError {
        module: Option<ModuleId>,
        message: String,
    },
    
    /// Validation error
    ValidationError {
        module: ModuleId,
        message: String,
    },
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleError::NotFound(id) => {
                write!(f, "Module not found: {}", id.0)
            }
            ModuleError::ParseError { module, message } => {
                write!(f, "Parse error in module {}: {}", module.0, message)
            }
            ModuleError::CircularDependency { path } => {
                write!(f, "Circular dependency detected: {}", 
                    path.iter().map(|id| id.0.as_str()).collect::<Vec<_>>().join(" -> "))
            }
            ModuleError::InvalidSymbol { module, symbol, reason } => {
                write!(f, "Invalid symbol '{}' in module {}: {}", symbol.0, module.0, reason)
            }
            ModuleError::VisibilityViolation { module, symbol, required, actual } => {
                write!(f, "Visibility violation for symbol '{}' in module {}: required {} but was {}", 
                    symbol.0, module.0, required, actual)
            }
            ModuleError::TypeError { module, message } => {
                write!(f, "Type error in module {}: {}", module.0, message)
            }
            ModuleError::IoError { module, source } => {
                if let Some(module) = module {
                    write!(f, "IO error in module {}: {}", module.0, source)
                } else {
                    write!(f, "IO error: {}", source)
                }
            }
            ModuleError::VersionMismatch { module, required, actual } => {
                write!(f, "Version mismatch in module {}: required {} but found {}", 
                    module.0, required, actual)
            }
            ModuleError::CacheError { module, message } => {
                if let Some(module) = module {
                    write!(f, "Cache error in module {}: {}", module.0, message)
                } else {
                    write!(f, "Cache error: {}", message)
                }
            }
            ModuleError::ValidationError { module, message } => {
                write!(f, "Validation error in module {}: {}", module.0, message)
            }
        }
    }
}

impl StdError for ModuleError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ModuleError::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, ModuleError>;

// Helper macros for error handling
#[macro_export]
macro_rules! module_error {
    ($kind:ident, $($arg:expr),*) => {
        Err(ModuleError::$kind($($arg),*))
    };
}

#[macro_export]
macro_rules! ensure_module {
    ($cond:expr, $kind:ident, $($arg:expr),*) => {
        if !$cond {
            return module_error!($kind, $($arg),*);
        }
    };
}