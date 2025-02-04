# ![Stremax](assets/branding/stremax-logo.svg)

# Stremax Programming Language

Stremax is a modern systems programming language designed for blockchain and smart contract development, with a focus on safety, performance, and expressiveness.

## Features

### 🛡️ Safety First
- Strong static typing with powerful type inference
- Linear types for resource management
- Ownership and borrowing system
- No null pointers
- Built-in formal verification
- Permission and effect systems for fine-grained control
- Comprehensive validation rules

### ⚡ High Performance
- Zero-cost abstractions
- Efficient memory management
- Optimized for blockchain operations
- Deterministic execution
- Gas-aware optimizations
- LRU caching with performance metrics
- Concurrent module loading

### 🔗 Blockchain Native
- First-class blockchain types
- Smart contract primitives
- Cross-chain interoperability
- Built-in cryptographic primitives
- Gas metering
- ZK-SNARK integration
- Private state management

### 🎯 Developer Experience
- Modern syntax
- Rich IDE support
- Comprehensive tooling
- Extensive documentation
- Active community
- Hot module reloading
- Actor-based concurrency model

### 🔄 Module System
- Thread-safe parallel module loading
- File system watching for hot reloading
- Configurable validation rules
- Comprehensive error handling
- Plugin system through module hooks
- Optimized caching mechanisms
- Cross-module dependency management

## Quick Start

1. Install Stremax:
```bash
curl -sSf https://stremax.io/install.sh | sh
```

2. Create a new project:
```bash
strm new my_project
cd my_project
```

3. Write your first smart contract:
```rust
contract Token {
    storage {
        total_supply: u64,
        balances: Map<Address, u64>
    }

    public fn transfer(to: Address, amount: u64) -> Result<()> {
        let sender = msg::sender();
        ensure!(self.balances[sender] >= amount, "Insufficient balance");
        
        self.balances[sender] -= amount;
        self.balances[to] += amount;
        
        emit Transfer(sender, to, amount);
        Ok(())
    }
}
```

4. Build and deploy:
```bash
strm build
strm deploy
```

## Documentation

### Core Documentation
- [Getting Started Guide](docs/getting-started.md)
- [Language Specification](src/docs/LANGUAGE_SPEC.md)
- [Standard Library Reference](docs/stdlib/README.md)
- [API Reference](docs/api-reference.md)

### Guides and Best Practices
- [Smart Contract Development Guide](docs/smart-contracts/README.md)
- [Security Best Practices](docs/best-practices.md)
- [Performance Optimization Guide](docs/performance.md)
- [Testing and Debugging Guide](docs/testing.md)

### Architecture and Design
- [Architecture Overview](docs/architecture.md)
- [Module System Design](docs/README.md)
- [Concurrency Model](docs/concurrency.md)
- [Type System](docs/type-system.md)

### Advanced Features
- [Zero-Knowledge Proofs](docs/zk-proofs.md)
- [Cross-Chain Integration](docs/cross-chain.md)
- [Formal Verification](docs/verification.md)
- [Gas Optimization](docs/gas-optimization.md)

### Examples and Tutorials
- [Basic Examples](examples/basic/)
- [DeFi Contracts](examples/defi/)
- [Cross-Chain Applications](examples/cross-chain/)
- [Governance Systems](examples/governance/)

### Developer Tools
- [IDE Integration Guide](docs/ide-support.md)
- [Hot Reloading Guide](docs/hot-reloading.md)
- [Debugging Tools](docs/debugging.md)
- [Package Management](docs/package-management.md)

## Tools

Stremax comes with a comprehensive set of development tools:

- 🛠️ `strm` - Command line interface
- 📦 Package manager with hot reloading
- 🔍 Language server (IDE support)
- 🐛 Interactive debugger
- 📊 Gas profiler
- ✅ Formal verifier
- 🔄 Module hot reloader
- 🧪 ZK-SNARK toolkit

## IDE Support

Stremax provides rich IDE support through the Language Server Protocol:

- Visual Studio Code
- IntelliJ IDEA
- Sublime Text
- Vim/Neovim
- Emacs

Features include:
- Syntax highlighting
- Code completion
- Go to definition
- Find references
- Inline documentation
- Live diagnostics
- Code formatting
- Refactoring tools
- Hot reload integration
- Module dependency visualization

## Future Directions

1. **Enhanced Formal Verification**
   - Automated theorem proving
   - Property-based testing
   - Smart contract verification

2. **Advanced Cross-Chain Features**
   - Native bridge support
   - Universal chain interface
   - Cross-chain message passing

3. **Zero-Knowledge Enhancements**
   - Advanced ZK-SNARK protocols
   - Private transaction support
   - Verifiable computation

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## Community

- [Discord](https://discord.gg/stremax)
- [Twitter](https://twitter.com/stremax_lang)
- [Reddit](https://reddit.com/r/stremax)
- [Blog](https://blog.stremax.io)

## License

Stremax is licensed under the [Apache License 2.0](LICENSE).

# Stremax Smart Contract Examples

This repository contains example smart contracts written in the Stremax programming language, demonstrating various advanced blockchain development patterns and features.

## Overview

The examples showcase different aspects of blockchain development:

1. **DeFi (Decentralized Finance)**
   - `LiquidityPool`: Automated Market Maker with flash loan capabilities
   - `StakingRewards`: Advanced staking system with boosting and vesting

2. **Cross-Chain**
   - `CrossChainToken`: Token that can be transferred across different blockchain networks

3. **Governance**
   - `DAO`: Decentralized Autonomous Organization with quadratic voting and timelock

## Contract Details

### LiquidityPool (`examples/defi/liquidity_pool.strx`)
An Automated Market Maker (AMM) implementation featuring:
- Constant product formula (x * y = k)
- Flash loan functionality
- Liquidity provider shares
- Anti-slippage protection
- Reentrancy protection

### StakingRewards (`examples/defi/staking_rewards.strx`)
Advanced staking system with:
- Dynamic reward rates
- Boost multipliers (1x-3x based on lock time)
- Linear and exponential vesting schedules
- Reward distribution tracking
- Anti-manipulation protections

### CrossChainToken (`examples/bridge/cross_chain_token.strx`)
Cross-chain compatible token featuring:
- Message passing between chains
- Proof verification
- Nonce management
- Double-spend prevention
- Chain ID validation

### DAO (`examples/governance/dao.strx`)
Governance system featuring:
- Quadratic voting
- Proposal lifecycle management
- Timelock functionality
- Multi-action proposals
- Vote delegation

## Key Features Demonstrated

1. **Security**
   - Reentrancy protection
   - Input validation
   - Access control
   - State management
   - Timelock mechanisms

2. **Advanced Mathematics**
   - Constant product formula
   - Quadratic voting calculations
   - Vesting schedules
   - Reward rate computations

3. **Cross-Chain Operations**
   - Message passing
   - Proof verification
   - Chain ID management
   - Nonce tracking

4. **Testing**
   - Unit tests
   - Integration tests
   - Time manipulation
   - Event verification

## Usage

Each contract includes comprehensive tests demonstrating its functionality. To run the tests:

```bash
stremax test examples/
```

## Best Practices Demonstrated

1. **Code Organization**
   - Clear state variable grouping
   - Logical function ordering
   - Comprehensive events
   - Detailed comments

2. **Security**
   - All state changes before external calls
   - Comprehensive input validation
   - Proper access control
   - Event emission for important state changes

3. **Gas Optimization**
   - Efficient state packing
   - Minimal storage operations
   - Optimized calculations
   - View function usage

4. **Testing**
   - Comprehensive test coverage
   - Edge case testing
   - Time-dependent scenarios
   - Event verification

## License

Apache License 2.0(https://www.apache.org/licenses/LICENSE-2.0)