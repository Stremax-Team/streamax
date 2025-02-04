# Formal Verification in Stremax

Stremax provides built-in support for formal verification, allowing developers to mathematically prove the correctness of their smart contracts.

## Overview

Formal verification in Stremax uses:
- Static analysis
- Model checking
- Theorem proving
- Property-based testing
- Symbolic execution

## Core Features

### 1. Contract Specifications

```rust
#[contract]
#[invariant(total_supply == balances.sum())]
struct Token {
    #[invariant(>= 0)]
    total_supply: u64,
    balances: Map<Address, u64>,
}

impl Token {
    #[requires(amount <= self.balances[sender])]
    #[ensures(
        self.balances[sender] == old(self.balances[sender]) - amount &&
        self.balances[recipient] == old(self.balances[recipient]) + amount
    )]
    pub fn transfer(
        &mut self,
        sender: Address,
        recipient: Address,
        amount: u64,
    ) -> Result<()> {
        // Implementation
    }
}
```

### 2. Property Verification

```rust
#[verify]
mod token_properties {
    #[property]
    fn no_overflow(a: u64, b: u64) -> bool {
        a.checked_add(b).is_some()
    }
    
    #[property]
    fn conservation_of_tokens(
        contract: Token,
        sender: Address,
        recipient: Address,
        amount: u64,
    ) -> bool {
        let initial_total = contract.total_supply;
        contract.transfer(sender, recipient, amount)?;
        contract.total_supply == initial_total
    }
}
```

## Verification Tools

### 1. Static Analysis

```rust
#[static_analysis]
fn analyze_contract(contract: &Contract) -> Analysis {
    // Check for common vulnerabilities
    check_reentrancy(contract)?;
    check_integer_overflow(contract)?;
    check_unchecked_external_calls(contract)?;
    
    // Analyze control flow
    analyze_control_flow(contract)?;
    
    // Check state invariants
    verify_state_invariants(contract)
}
```

### 2. Model Checking

```rust
#[model_check]
fn verify_state_machine(contract: &Contract) -> Result<()> {
    let mut checker = ModelChecker::new();
    
    // Define state transitions
    checker.add_transition("transfer", |state| {
        // Define transfer state transition
    });
    
    // Verify properties
    checker.verify_property("no_double_spend")?;
    checker.verify_property("balance_conservation")?;
    
    Ok(())
}
```

### 3. Symbolic Execution

```rust
#[symbolic_execution]
fn explore_paths(contract: &Contract) -> Result<Coverage> {
    let mut explorer = SymbolicExplorer::new();
    
    // Define symbolic variables
    let amount = explorer.symbolic_u64("amount");
    let sender = explorer.symbolic_address("sender");
    
    // Explore execution paths
    explorer.explore(|state| {
        contract.transfer(sender, recipient, amount)
    })
}
```

## Advanced Features

### 1. Temporal Properties

```rust
#[temporal_verify]
fn verify_temporal_properties(contract: &Contract) {
    // Always eventually process all pending transactions
    #[always_eventually]
    fn process_pending() -> bool {
        pending_transactions.is_empty()
    }
    
    // Never allow negative balances
    #[invariant]
    fn non_negative_balance() -> bool {
        balances.values().all(|v| *v >= 0)
    }
}
```

### 2. Refinement Types

```rust
#[refine_type]
type PositiveAmount = i64 where self > 0;

#[refine_type]
type Percentage = u8 where self <= 100;

fn apply_discount(
    amount: PositiveAmount,
    discount: Percentage,
) -> PositiveAmount {
    amount * (100 - discount) / 100
}
```

## Testing Integration

### 1. Property-Based Testing

```rust
#[quickcheck]
fn test_transfer_properties(
    sender: Address,
    recipient: Address,
    amount: u64,
) -> bool {
    let mut contract = Token::new();
    
    // Property: Total supply remains constant
    let initial_supply = contract.total_supply();
    contract.transfer(sender, recipient, amount)?;
    contract.total_supply() == initial_supply
}
```

### 2. Invariant Testing

```rust
#[invariant_test]
fn test_invariants(contract: &mut Token) {
    // Test state transitions
    contract.mint(100)?;
    assert_invariants!(contract);
    
    contract.transfer(addr1, addr2, 50)?;
    assert_invariants!(contract);
    
    contract.burn(25)?;
    assert_invariants!(contract);
}
```

## Security Properties

### 1. Access Control

```rust
#[verify_access]
fn verify_permissions(contract: &Contract) {
    // Only owner can mint
    #[requires(msg::sender() == contract.owner)]
    fn mint(amount: u64) -> Result<()>;
    
    // Anyone can transfer their own tokens
    #[requires(msg::sender() == sender || is_approved(msg::sender(), sender))]
    fn transfer(sender: Address, recipient: Address, amount: u64) -> Result<()>;
}
```

### 2. Resource Safety

```rust
#[verify_resources]
fn verify_resource_usage(contract: &Contract) {
    // Verify no resource leaks
    #[resource_safe]
    fn process_payment(payment: Payment) -> Result<()>;
    
    // Verify all locks are released
    #[lock_safe]
    fn atomic_swap(token_a: Token, token_b: Token) -> Result<()>;
}
```

## Best Practices

1. **Specification Writing**
   - Write clear, concise specifications
   - Cover all edge cases
   - Use appropriate abstraction levels

2. **Verification Strategy**
   - Start with critical properties
   - Use incremental verification
   - Combine different verification techniques

3. **Performance**
   - Optimize verification scope
   - Use modular verification
   - Cache verification results

## Common Issues and Solutions

1. **State Space Explosion**
   - Use abstraction
   - Decompose properties
   - Apply bounded verification

2. **Verification Time**
   - Use incremental verification
   - Parallelize verification tasks
   - Cache intermediate results
   