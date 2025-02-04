use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::core::{Error, Result};

/// Timestamp representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Create a new timestamp from Unix timestamp
    pub fn from_unix_timestamp(timestamp: u64) -> Self {
        Timestamp(timestamp)
    }
    
    /// Get current timestamp
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        Timestamp(now.as_secs())
    }
    
    /// Get timestamp value in seconds
    pub fn as_secs(&self) -> u64 {
        self.0
    }
    
    /// Get elapsed time since this timestamp
    pub fn elapsed(&self) -> Duration {
        let now = Self::now();
        Duration::from_secs(now.0.saturating_sub(self.0))
    }
    
    /// Add duration to timestamp
    pub fn add_duration(&self, duration: Duration) -> Self {
        Timestamp(self.0.saturating_add(duration.as_secs()))
    }
    
    /// Subtract duration from timestamp
    pub fn sub_duration(&self, duration: Duration) -> Self {
        Timestamp(self.0.saturating_sub(duration.as_secs()))
    }
}

/// Block time utilities
#[derive(Debug, Clone, Copy)]
pub struct BlockTime {
    pub number: u64,
    pub timestamp: Timestamp,
}

impl BlockTime {
    pub fn new(number: u64, timestamp: Timestamp) -> Self {
        BlockTime { number, timestamp }
    }
    
    pub fn elapsed_since(&self, previous: &BlockTime) -> Duration {
        Duration::from_secs(self.timestamp.0.saturating_sub(previous.timestamp.0))
    }
    
    pub fn blocks_since(&self, previous: &BlockTime) -> u64 {
        self.number.saturating_sub(previous.number)
    }
}

/// Time constants
pub const SECONDS_PER_BLOCK: u64 = 15;
pub const BLOCKS_PER_DAY: u64 = 24 * 60 * 60 / SECONDS_PER_BLOCK;
pub const BLOCKS_PER_YEAR: u64 = 365 * BLOCKS_PER_DAY;

/// Time conversion utilities
pub mod convert {
    use super::*;
    
    pub fn blocks_to_duration(blocks: u64) -> Duration {
        Duration::from_secs(blocks * SECONDS_PER_BLOCK)
    }
    
    pub fn duration_to_blocks(duration: Duration) -> u64 {
        duration.as_secs() / SECONDS_PER_BLOCK
    }
    
    pub fn timestamp_to_block_number(timestamp: Timestamp, genesis_time: Timestamp) -> u64 {
        timestamp.0.saturating_sub(genesis_time.0) / SECONDS_PER_BLOCK
    }
    
    pub fn block_number_to_timestamp(block: u64, genesis_time: Timestamp) -> Timestamp {
        Timestamp(genesis_time.0.saturating_add(block * SECONDS_PER_BLOCK))
    }
}