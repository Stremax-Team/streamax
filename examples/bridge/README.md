# Cross-Chain Bridge Contracts

This directory contains contracts implementing cross-chain functionality, allowing assets and messages to be transferred between different blockchain networks.

## CrossChainToken

The `CrossChainToken` contract implements a token that can be transferred across different blockchain networks using a bridge mechanism.

### Architecture

1. **Bridge Components**
   - Token contracts on each chain
   - Bridge validators
   - Message passing system
   - Proof verification

2. **Token Mechanics**
   - Burn on source chain
   - Mint on target chain
   - Maintain total supply across chains
   - Track cross-chain transfers

3. **Security Model**
   - Multi-validator consensus
   - Proof verification
   - Nonce management
   - Double-spend prevention

### Key Features

1. **Cross-Chain Transfers**
   ```rust
   // Transfer tokens to another chain
   token.transfer_cross_chain(
       recipient,
       amount,
       target_chain
   );
   
   // Receive tokens from another chain
   token.receive_tokens(
       message,
       proof
   );
   ```

2. **Chain Management**
   ```rust
   // Add support for a new chain
   token.add_supported_chain(chain_id);
   
   // Remove chain support
   token.remove_supported_chain(chain_id);
   ```

3. **Message Verification**
   - Cryptographic proof verification
   - Chain ID validation
   - Nonce checking
   - Message deduplication

### Security Considerations

1. **Bridge Security**
   - Validator set management
   - Threshold signatures
   - Proper proof verification
   - Message replay protection

2. **Token Security**
   - Atomic operations
   - Balance consistency
   - Supply management
   - Access control

3. **Cross-Chain Security**
   - Chain ID validation
   - Nonce management
   - Message uniqueness
   - Timeout handling

### Message Format

Cross-chain messages include:
```rust
struct Message {
    source_chain: ChainId,
    target_chain: ChainId,
    sender: Address,
    recipient: Address,
    amount: u256,
    nonce: u256,
}
```

### Proof Format

Proofs contain:
```rust
struct Proof {
    signatures: Vec<Signature>,
    block_number: u256,
    block_hash: bytes32,
    transaction_hash: bytes32,
}
```

### Flow Diagram

```
Chain A                 Bridge                 Chain B
   |                      |                      |
   |--- Transfer -------->|                      |
   |   (Burn tokens)      |                      |
   |                      |--- Verify Message -->|
   |                      |                      |
   |                      |<-- Submit Proof -----|
   |                      |                      |
   |                      |---- Mint Tokens ---->|
   |                      |                      |
```

### Testing

The contract includes tests for:
- Cross-chain transfers
- Proof verification
- Chain management
- Error cases
- Security scenarios

Run tests with:
```bash
stremax test examples/bridge/
```

### Bridge Configuration

Example bridge configuration:
```rust
let bridge_config = BridgeConfig {
    validators: vec![
        "0x1234...",
        "0x5678...",
        "0x9abc..."
    ],
    threshold: 2,  // 2/3 validators required
    // ... other settings
};
```

### Security Best Practices

1. **Validator Management**
   - Regular validator rotation
   - Threshold signature scheme
   - Validator incentives
   - Slashing conditions

2. **Message Processing**
   - Ordered message processing
   - Timeout handling
   - Error recovery
   - State reconciliation

3. **Asset Security**
   - Lock/unlock mechanism
   - Balance verification
   - Supply consistency
   - Emergency procedures

4. **Network Security**
   - Chain ID verification
   - Network upgrades
   - Version compatibility
   - Pause mechanism 