# Stremax Language Guide

## Introduction

Stremax is a modern programming language designed specifically for blockchain development and smart contract execution. It combines the safety and expressiveness of Rust with blockchain-specific features and optimizations.

## Key Features

- Memory safety through ownership system
- Built-in cryptographic primitives
- Cross-chain compatibility
- Efficient smart contract execution
- Modern package management
- Comprehensive testing framework

## Getting Started

### Installation

```bash
curl -sSf https://stremax.io/install.sh | sh
```

### Creating a New Project

```bash
strxc new my_project
cd my_project
```

### Project Structure

```
my_project/
├── stremax.toml      # Project manifest
├── src/
│   ├── lib.strx      # Library code
│   └── main.strx     # Entry point
├── tests/            # Test files
└── target/          # Build output
```

## Language Basics

### Variables and Types

```stremax
// Variables are immutable by default
let x: u256 = 42;

// Mutable variables
let mut y: u256 = 0;
y = 100;

// Basic types
let amount: u256 = 1000;
let address: Address = 0x123...;
let flag: bool = true;
let text: string = "Hello";

// Complex types
let balances: Map<Address, u256>;
let result: Result<u256, Error>;
```

### Functions

```stremax
// Pure function (no state modification)
pure fn add(a: u256, b: u256) -> u256 {
    a + b
}

// Mutable function
mut fn transfer(to: Address, amount: u256) -> Result<(), Error> {
    ensure!(balance[msg.sender] >= amount, "Insufficient balance");
    balance[msg.sender] -= amount;
    balance[to] += amount;
    Ok(())
}
```

### Smart Contracts

```stremax
contract TokenContract {
    // State variables
    state balance: Map<Address, u256>;
    
    // Events
    event Transfer(from: Address, to: Address, amount: u256);
    
    // Constructor
    fn init() {
        balance[msg.sender] = 1000000;
    }
    
    // View function
    pure fn get_balance(owner: Address) -> u256 {
        balance[owner]
    }
    
    // Mutable function with reentrancy protection
    @no_reentry
    mut fn transfer(to: Address, amount: u256) -> Result<(), Error> {
        let sender = msg.sender;
        ensure!(balance[sender] >= amount, "Insufficient balance");
        
        balance[sender] -= amount;
        balance[to] += amount;
        
        emit Transfer(sender, to, amount);
        Ok(())
    }
}
```

## Cross-Chain Development

### Bridge Configuration

```stremax
use bridge::BridgeConfig;

let config = BridgeConfig {
    bitcoin_network: Network::Mainnet,
    ethereum_chain_id: 1,
    ton_network: "mainnet",
    validators: vec![/* ... */],
    threshold: 3,
};
```

### Cross-Chain Messages

```stremax
// Send tokens to another chain
mut fn bridge_tokens(
    target_chain: ChainType,
    recipient: Address,
    amount: u256,
) -> Result<(), Error> {
    // Lock tokens on current chain
    self.balance[msg.sender] -= amount;
    
    // Create cross-chain message
    let message = CrossChainMessage {
        source_chain: ChainType::Stremax,
        target_chain,
        payload: encode_transfer(recipient, amount),
        signatures: vec![],
    };
    
    // Submit to bridge
    bridge.submit_message(message)
}
```

## Package Management

### Project Manifest (stremax.toml)

```toml
[package]
name = "my_contract"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
token = "1.0"
bridge = "0.5"

[dev-dependencies]
test-utils = "0.1"
```

### Publishing a Package

```bash
strxc publish
```

## Testing

### Unit Tests

```stremax
#[test]
fn test_transfer() {
    // Setup
    let env = TestEnvironment::new();
    let contract = TokenContract::new();
    
    // Test
    let result = contract.transfer(
        Address::from("0x123..."),
        100,
    );
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(contract.balance[msg.sender], 900);
}
```

### Integration Tests

```stremax
#[test]
fn test_cross_chain_transfer() {
    let mut runner = TestRunner::new();
    
    runner.add_test("bridge_transfer", |env| {
        // Setup
        let token = deploy_token_contract(env)?;
        let bridge = deploy_bridge_contract(env)?;
        
        // Execute transfer
        token.bridge_tokens(
            ChainType::Ethereum,
            eth_address,
            1000,
        )?;
        
        // Verify
        assert::event_emitted(env, "TokensBridged")?;
        assert::balance_equals(env, sender, 0)?;
        
        Ok(())
    });
    
    runner.run();
}
```

## Best Practices

### Security

1. Always use `@no_reentry` for functions that modify state
2. Use `ensure!` for input validation
3. Follow the checks-effects-interactions pattern
4. Implement proper access control
5. Use safe math operations

### Performance

1. Minimize storage operations
2. Batch updates when possible
3. Use appropriate data structures
4. Implement gas-efficient algorithms
5. Cache frequently accessed values

### Code Organization

1. Separate concerns into different contracts
2. Use interfaces for contract interaction
3. Keep functions small and focused
4. Document code thoroughly
5. Follow consistent naming conventions

## Deployment

### Local Testing

```bash
strxc test
```

### Network Deployment

```bash
strxc deploy --network mainnet
```

### Contract Verification

```bash
strxc verify --network mainnet --contract 0x123...
```

## Tools and Ecosystem

- Stremax CLI (`strxc`)
- Package Registry (registry.stremax.io)
- IDE Support (VS Code, IntelliJ)
- Block Explorer
- Development Framework

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more information.
