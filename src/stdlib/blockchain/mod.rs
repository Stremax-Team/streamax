use crate::core::{Result, Error, Serialize};

/// Represents a block in the blockchain
pub struct Block {
    pub height: u64,
    pub timestamp: u64,
    pub prev_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub hash: [u8; 32],
}

/// Represents a transaction in the blockchain
pub struct Transaction {
    pub sender: Address,
    pub receiver: Address,
    pub amount: u64,
    pub nonce: u64,
    pub signature: [u8; 64],
}

/// Represents an address in the blockchain
pub struct Address(pub [u8; 32]);

/// Trait for blockchain storage implementations
pub trait BlockchainStorage {
    fn get_block(&self, height: u64) -> Result<Option<Block>>;
    fn get_latest_block(&self) -> Result<Option<Block>>;
    fn append_block(&mut self, block: Block) -> Result<()>;
    fn get_transaction(&self, tx_hash: &[u8; 32]) -> Result<Option<Transaction>>;
}

/// Trait for transaction validation
pub trait TransactionValidator {
    fn validate(&self, tx: &Transaction) -> Result<bool>;
    fn verify_signature(&self, tx: &Transaction) -> Result<bool>;
} 