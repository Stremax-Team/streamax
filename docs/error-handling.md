# Error Handling

The Stremax module system implements a comprehensive error handling system that provides detailed error information and context.

## Error Types

### ModuleError Enum

The core error type is `ModuleError`, which covers all possible error cases:

```rust
pub enum ModuleError {
    /// Module not found in search paths
    NotFound(ModuleId),
    
    /// Error parsing module contents
    ParseError {
        module: ModuleId,
        message: String,
    },
    
    /// Circular dependency detected
    CircularDependency {
        path: Vec<ModuleId>,
    },
    
    /// Invalid or undefined symbol
    InvalidSymbol {
        module: ModuleId,
        symbol: Symbol,
        reason: String,
    },
    
    /// Visibility violation
    VisibilityViolation {
        module: ModuleId,
        symbol: Symbol,
        required: String,
        actual: String,
    },
    
    /// Type error
    TypeError {
        module: ModuleId,
        message: String,
    },
    
    /// IO error during module operations
    IoError {
        module: Option<ModuleId>,
        source: std::io::Error,
    },
    
    /// Version mismatch between modules
    VersionMismatch {
        module: ModuleId,
        required: String,
        actual: String,
    },
    
    /// Cache error
    CacheError {
        module: Option<ModuleId>,
        message: String,
    },
    
    /// Validation error
    ValidationError {
        module: ModuleId,
        message: String,
    },
}
```

## Error Handling Patterns

### 1. Using Result Type

The module system uses a custom `Result` type alias:

```rust
pub type Result<T> = std::result::Result<T, ModuleError>;
```

Example usage:

```rust
async fn load_module(&self, id: ModuleId) -> Result<Arc<RwLock<Module>>> {
    if let Some(module) = self.modules.read().get(&id) {
        return Ok(module.clone());
    }
    
    Err(ModuleError::NotFound(id))
}
```

### 2. Error Macros

Helper macros for common error handling patterns:

```rust
// Create a module error
macro_rules! module_error {
    ($kind:ident, $($arg:expr),*) => {
        Err(ModuleError::$kind($($arg),*))
    };
}

// Check condition and return error if false
macro_rules! ensure_module {
    ($cond:expr, $kind:ident, $($arg:expr),*) => {
        if !$cond {
            return module_error!($kind, $($arg),*);
        }
    };
}
```

Example usage:

```rust
fn validate_export(&self, export: &Export) -> Result<()> {
    ensure_module!(
        export.visibility != Visibility::Private || !export.deprecated,
        ValidationError,
        self.id().clone(),
        "Private export cannot be deprecated".to_string()
    );
    Ok(())
}
```

## Error Context

Errors include detailed context information:

1. **Module Identity**: Most errors include the `ModuleId` of the module where the error occurred
2. **Symbol Information**: Symbol-related errors include the symbol name and context
3. **Chain of Events**: Dependency errors include the full path of modules involved
4. **Detailed Messages**: Human-readable error messages with specific details

Example:

```rust
match error {
    ModuleError::CircularDependency { path } => {
        println!("Circular dependency detected:");
        for module in path {
            println!("  -> {}", module.0);
        }
    }
    ModuleError::VisibilityViolation { module, symbol, required, actual } => {
        println!("Visibility violation in module {}:", module.0);
        println!("Symbol: {}", symbol.0);
        println!("Required: {}", required);
        println!("Actual: {}", actual);
    }
    // ...
}
```

## Error Recovery

### 1. Cache Recovery

```rust
impl OptimizedModuleCache {
    pub fn recover_from_error(&self, error: &ModuleError) -> Result<()> {
        match error {
            ModuleError::CacheError { module, .. } => {
                if let Some(module) = module {
                    // Clear cache entries for the affected module
                    self.remove_module_entries(module);
                } else {
                    // Clear entire cache in case of serious errors
                    self.clear();
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
```

### 2. Module Recovery

```rust
impl Module {
    pub fn recover_from_error(&mut self, error: &ModuleError) -> Result<()> {
        match error {
            ModuleError::ParseError { .. } => {
                // Reset module state
                self.clear_cache();
                self.exports.clear();
                self.imports.clear();
                Ok(())
            }
            ModuleError::CircularDependency { path } => {
                // Remove problematic dependencies
                for module_id in path {
                    self.remove_dependency(module_id);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
```

## Best Practices

1. **Always Include Context**:
   ```rust
   Err(ModuleError::ParseError {
       module: module_id,
       message: format!("Failed to parse at line {}: {}", line, error),
   })
   ```

2. **Use Error Chaining**:
   ```rust
   fn load_module(&self, id: ModuleId) -> Result<Module> {
       let source = std::fs::read_to_string(&self.path)
           .map_err(|e| ModuleError::IoError {
               module: Some(id.clone()),
               source: e,
           })?;
       // ...
   }
   ```

3. **Provide Recovery Options**:
   ```rust
   match loader.load_module(id) {
       Ok(module) => module,
       Err(e) => {
           eprintln!("Error loading module: {}", e);
           if let Some(cached) = loader.get_cached_module(id) {
               println!("Using cached version");
               cached
           } else {
               return Err(e);
           }
       }
   }
   ```

4. **Log Error Details**:
   ```rust
   fn handle_error(error: &ModuleError) {
       match error {
           ModuleError::NotFound(id) => {
               log::error!("Module not found: {}", id.0);
               log::debug!("Search paths: {:?}", loader.get_search_paths());
           }
           // ...
       }
   }
   ``` 