# Stremax Module System Documentation

This documentation provides a comprehensive guide to the Stremax module system, a robust and flexible module management system implemented in Rust.

## Table of Contents

1. [Overview](overview.md)
2. [Architecture](architecture.md)
3. [Core Components](components.md)
4. [Error Handling](error-handling.md)
5. [Module Validation](validation.md)
6. [Caching System](caching.md)
7. [Hot Reloading](hot-reloading.md)
8. [Concurrency](concurrency.md)
9. [Best Practices](best-practices.md)
10. [API Reference](api-reference.md)

## Quick Start

```rust
use stremax::core::module::{Module, ModuleId, ConcurrentModuleLoader};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a concurrent module loader
    let loader = Arc::new(ConcurrentModuleLoader::new());
    
    // Add search paths
    loader.add_search_path("modules");
    
    // Load a module
    let module_id = ModuleId("my_module".to_string());
    let module = loader.load_module(module_id).await.unwrap();
    
    // Access module contents
    let module = module.read();
    println!("Loaded module: {}", module.id());
}
```

## Features

- **Robust Error Handling**: Comprehensive error types and handling mechanisms
- **Module Validation**: Configurable validation rules and dependency checking
- **Concurrent Loading**: Thread-safe parallel module loading
- **Optimized Caching**: LRU cache with performance metrics
- **Hot Reloading**: File system watching and automatic module reloading
- **Type Safety**: Strong typing throughout the system
- **Extensible**: Plugin system through module hooks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
stremax = "0.1.0"
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details. 