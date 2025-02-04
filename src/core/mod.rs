pub mod types;
pub mod memory;
pub mod error;
pub mod module;

use std::fmt;

// Core traits
pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self> where Self: Sized;
}

// Re-exports
pub use self::error::{Error, Result};
pub use self::memory::Region;
pub use self::module::Module;

// Basic types that are fundamental to the language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub module: ModuleId,
    pub name: Symbol,
}

impl Path {
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Path {
            module: ModuleId(module.into()),
            name: Symbol(name.into()),
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.module.0, self.name.0)
    }
}

// Core constants
pub const MAX_MEMORY_PAGES: usize = 16384; // 1GB with 64KB pages
pub const MAX_STACK_HEIGHT: usize = 1024;
pub const MAX_CALL_DEPTH: usize = 1024;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MIN_SUPPORTED_VERSION: &str = "0.1.0"; 