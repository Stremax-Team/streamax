pub mod math;
pub mod convert;
pub mod time;
pub mod gas;
pub mod events;
pub mod crypto;

use crate::core::{Error, Result};

// Re-exports
pub use self::math::*;
pub use self::convert::*;
pub use self::time::*;
pub use self::gas::*;
pub use self::events::*;
pub use self::crypto::*;

// Core traits
pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self> where Self: Sized;
}

// Standard library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Feature flags
pub const FEATURE_CRYPTO: bool = cfg!(feature = "crypto");
pub const FEATURE_PARALLEL: bool = cfg!(feature = "parallel");
pub const FEATURE_ASYNC: bool = cfg!(feature = "async");

// Core types and traits
pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, String> where Self: Sized;
}

// Basic numeric operations with overflow checking
pub mod math {
    pub fn checked_add(a: u256, b: u256) -> Option<u256> {
        a.checked_add(b)
    }

    pub fn checked_sub(a: u256, b: u256) -> Option<u256> {
        a.checked_sub(b)
    }

    pub fn checked_mul(a: u256, b: u256) -> Option<u256> {
        a.checked_mul(b)
    }

    pub fn checked_div(a: u256, b: u256) -> Option<u256> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

// Memory management utilities
pub mod memory {
    pub struct Region {
        data: Vec<u8>,
        capacity: usize,
    }

    impl Region {
        pub fn new(capacity: usize) -> Self {
            Region {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        pub fn allocate(&mut self, size: usize) -> Option<&mut [u8]> {
            if self.data.len() + size <= self.capacity {
                let start = self.data.len();
                self.data.resize(start + size, 0);
                Some(&mut self.data[start..start + size])
            } else {
                None
            }
        }

        pub fn deallocate(&mut self, ptr: &[u8]) {
            // Implementation of memory deallocation
        }
    }
}

// Error handling
#[derive(Debug)]
pub enum Error {
    OutOfGas,
    OutOfMemory,
    InvalidOperation,
    ContractError(String),
    SerializationError,
    CryptoError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::OutOfGas => write!(f, "Out of gas"),
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::InvalidOperation => write!(f, "Invalid operation"),
            Error::ContractError(msg) => write!(f, "Contract error: {}", msg),
            Error::SerializationError => write!(f, "Serialization error"),
            Error::CryptoError => write!(f, "Cryptographic operation failed"),
        }
    }
}

// Result type alias for Stremax operations
pub type Result<T> = std::result::Result<T, Error>;

// Time and block utilities
pub mod time {
    pub struct Timestamp(u64);

    impl Timestamp {
        pub fn now() -> Self {
            // Implementation to get current block timestamp
            Timestamp(0)
        }

        pub fn elapsed(&self) -> u64 {
            let now = Self::now();
            now.0.saturating_sub(self.0)
        }
    }
}

// Testing utilities
#[cfg(test)]
pub mod test {
    use super::*;

    pub struct TestContext {
        pub gas_meter: gas::GasMeter,
        pub memory: memory::Region,
        pub events: Vec<events::Event>,
    }

    impl TestContext {
        pub fn new() -> Self {
            TestContext {
                gas_meter: gas::GasMeter::new(1_000_000),
                memory: memory::Region::new(1024 * 1024),
                events: Vec::new(),
            }
        }

        pub fn assert_event_emitted(&self, name: &str) -> bool {
            self.events.iter().any(|e| e.name == name)
        }
    }
} 