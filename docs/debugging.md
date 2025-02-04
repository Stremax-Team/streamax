# Debugging Tools Guide

Stremax provides a comprehensive suite of debugging tools to help developers identify and fix issues in their smart contracts.

## Core Debugging Tools

### 1. Interactive Debugger

```rust
// Launch debugger
strm debug contracts/Token.strx

// Set breakpoints in code
contract Token {
    #[breakpoint]
    fn transfer(...) {
        let sender = msg::sender();  // Break here
        // ...
    }
}
```

### 2. Console Logger

```rust
use stremax::debug::log;

contract Token {
    fn transfer(...) {
        // Different log levels
        log::debug!("Transfer initiated: {}", amount);
        log::info!("Sender: {}", sender);
        log::warn!("Large transfer detected") if amount > 1000;
        log::error!("Insufficient balance: {}", balance);
    }
}
```

### 3. State Inspector

```rust
#[debug_view]
fn inspect_state(contract: &Token) {
    // View contract state
    println!("Total Supply: {}", contract.total_supply);
    println!("Balances: {:?}", contract.balances);
    
    // View storage layout
    debug::show_storage_layout(contract);
}
```

## Advanced Features

### 1. Transaction Simulator

```rust
#[test]
fn simulate_transaction() {
    let mut sim = TransactionSimulator::new();
    
    // Set up initial state
    sim.set_balance(sender, 1000);
    
    // Simulate transaction
    let result = sim.execute(|ctx| {
        contract.transfer(recipient, 500)
    });
    
    // Inspect results
    assert_eq!(result.gas_used, 21000);
    assert!(result.events.contains("Transfer"));
}
```

### 2. Gas Profiler

```rust
#[gas_profile]
fn analyze_gas_usage() {
    let profiler = GasProfiler::new();
    
    // Profile specific function
    profiler.start();
    contract.complex_operation();
    let report = profiler.stop();
    
    // View gas breakdown
    println!("Gas Report: {:#?}", report);
}
```

### 3. Memory Analyzer

```rust
#[memory_analysis]
fn analyze_memory() {
    let analyzer = MemoryAnalyzer::new();
    
    // Track allocations
    analyzer.start_tracking();
    contract.process_large_data();
    let report = analyzer.get_report();
    
    // View memory usage
    println!("Memory Report: {:#?}", report);
}
```

## Debugging Tools

### 1. Contract Tracer

```rust
#[trace]
fn trace_execution() {
    let tracer = ContractTracer::new()
        .with_storage_tracking()
        .with_call_tracking()
        .with_event_tracking();
    
    // Execute with tracing
    tracer.run(|| {
        contract.complex_operation();
    });
    
    // View trace
    println!("Execution Trace: {:#?}", tracer.get_trace());
}
```

### 2. State Differ

```rust
#[state_diff]
fn compare_states() {
    let differ = StateDiffer::new();
    
    // Capture initial state
    let initial = differ.capture_state();
    
    // Execute operation
    contract.update_state();
    
    // Compare states
    let diff = differ.compare_with(initial);
    println!("State Changes: {:#?}", diff);
}
```

### 3. Event Monitor

```rust
#[event_monitor]
fn monitor_events() {
    let monitor = EventMonitor::new()
        .filter(EventType::Transfer)
        .filter(EventType::Approval);
    
    // Start monitoring
    monitor.start();
    contract.batch_operation();
    let events = monitor.collect();
    
    // Analyze events
    for event in events {
        println!("Event: {:?}", event);
    }
}
```

## IDE Integration

### 1. Breakpoint Management

```rust
// In VS Code
contract Token {
    fn transfer(...) {
        // F9 to toggle breakpoint
        let sender = msg::sender();
        
        // Shift+F9 for conditional breakpoint
        if amount > 1000 {
            // Break here if condition is true
        }
    }
}
```

### 2. Variable Inspection

```rust
// Watch window expressions
contract.balances[sender]
contract.total_supply
msg::sender()

// Evaluate expressions
> contract.balance_of(address)
> contract.allowance(owner, spender)
```

### 3. Call Stack Navigation

```rust
// View call stack
contract.transfer()
  ├── check_balance()
  ├── update_balances()
  └── emit_event()
```

## Testing Integration

### 1. Debug Tests

```rust
#[test]
#[debug_test]
fn test_transfer() {
    // Automatically breaks on test failure
    let result = contract.transfer(recipient, 1000);
    assert!(result.is_ok());
}
```

### 2. Test Coverage

```rust
#[coverage]
fn analyze_coverage() {
    let coverage = CoverageAnalyzer::new();
    
    // Run tests with coverage
    coverage.run_tests();
    
    // Generate report
    coverage.generate_report("coverage.html");
}
```

## Best Practices

1. **Logging Strategy**
   - Use appropriate log levels
   - Include relevant context
   - Avoid sensitive information

2. **Debugging Setup**
   - Configure source maps
   - Set up proper watch expressions
   - Use conditional breakpoints

3. **Performance**
   - Profile gas usage regularly
   - Monitor memory consumption
   - Track state changes

## Common Issues and Solutions

1. **Contract Deployment**
   ```bash
   # Debug deployment
   strm deploy --debug contracts/Token.strx
   
   # View deployment logs
   strm logs --deployment latest
   ```

2. **Transaction Issues**
   ```bash
   # Trace transaction
   strm trace tx 0x123...
   
   # View transaction state changes
   strm state-diff tx 0x123...
   ```
