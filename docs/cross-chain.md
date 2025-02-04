# Cross-Chain Integration Guide

This guide covers Stremax's cross-chain capabilities, enabling seamless interaction between different blockchain networks.

## Overview

Stremax provides native support for cross-chain operations through:
- Message passing protocol
- State verification
- Asset bridging
- Cross-chain smart contract calls

## Core Components

### 1. Bridge Protocol

```rust
#[bridge_protocol]
trait ChainBridge {
    fn send_message(target_chain: ChainId, message: Message) -> Result<Hash>;
    fn receive_message(source_chain: ChainId, proof: Proof) -> Result<Message>;
    fn verify_state(chain: ChainId, state_root: Hash) -> Result<bool>;
}
```

### 2. Cross-Chain Messages

```rust
#[derive(Message)]
struct CrossChainMessage {
    source_chain: ChainId,
    target_chain: ChainId,
    nonce: u64,
    payload: Vec<u8>,
    signature: Signature,
}

impl CrossChainMessage {
    fn verify(&self) -> Result<()> {
        // Verify message authenticity
        verify_signature(self.signature, self.payload)?;
        verify_nonce(self.source_chain, self.nonce)?;
        Ok(())
    }
}
```

## Implementation Examples

### 1. Token Bridge

```rust
#[cross_chain_contract]
contract TokenBridge {
    storage {
        locked_tokens: Map<ChainId, Map<Address, Amount>>,
        processed_nonces: Set<(ChainId, u64)>,
    }
    
    // Lock tokens on source chain
    pub fn lock_tokens(
        target_chain: ChainId,
        amount: Amount,
        recipient: Address,
    ) -> Result<()> {
        let sender = msg::sender();
        self.token.transfer_from(sender, self.address, amount)?;
        self.locked_tokens[target_chain][sender] += amount;
        
        emit TokensLocked(sender, target_chain, amount);
        Ok(())
    }
    
    // Release tokens on target chain
    #[verify_proof]
    pub fn release_tokens(
        proof: Proof,
        source_chain: ChainId,
        amount: Amount,
    ) -> Result<()> {
        verify_cross_chain_proof(proof)?;
        ensure!(!self.processed_nonces.contains((source_chain, proof.nonce)));
        
        self.token.mint(msg::sender(), amount)?;
        self.processed_nonces.insert((source_chain, proof.nonce));
        
        emit TokensReleased(msg::sender(), source_chain, amount);
        Ok(())
    }
}
```

### 2. Cross-Chain State Verification

```rust
#[state_verifier]
contract StateVerifier {
    storage {
        state_roots: Map<ChainId, Map<BlockNumber, Hash>>,
        validators: Set<Address>,
    }
    
    // Update state root with validator consensus
    #[require_consensus]
    pub fn update_state_root(
        chain: ChainId,
        block: BlockNumber,
        root: Hash,
        signatures: Vec<Signature>,
    ) -> Result<()> {
        verify_validator_signatures(signatures)?;
        self.state_roots[chain][block] = root;
        emit StateRootUpdated(chain, block, root);
        Ok(())
    }
    
    // Verify transaction inclusion
    pub fn verify_tx(
        chain: ChainId,
        block: BlockNumber,
        tx_hash: Hash,
        proof: MerkleProof,
    ) -> Result<bool> {
        let root = self.state_roots[chain][block]
            .ok_or(Error::StateRootNotFound)?;
        verify_merkle_proof(proof, root, tx_hash)
    }
}
```

## Security Considerations

### 1. Double-Spend Prevention

```rust
trait DoubleSpendPrevention {
    // Track processed messages
    fn is_processed(&self, message_id: Hash) -> bool;
    
    // Mark message as processed
    fn mark_processed(&mut self, message_id: Hash);
    
    // Clean up old processed messages
    fn cleanup_processed(&mut self, before: BlockNumber);
}
```

### 2. Consensus Verification

```rust
#[consensus_verification]
fn verify_consensus(
    signatures: &[Signature],
    message: &[u8],
    threshold: usize,
) -> Result<()> {
    let valid_sigs = signatures
        .iter()
        .filter(|sig| verify_validator_signature(sig, message))
        .count();
    
    ensure!(
        valid_sigs >= threshold,
        "Insufficient validator signatures"
    );
    Ok(())
}
```

## Advanced Features

### 1. Atomic Swaps

```rust
#[atomic_swap]
contract CrossChainSwap {
    #[locked_operation]
    fn swap_assets(
        asset_a: Asset,
        chain_b: ChainId,
        asset_b: Asset,
        timeout: BlockNumber,
    ) -> Result<()> {
        // Atomic swap implementation
    }
}
```

### 2. Cross-Chain Calls

```rust
#[cross_chain_call]
async fn call_remote_contract(
    chain: ChainId,
    contract: Address,
    function: String,
    args: Vec<Value>,
) -> Result<Value> {
    let message = encode_contract_call(function, args);
    let response = await!(send_cross_chain_message(chain, contract, message))?;
    decode_response(response)
}
```

## Performance Optimization

### 1. Batched Messages

```rust
#[batch_messages]
fn send_batch(
    messages: Vec<CrossChainMessage>,
    target_chain: ChainId,
) -> Result<Hash> {
    let batch = MessageBatch::new(messages);
    let proof = batch.generate_proof();
    submit_batch_to_bridge(target_chain, batch, proof)
}
```

### 2. Optimistic Verification

```rust
#[optimistic_verification]
trait OptimisticBridge {
    // Accept message optimistically
    fn accept_message(message: Message) -> Result<()>;
    
    // Challenge invalid message
    fn challenge_message(
        message: Message,
        fraud_proof: Proof,
    ) -> Result<()>;
}
```

## Testing and Debugging

### 1. Local Testing

```rust
#[test]
fn test_cross_chain_transfer() {
    let chain_a = TestChain::new(1);
    let chain_b = TestChain::new(2);
    
    // Set up bridge between chains
    let bridge = Bridge::connect(&chain_a, &chain_b);
    
    // Test transfer
    let result = bridge.transfer_tokens(
        chain_a,
        chain_b,
        Amount::new(100),
    );
    
    assert!(result.is_ok());
}
```

### 2. Network Simulation

```rust
#[test]
fn test_network_conditions() {
    let network = NetworkSimulator::new()
        .with_latency(Duration::from_secs(1))
        .with_packet_loss(0.1);
    
    let bridge = Bridge::new().with_network(network);
    // Test bridge under different network conditions
}
```

## Best Practices

1. **Message Handling**
   - Implement idempotency
   - Use unique message IDs
   - Handle message timeouts

2. **State Verification**
   - Use merkle proofs
   - Implement fraud proofs
   - Maintain validator sets

3. **Security**
   - Implement proper access controls
   - Use secure random numbers
   - Handle chain reorganizations

## Common Issues and Solutions

1. **Message Ordering**
   - Use sequence numbers
   - Implement message queues
   - Handle out-of-order messages

2. **Chain Reorganizations**
   - Wait for sufficient confirmations
   - Implement rollback mechanisms
   - Track alternative states
