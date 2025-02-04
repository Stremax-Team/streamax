use crate::core::{Error, Result};

/// Gas meter for tracking gas usage
pub struct GasMeter {
    remaining: u64,
    limit: u64,
    used: u64,
}

impl GasMeter {
    /// Create a new gas meter with the given limit
    pub fn new(limit: u64) -> Self {
        GasMeter {
            remaining: limit,
            limit,
            used: 0,
        }
    }
    
    /// Consume gas
    pub fn consume(&mut self, amount: u64) -> Result<()> {
        if self.remaining >= amount {
            self.remaining = self.remaining.saturating_sub(amount);
            self.used = self.used.saturating_add(amount);
            Ok(())
        } else {
            Err(Error::OutOfGas)
        }
    }
    
    /// Get remaining gas
    pub fn remaining(&self) -> u64 {
        self.remaining
    }
    
    /// Get gas limit
    pub fn limit(&self) -> u64 {
        self.limit
    }
    
    /// Get used gas
    pub fn used(&self) -> u64 {
        self.used
    }
    
    /// Check if out of gas
    pub fn is_out_of_gas(&self) -> bool {
        self.remaining == 0
    }
}

/// Gas costs for different operations
pub mod costs {
    pub const ZERO: u64 = 0;
    pub const BASE: u64 = 2;
    pub const VERY_LOW: u64 = 3;
    pub const LOW: u64 = 5;
    pub const MID: u64 = 8;
    pub const HIGH: u64 = 10;
    pub const EXT: u64 = 20;
    pub const SPECIAL: u64 = 40;
    
    // Memory operations
    pub const MEMORY_STORE: u64 = LOW;
    pub const MEMORY_LOAD: u64 = LOW;
    pub const MEMORY_COPY: u64 = MID;
    
    // Storage operations
    pub const STORAGE_STORE: u64 = HIGH;
    pub const STORAGE_LOAD: u64 = HIGH;
    pub const STORAGE_DELETE: u64 = HIGH;
    
    // Contract operations
    pub const CONTRACT_CREATE: u64 = SPECIAL;
    pub const CONTRACT_CALL: u64 = EXT;
    pub const CONTRACT_RETURN: u64 = ZERO;
    
    // Crypto operations
    pub const HASH: u64 = MID;
    pub const VERIFY: u64 = SPECIAL;
    
    // Other operations
    pub const LOG: u64 = LOW;
    pub const EVENT: u64 = MID;
}

/// Gas estimator for estimating gas costs
pub struct GasEstimator {
    base_cost: u64,
    per_byte_cost: u64,
}

impl GasEstimator {
    pub fn new(base_cost: u64, per_byte_cost: u64) -> Self {
        GasEstimator {
            base_cost,
            per_byte_cost,
        }
    }
    
    pub fn estimate_cost(&self, size: usize) -> u64 {
        self.base_cost.saturating_add(
            self.per_byte_cost.saturating_mul(size as u64)
        )
    }
    
    pub fn estimate_storage_cost(&self, key_size: usize, value_size: usize) -> u64 {
        costs::STORAGE_STORE.saturating_add(
            self.estimate_cost(key_size).saturating_add(
                self.estimate_cost(value_size)
            )
        )
    }
}