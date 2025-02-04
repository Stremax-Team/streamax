# Getting Started with Stremax

This guide will help you get up and running with Stremax, from installation to writing your first smart contract.

## Prerequisites

- Linux, macOS, or Windows
- Rust toolchain (1.70.0 or later)
- Git
- A code editor with Stremax support (VS Code recommended)

## Installation

1. Install the Stremax toolchain:
```bash
curl -sSf https://stremax.io/install.sh | sh
```

2. Verify the installation:
```bash
strm --version
```

## Creating Your First Project

1. Create a new project:
```bash
strm new my_first_project
cd my_first_project
```

2. Project structure:
```
my_first_project/
├── Cargo.toml
├── .gitignore
├── README.md
├── src/
│   ├── lib.rs
│   └── contracts/
│       └── token.strx
└── tests/
    └── token_tests.rs
```

## Writing Your First Smart Contract

Create a simple token contract in `src/contracts/token.strx`:

```rust
contract Token {
    storage {
        total_supply: u64,
        balances: Map<Address, u64>
    }

    // Initialize the contract
    public fn init(initial_supply: u64) {
        self.total_supply = initial_supply;
        self.balances[msg::sender()] = initial_supply;
    }

    // Transfer tokens
    public fn transfer(to: Address, amount: u64) -> Result<()> {
        let sender = msg::sender();
        ensure!(self.balances[sender] >= amount, "Insufficient balance");
        
        self.balances[sender] -= amount;
        self.balances[to] += amount;
        
        emit Transfer(sender, to, amount);
        Ok(())
    }

    // View balance
    public view fn balance_of(account: Address) -> u64 {
        self.balances[account]
    }
}
```

## Testing Your Contract

Create a test file in `tests/token_tests.rs`:

```rust
#[test]
fn test_token_transfer() {
    let mut contract = Token::new();
    
    // Initialize with 1000 tokens
    contract.init(1000);
    
    // Test transfer
    let result = contract.transfer(Address::from("0x123"), 100);
    assert!(result.is_ok());
    
    // Check balances
    assert_eq!(contract.balance_of(msg::sender()), 900);
    assert_eq!(contract.balance_of(Address::from("0x123")), 100);
}
```

Run the tests:
```bash
strm test
```

## Building and Deploying

1. Build your contract:
```bash
strm build
```

2. Deploy to testnet:
```bash
strm deploy --network testnet
```

## Next Steps

- Learn about [Smart Contract Development](smart-contracts/README.md)
- Explore [Security Best Practices](best-practices.md)
- Read the [Language Specification](../src/docs/LANGUAGE_SPEC.md)

## Development Tools

- Install the VS Code extension:
```bash
code --install-extension stremax.stremax-lang
```

- Enable hot reloading during development:
```bash
strm dev --hot-reload
```

## Common Issues and Solutions

### Gas Optimization
- Use the gas profiler to optimize your contract:
```bash
strm profile my_contract.strx
```

### Debugging
- Enable debug logging:
```bash
RUST_LOG=debug strm build
```

### IDE Support
- If autocomplete isn't working, try:
```bash
strm language-server --restart
```

## Additional Resources

- [Standard Library Documentation](stdlib/README.md)
- [Example Projects](../examples/)
- [API Reference](api-reference.md)
- [Community Forums](https://forum.stremax.io) 