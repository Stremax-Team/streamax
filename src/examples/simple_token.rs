use crate::stdlib::blockchain::{Transaction, Address};
use crate::stdlib::crypto::{PublicKey, PrivateKey, Signature};
use crate::core::{Result, Error};
use crate::stdlib::collections::Map;

/// A simple token implementation
pub struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
    balances: Map<Address, u64>,
}

impl Token {
    pub fn new(name: String, symbol: String, decimals: u8, initial_supply: u64) -> Self {
        let mut token = Token {
            name,
            symbol,
            decimals,
            total_supply: initial_supply,
            balances: Map::new(b"token_balances"),
        };
        
        // Mint initial supply to contract creator
        // In a real implementation, this would use the caller's address
        let creator = Address([0; 32]);
        token.balances.set(&creator, &initial_supply).unwrap();
        
        token
    }
    
    pub fn transfer(&mut self, from: &Address, to: &Address, amount: u64) -> Result<()> {
        // Check balance
        let from_balance = self.balances.get(from)?.unwrap_or(0);
        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }
        
        // Update balances
        self.balances.set(from, &(from_balance - amount))?;
        let to_balance = self.balances.get(to)?.unwrap_or(0);
        self.balances.set(to, &(to_balance + amount))?;
        
        Ok(())
    }
    
    pub fn balance_of(&self, address: &Address) -> Result<u64> {
        Ok(self.balances.get(address)?.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token() {
        let mut token = Token::new(
            "Example Token".to_string(),
            "EXT".to_string(),
            18,
            1_000_000,
        );
        
        let address1 = Address([1; 32]);
        let address2 = Address([2; 32]);
        
        // Test initial supply
        assert_eq!(token.balance_of(&Address([0; 32])).unwrap(), 1_000_000);
        
        // Test transfer
        token.transfer(&Address([0; 32]), &address1, 1000).unwrap();
        assert_eq!(token.balance_of(&address1).unwrap(), 1000);
        
        // Test insufficient balance
        assert!(token.transfer(&address2, &address1, 100).is_err());
    }
} 