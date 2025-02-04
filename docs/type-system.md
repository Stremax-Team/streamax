# Stremax Type System

Stremax features a powerful, static type system designed for blockchain development, combining safety and expressiveness.

## Core Types

### Primitive Types
- `u8`, `u16`, `u32`, `u64`, `u128`: Unsigned integers
- `i8`, `i16`, `i32`, `i64`, `i128`: Signed integers
- `bool`: Boolean values
- `char`: Unicode character
- `str`: String slice
- `String`: Owned string

### Blockchain-Specific Types
- `Address`: Blockchain address
- `Hash`: Cryptographic hash
- `Signature`: Digital signature
- `Amount`: Token amount with overflow protection
- `Gas`: Gas units

### Resource Types
```rust
resource Token {
    amount: Amount,
    issuer: Address,
    
    fn split(self, value: Amount) -> (Token, Token) {
        // Resource types ensure no double-spending
    }
}
```

## Type Safety Features

### Linear Types
```rust
fn transfer(token: Token) {  // Token is consumed
    // Token can only be used once
}

// Won't compile - token already consumed
// transfer(token);
```

### Ownership and Borrowing
```rust
fn view_balance(account: &Account) {  // Borrowed reference
    // Can read but not modify
}

fn update_balance(account: &mut Account) {  // Mutable reference
    // Can modify the account
}
```

## Generic Types

### Type Parameters
```rust
struct Map<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Hash, V> Map<K, V> {
    fn insert(&mut self, key: K, value: V) {
        // Implementation
    }
}
```

### Trait Bounds
```rust
trait Transferable {
    fn transfer(self, to: Address) -> Result<()>;
}

fn send<T: Transferable>(asset: T, recipient: Address) {
    asset.transfer(recipient)?;
}
```

## Sum Types

### Enums
```rust
enum TransactionStatus {
    Pending,
    Confirmed(BlockNumber),
    Failed(Error),
}
```

### Pattern Matching
```rust
match status {
    TransactionStatus::Pending => wait(),
    TransactionStatus::Confirmed(block) => process(block),
    TransactionStatus::Failed(error) => handle_error(error),
}
```

## Effect System

### Effect Types
```rust
effect Storage {
    read(key: Vec<u8>) -> Option<Vec<u8>>,
    write(key: Vec<u8>, value: Vec<u8>),
}

fn store_value() with Storage {
    Storage::write(b"key", b"value")
}
```

### Permission Types
```rust
permission ContractCall {
    target: Address,
    gas_limit: Gas,
}

fn call_external(perm: Permission<ContractCall>) {
    // Can only call if permission is granted
}
```

## Type Inference

```rust
// Type inference in variable declarations
let x = 42;  // Inferred as i32
let map = Map::new();  // Types inferred from usage

// Type inference in closures
let numbers = vec![1, 2, 3];
let doubled = numbers.map(|x| x * 2);
```

## Zero-Cost Abstractions

```rust
// High-level abstractions compile to efficient code
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// Compiles to optimal machine code
for item in collection {
    process(item);
}
```

## Type System Extensions

### Custom Types
```rust
#[derive(Clone, Debug, Serialize)]
struct CustomToken {
    name: String,
    decimals: u8,
    total_supply: Amount,
}
```

### Type Aliases
```rust
type Balance = Amount;
type AccountMap = Map<Address, Balance>;
```

## Best Practices

1. **Use Resource Types** for assets that need ownership tracking
2. **Leverage Type Inference** while maintaining readability
3. **Define Clear Traits** for common behavior
4. **Use Generic Constraints** to ensure type safety
5. **Implement Custom Types** for domain-specific logic

## Common Patterns

### Builder Pattern
```rust
struct ContractBuilder {
    name: Option<String>,
    initial_supply: Option<Amount>,
}

impl ContractBuilder {
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    fn build(self) -> Result<Contract> {
        // Build contract with validated parameters
    }
}
```

### Type-State Pattern
```rust
struct Uninitialized;
struct Initialized;

struct Contract<State> {
    balance: Amount,
    _state: PhantomData<State>,
}

impl Contract<Uninitialized> {
    fn initialize(self) -> Contract<Initialized> {
        // Initialize contract
    }
}
```

## Error Handling

```rust
// Result type for fallible operations
type Result<T> = std::result::Result<T, ContractError>;

// Custom error types
enum ContractError {
    InsufficientBalance(Amount),
    Unauthorized(Address),
    InvalidState,
}
```

## Advanced Topics

1. **Dependent Types**
2. **Higher-Kinded Types**
3. **Type-Level Programming**
4. **Refinement Types**
5. **Linear Type System Extensions** 