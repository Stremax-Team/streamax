//! Example test suite for Token contract

use stremax::testing::*;
use stremax::prelude::*;

// Test contract
#[contract]
struct Token {
    total_supply: u64,
    balances: Map<Address, u64>,
}

impl Token {
    #[constructor]
    fn new(initial_supply: u64) -> Self {
        let mut token = Self {
            total_supply: initial_supply,
            balances: Map::new(),
        };
        token.balances.insert(msg::sender(), initial_supply);
        token
    }

    #[transaction]
    fn transfer(&mut self, to: Address, amount: u64) -> Result<()> {
        let sender = msg::sender();
        ensure!(self.balances[sender] >= amount, "Insufficient balance");
        
        self.balances[sender] -= amount;
        self.balances[to] += amount;
        
        emit!(Transfer { from: sender, to, amount });
        Ok(())
    }

    #[view]
    fn balance_of(&self, account: Address) -> u64 {
        self.balances[account]
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let initial_supply = 1000;
        let contract = Token::new(initial_supply);
        
        assert_eq!(contract.total_supply, initial_supply);
        assert_eq!(contract.balance_of(msg::sender()), initial_supply);
    }

    #[test]
    fn test_transfer() {
        let mut contract = Token::new(1000);
        let recipient = Address::from([1u8; 20]);
        
        contract.transfer(recipient, 100).unwrap();
        
        assert_eq!(contract.balance_of(msg::sender()), 900);
        assert_eq!(contract.balance_of(recipient), 100);
    }

    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_transfer_insufficient_balance() {
        let mut contract = Token::new(100);
        let recipient = Address::from([1u8; 20]);
        
        contract.transfer(recipient, 200).unwrap();
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use stremax::quickcheck;

    #[quickcheck]
    fn prop_total_supply_constant(
        initial_supply: u64,
        transfer_amount: u64,
        recipient: Address
    ) -> bool {
        let mut contract = Token::new(initial_supply);
        
        // Skip if transfer would fail
        if transfer_amount > initial_supply {
            return true;
        }
        
        contract.transfer(recipient, transfer_amount).unwrap();
        contract.total_supply == initial_supply
    }

    #[quickcheck]
    fn prop_transfer_balances(
        initial_supply: u64,
        transfer_amount: u64,
        recipient: Address
    ) -> bool {
        let mut contract = Token::new(initial_supply);
        
        // Skip if transfer would fail
        if transfer_amount > initial_supply {
            return true;
        }
        
        let sender = msg::sender();
        let initial_sender_balance = contract.balance_of(sender);
        let initial_recipient_balance = contract.balance_of(recipient);
        
        contract.transfer(recipient, transfer_amount).unwrap();
        
        contract.balance_of(sender) == initial_sender_balance - transfer_amount &&
        contract.balance_of(recipient) == initial_recipient_balance + transfer_amount
    }
}

// Formal verification
#[cfg(test)]
mod verification {
    use super::*;
    use stremax::verify;

    #[verify]
    fn verify_total_supply_constant(contract: &Token) {
        let initial_supply = contract.total_supply;
        
        forall |recipient: Address, amount: u64| {
            let mut contract = contract.clone();
            if amount <= contract.balance_of(msg::sender()) {
                contract.transfer(recipient, amount).unwrap();
                assert_eq!(contract.total_supply, initial_supply);
            }
        }
    }

    #[verify]
    #[invariant]
    fn verify_balance_sum(contract: &Token) -> bool {
        let sum: u64 = contract.balances.values().sum();
        contract.total_supply == sum
    }
}

// Gas tests
#[cfg(test)]
mod gas_tests {
    use super::*;
    use stremax::gas;

    #[test]
    fn test_transfer_gas() {
        let mut contract = Token::new(1000);
        let recipient = Address::from([1u8; 20]);
        
        let gas_used = gas::measure(|| {
            contract.transfer(recipient, 100).unwrap();
        });
        
        assert!(gas_used < 50000, "Transfer uses too much gas");
    }

    #[test]
    fn compare_transfer_implementations() {
        let mut contract = Token::new(1000);
        let recipient = Address::from([1u8; 20]);
        
        let old_gas = gas::measure(|| {
            contract.transfer(recipient, 100).unwrap();
        });
        
        let new_gas = gas::measure(|| {
            contract.optimized_transfer(recipient, 100).unwrap();
        });
        
        assert!(new_gas < old_gas, "New implementation should use less gas");
    }
}

// Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;
    use stremax::test_chain;

    #[test]
    async fn test_cross_chain_transfer() {
        let chain_a = test_chain::new(ChainConfig::Ethereum);
        let chain_b = test_chain::new(ChainConfig::Polygon);
        
        // Deploy contracts
        let token_a = chain_a.deploy(Token::new(1000));
        let token_b = chain_b.deploy(Token::new(0));
        
        // Perform cross-chain transfer
        let amount = 100;
        let recipient = Address::from([1u8; 20]);
        
        token_a.cross_chain_transfer(chain_b.id(), recipient, amount).await.unwrap();
        
        // Verify balances
        assert_eq!(token_a.balance_of(msg::sender()), 900);
        assert_eq!(token_b.balance_of(recipient), 100);
    }
}

// Privacy tests
#[cfg(test)]
mod privacy_tests {
    use super::*;
    use stremax::privacy;

    #[test]
    fn test_private_transfer() {
        let mut contract = Token::new(1000);
        let recipient = Address::from([1u8; 20]);
        
        // Create private transaction
        let tx = privacy::PrivateTransaction::new()
            .with_amount(100)
            .with_recipient(recipient);
        
        // Generate and verify proof
        let proof = tx.generate_proof();
        assert!(privacy::verify_proof(&proof));
        
        // Execute private transfer
        contract.private_transfer(tx, proof).unwrap();
        
        // Verify balances are hidden but correct
        assert!(contract.is_balance_hidden(msg::sender()));
        assert!(contract.is_balance_hidden(recipient));
        assert_eq!(contract.reveal_balance(msg::sender()), 900);
        assert_eq!(contract.reveal_balance(recipient), 100);
    }
} 