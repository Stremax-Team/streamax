# Gas Optimization Guide

This guide covers best practices and techniques for optimizing gas usage in Stremax smart contracts.

## Understanding Gas Costs

### Basic Operations
| Operation | Gas Cost | Notes |
|-----------|----------|-------|
| Storage Write | 20000 | Per 32 bytes |
| Storage Read | 200 | Per 32 bytes |
| Memory Operation | 3 | Per byte |
| Contract Call | 700 | Base cost |
| Contract Creation | 32000 | Base cost |
| Zero Byte | 4 | In calldata |
| Non-Zero Byte | 68 | In calldata |

## Optimization Techniques

### 1. Storage Optimization

```rust
// Bad: Multiple storage slots
struct Token {
    name: String,      // 1 slot
    symbol: String,    // 1 slot
    decimals: u8,      // 1 slot
    total_supply: u64  // 1 slot
}

// Good: Packed storage
#[packed_storage]
struct Token {
    decimals: u8,      // Packed together
    symbol: [u8; 3],   // in a single
    name: [u8; 20],    // 32-byte slot
    total_supply: u64  // New slot
}
```

### 2. Memory Management

```rust
// Bad: Unnecessary memory allocation
fn process_array(data: Vec<u64>) {
    let mut temp = Vec::new();
    for item in data {
        temp.push(item * 2);
    }
    // Process temp...
}

// Good: In-place processing
fn process_array(mut data: Vec<u64>) {
    for item in &mut data {
        *item *= 2;
    }
    // Process data...
}
```

### 3. Batch Operations

```rust
// Bad: Multiple separate transfers
fn distribute_rewards(recipients: Vec<Address>, amount: u64) {
    for recipient in recipients {
        transfer(recipient, amount);  // Each transfer costs gas
    }
}

// Good: Batch transfer
fn distribute_rewards(recipients: Vec<Address>, amount: u64) {
    batch_transfer(recipients, amount);  // Single operation
}
```

### 4. Caching

```rust
// Bad: Repeated storage reads
fn process_balance() {
    let balance = self.balance.get();  // Storage read
    if balance > 100 {
        // Do something
        let new_balance = self.balance.get();  // Another read
    }
}

// Good: Cache storage values
fn process_balance() {
    let balance = self.balance.get();  // Single storage read
    if balance > 100 {
        // Do something
        let new_balance = balance;  // Use cached value
    }
}
```

### 5. Event Optimization

```rust
// Bad: Emitting unnecessary data
#[event]
struct Transfer {
    from: Address,
    to: Address,
    amount: u64,
    timestamp: u64,
    block: u64,
    transaction: Hash,
}

// Good: Emit only essential data
#[event]
struct Transfer {
    #[indexed]
    from: Address,
    #[indexed]
    to: Address,
    amount: u64,
}
```

## Advanced Optimizations

### 1. Bit Packing

```rust
// Pack multiple flags into a single u8
#[bitpack]
struct Flags {
    is_active: bool,     // 1 bit
    is_frozen: bool,     // 1 bit
    role: Role,          // 3 bits
    permissions: u8,     // 3 bits
}
```

### 2. Lazy Loading

```rust
// Load data only when needed
struct LazyMap<K, V> {
    data: StorageMap<K, V>,
    cache: RefCell<HashMap<K, V>>,
}

impl<K, V> LazyMap<K, V> {
    fn get(&self, key: K) -> V {
        if let Some(v) = self.cache.borrow().get(&key) {
            return v.clone();
        }
        let value = self.data.get(key);
        self.cache.borrow_mut().insert(key, value.clone());
        value
    }
}
```

### 3. Proxy Patterns

```rust
#[proxy]
contract UpgradeableContract {
    fn initialize() {
        // Initialization logic
    }
    
    #[delegate_call]
    fn execute_logic() {
        // Business logic in separate contract
    }
}
```

## Gas Profiling Tools

### 1. Gas Profiler Usage

```bash
# Profile a specific contract
strm profile contracts/Token.strx

# Profile with detailed breakdown
strm profile --detailed contracts/Token.strx

# Compare gas usage between versions
strm profile --diff v1.strx v2.strx
```

### 2. Gas Estimation

```rust
#[gas_estimate]
fn estimate_transaction_cost(tx: Transaction) -> Gas {
    // Estimate gas cost before execution
}
```

## Best Practices

1. **Storage**
   - Pack related storage variables
   - Use appropriate data types
   - Minimize storage writes

2. **Computation**
   - Use efficient algorithms
   - Avoid unnecessary loops
   - Cache frequently used values

3. **Memory**
   - Reuse memory when possible
   - Clear unused memory
   - Use appropriate data structures

4. **Events**
   - Index important fields
   - Emit minimal required data
   - Use structured logging

## Common Pitfalls

1. **Unbounded Operations**
   ```rust
   // Bad: Unbounded loop
   fn process_all(data: Vec<T>) {
       for item in data {  // Could run out of gas
           process(item);
       }
   }
   
   // Good: Bounded operation
   fn process_batch(data: Vec<T>, limit: usize) {
       for item in data.iter().take(limit) {
           process(item);
       }
   }
   ```

2. **Unnecessary Storage**
   ```rust
   // Bad: Storing calculable data
   storage {
       total: u64,
       squares: Map<u64, u64>,  // Unnecessary storage
   }
   
   // Good: Calculate on demand
   fn get_square(n: u64) -> u64 {
       n * n  // Compute instead of store
   }
   ```

## Testing Gas Usage

```rust
#[test]
fn test_gas_usage() {
    let contract = Token::new();
    
    #[gas_track]
    let result = contract.transfer(recipient, amount);
    
    assert!(result.gas_used() < GAS_LIMIT);
}
```
