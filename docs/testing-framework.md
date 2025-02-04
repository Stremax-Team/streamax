# Stremax Testing Framework

## Overview

The Stremax testing framework provides comprehensive tools for testing smart contracts, language features, and cross-chain functionality at multiple levels.

## Test Types

### 1. Unit Testing

```rust
#[test]
mod token_tests {
    use stremax::testing::*;
    
    #[test]
    fn test_transfer() {
        // Setup
        let mut contract = Token::new();
        let sender = Address::from("0x1");
        let recipient = Address::from("0x2");
        
        // Execute
        let result = contract.transfer(recipient, 100);
        
        // Verify
        assert!(result.is_ok());
        assert_eq!(contract.balance_of(sender), 900);
        assert_eq!(contract.balance_of(recipient), 100);
    }
    
    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_insufficient_balance() {
        let contract = Token::new();
        contract.transfer(Address::from("0x2"), 1000000);
    }
}
```

### 2. Property-Based Testing

```rust
#[quickcheck]
mod property_tests {
    #[property]
    fn total_supply_constant(
        sender: Address,
        recipient: Address,
        amount: u64
    ) -> bool {
        let contract = Token::new();
        let initial_supply = contract.total_supply();
        
        // Property: transfers don't change total supply
        contract.transfer(recipient, amount)?;
        contract.total_supply() == initial_supply
    }
    
    #[property]
    fn no_double_spend(tx1: Transaction, tx2: Transaction) -> bool {
        // Property: can't spend same tokens twice
        let mut state = State::new();
        let result1 = state.apply(tx1);
        let result2 = state.apply(tx2);
        
        !(result1.is_ok() && result2.is_ok())
    }
}
```

### 3. Integration Testing

```rust
#[integration_test]
async fn test_cross_chain_transfer() {
    // Setup test networks
    let chain_a = TestChain::new(ChainConfig::Ethereum);
    let chain_b = TestChain::new(ChainConfig::Polygon);
    
    // Deploy contracts
    let token_a = chain_a.deploy(TokenContract::new());
    let token_b = chain_b.deploy(TokenContract::new());
    
    // Test cross-chain transfer
    let result = token_a.cross_chain_transfer(
        chain_b.id(),
        recipient,
        amount
    ).await;
    
    assert!(result.is_ok());
    assert_eq!(token_b.balance_of(recipient), amount);
}
```

### 4. Formal Verification Tests

```rust
#[verify]
mod verification_tests {
    #[invariant]
    fn balance_sum_equals_total_supply(contract: &Token) -> bool {
        let total = contract.total_supply();
        let sum: u64 = contract.balances.values().sum();
        total == sum
    }
    
    #[temporal_property]
    fn eventually_processed(tx: Transaction) -> bool {
        // All valid transactions eventually get processed
        eventually!(state.contains(tx))
    }
}
```

### 5. Gas Testing

```rust
#[gas_test]
mod gas_tests {
    #[test]
    fn test_transfer_gas() {
        let profiler = GasProfiler::new();
        
        // Profile gas usage
        profiler.start();
        contract.transfer(recipient, 100);
        let report = profiler.stop();
        
        // Verify gas usage within limits
        assert!(report.total_gas < 50000);
        assert!(report.storage_gas < 20000);
    }
    
    #[test]
    fn compare_gas_optimizations() {
        let old_gas = measure_gas(|| old_implementation());
        let new_gas = measure_gas(|| new_implementation());
        
        assert!(new_gas < old_gas);
    }
}
```

### 6. Privacy Testing

```rust
#[privacy_test]
mod privacy_tests {
    #[test]
    fn test_private_transaction() {
        let tx = PrivateTransaction::new()
            .with_amount(100)
            .with_recipient(recipient);
            
        // Verify transaction privacy
        assert!(tx.is_amount_hidden());
        assert!(tx.is_sender_anonymous());
        
        // Verify correct execution
        let proof = tx.generate_proof();
        assert!(verify_private_tx(tx, proof));
    }
}
```

## Test Infrastructure

### 1. Test Environment Setup

```rust
#[test_environment]
fn setup_test_env() -> TestEnv {
    TestEnv::new()
        .with_chain(ChainConfig::default())
        .with_accounts(test_accounts())
        .with_gas_limit(1_000_000)
        .with_block_time(Duration::from_secs(15))
}
```

### 2. Mock Components

```rust
#[mock]
mod mocks {
    // Mock blockchain
    mock_blockchain! {
        name: TestChain,
        features: [Transactions, Storage, Events]
    }
    
    // Mock external oracle
    mock_oracle! {
        name: PriceOracle,
        data: price_feed_data()
    }
}
```

### 3. Test Networks

```rust
#[test_network]
fn setup_test_network() -> TestNetwork {
    TestNetwork::new()
        .add_validator(validator_config())
        .add_full_node(node_config())
        .with_consensus(ConsensusConfig::PoA)
}
```

## Testing Tools

### 1. State Inspection

```rust
#[test]
fn inspect_state_changes() {
    let inspector = StateInspector::new();
    
    // Record state changes
    inspector.start();
    contract.complex_operation();
    let changes = inspector.stop();
    
    // Verify state changes
    assert_eq!(changes.storage_writes.len(), 2);
    assert!(changes.events.contains("Transfer"));
}
```

### 2. Transaction Simulation

```rust
#[test]
fn simulate_complex_scenario() {
    let sim = TransactionSimulator::new();
    
    // Setup scenario
    sim.set_block_number(1000);
    sim.set_timestamp(current_time);
    
    // Run simulation
    let result = sim.execute_transactions(vec![
        tx1, tx2, tx3
    ]);
    
    // Verify results
    assert_all_succeeded(&result);
}
```

### 3. Coverage Analysis

```rust
#[test]
fn analyze_test_coverage() {
    let coverage = CoverageAnalyzer::new();
    
    // Run tests with coverage
    coverage.start();
    run_all_tests();
    let report = coverage.generate_report();
    
    // Verify coverage metrics
    assert!(report.line_coverage > 90.0);
    assert!(report.branch_coverage > 85.0);
}
```

## Best Practices

1. **Test Organization**
   - Group related tests together
   - Use descriptive test names
   - Follow arrange-act-assert pattern

2. **Test Data**
   - Use realistic test data
   - Test edge cases
   - Avoid hardcoded values

3. **Gas Optimization**
   - Profile gas usage regularly
   - Compare gas costs after changes
   - Test with different network conditions

4. **Cross-Chain Testing**
   - Test with multiple chain configurations
   - Verify message passing
   - Test failure scenarios

## Common Testing Patterns

### 1. Setup Patterns

```rust
#[test]
fn standard_test_setup() {
    // Common setup
    let env = TestEnv::default();
    let contract = env.deploy(Contract::new());
    
    // Test-specific setup
    let initial_state = setup_initial_state();
    
    // Test execution
    let result = contract.operation();
    
    // Verification
    assert_state_valid(result);
}
```

### 2. Cleanup Patterns

```rust
#[test]
fn cleanup_after_test() {
    let resources = setup_resources();
    
    // Ensure cleanup
    defer! {
        cleanup_resources(resources);
    }
    
    // Test execution
    perform_test();
}
```
