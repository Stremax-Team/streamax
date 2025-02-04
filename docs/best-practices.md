# Best Practices Guide

This guide outlines recommended practices for using the Stremax module system effectively.

## Module Organization

### 1. Directory Structure

```
modules/
├── core/
│   ├── types.strx
│   ├── error.strx
│   └── utils.strx
├── std/
│   ├── collections.strx
│   ├── io.strx
│   └── net.strx
└── app/
    ├── config.strx
    └── main.strx
```

### 2. Module Naming

- Use lowercase names with underscores
- Be descriptive but concise
- Follow a consistent naming pattern

```rust
// Good
ModuleId("core_types")
ModuleId("std_collections")
ModuleId("app_config")

// Bad
ModuleId("CoreTypes")  // Not lowercase
ModuleId("t")         // Too short
ModuleId("my_really_long_module_name_that_does_something")  // Too long
```

## Error Handling

### 1. Use Proper Error Types

```rust
// Good
fn load_config() -> Result<Config> {
    let data = std::fs::read_to_string("config.strx")
        .map_err(|e| ModuleError::IoError {
            module: Some(self.id().clone()),
            source: e,
        })?;
    // ...
}

// Bad
fn load_config() -> Result<Config> {
    let data = std::fs::read_to_string("config.strx")
        .map_err(|_| ModuleError::ParseError {  // Lost error context
            module: self.id().clone(),
            message: "Failed to load config".to_string(),
        })?;
    // ...
}
```

### 2. Error Recovery

```rust
// Good
match loader.load_module(id).await {
    Ok(module) => module,
    Err(ModuleError::NotFound(_)) => {
        log::warn!("Module not found, using fallback");
        create_fallback_module(id)
    }
    Err(e) => return Err(e),
}

// Bad
loader.load_module(id).await.unwrap()  // May panic
```

## Concurrency

### 1. Lock Management

```rust
// Good
{
    let modules = self.modules.read();
    if let Some(module) = modules.get(&id) {
        return Ok(module.clone());
    }
}  // Release read lock before write

let mut modules = self.modules.write();
// ...

// Bad
let modules = self.modules.read();
let mut modules_write = self.modules.write();  // Potential deadlock
```

### 2. Resource Cleanup

```rust
// Good
impl Drop for Module {
    fn drop(&mut self) {
        self.clear_cache();
        if let Err(e) = self.cleanup_resources() {
            log::error!("Failed to cleanup module resources: {}", e);
        }
    }
}

// Bad
impl Module {
    fn unload(&mut self) {
        // Manual cleanup without Drop implementation
        // May be forgotten or skipped on panic
    }
}
```

## Caching

### 1. Cache Size Management

```rust
// Good
let cache = OptimizedModuleCache::with_sizes(
    symbol_size: estimate_symbol_cache_size(),
    type_size: estimate_type_cache_size(),
    function_size: estimate_function_cache_size(),
    constant_size: estimate_constant_cache_size(),
);

// Bad
let cache = OptimizedModuleCache::new(10000);  // Arbitrary size
```

### 2. Cache Monitoring

```rust
// Good
fn monitor_cache_performance(cache: &OptimizedModuleCache) {
    let stats = cache.get_stats();
    if stats.hit_rate() < 0.5 {
        log::warn!("Low cache hit rate: {:.2}%", stats.hit_rate() * 100.0);
        if stats.evictions > 1000 {
            cache.resize(cache.max_size() * 2);
        }
    }
}

// Bad
// No monitoring, no adaptation
```

## Module Loading

### 1. Search Paths

```rust
// Good
loader.add_search_path(env::var("MODULE_PATH")?);
loader.add_search_path("./modules");
loader.add_search_path("~/.stremax/modules");

// Bad
loader.add_search_path(".");  // Too broad
```

### 2. Dependency Management

```rust
// Good
fn load_with_dependencies(loader: &ConcurrentModuleLoader, id: ModuleId) -> Result<()> {
    let module = loader.load_module(id).await?;
    
    // Load dependencies in parallel
    let deps: Vec<_> = module.read().dependencies().cloned().collect();
    join_all(deps.into_iter().map(|dep_id| loader.load_module(dep_id))).await?;
    
    Ok(())
}

// Bad
// Loading dependencies sequentially
for dep_id in module.dependencies() {
    loader.load_module(dep_id).await?;
}
```

## Hot Reloading

### 1. State Preservation

```rust
// Good
struct ModuleState {
    version: u32,
    config: Arc<Config>,
    cache: Arc<Cache>,
}

impl Module {
    fn preserve_state(&self) -> ModuleState {
        // Save important state
    }
    
    fn restore_state(&mut self, state: ModuleState) {
        // Restore state after reload
    }
}

// Bad
// No state preservation during reload
```

### 2. Change Detection

```rust
// Good
impl HotReloadManager {
    fn should_reload(&self, module: &Module, new_content: &str) -> bool {
        if let Some(old_content) = module.source() {
            // Compare content hashes
            hash(old_content) != hash(new_content)
        } else {
            true
        }
    }
}

// Bad
// Always reload on any file change
```

## Validation

### 1. Progressive Validation

```rust
// Good
impl ModuleValidator {
    fn validate(&self, module: &Module) -> Result<()> {
        // Basic checks first
        self.validate_syntax(module)?;
        
        // Then structural checks
        self.validate_exports(module)?;
        self.validate_imports(module)?;
        
        // Finally semantic checks
        self.validate_types(module)?;
        self.validate_dependencies(module)?;
        
        Ok(())
    }
}

// Bad
// All validation at once
```

### 2. Custom Validators

```rust
// Good
pub struct SecurityValidator {
    allowed_unsafe: bool,
    trusted_modules: HashSet<ModuleId>,
}

impl ModuleValidator for SecurityValidator {
    fn validate(&self, module: &Module) -> Result<()> {
        if !self.allowed_unsafe {
            self.check_unsafe_usage(module)?;
        }
        self.verify_signatures(module)?;
        Ok(())
    }
}

// Bad
// Security checks mixed with other validation
```

## Performance

### 1. Memory Management

```rust
// Good
impl Module {
    fn load_large_resource(&self) -> Result<Arc<Resource>> {
        let resource = Arc::new(Resource::load()?);
        self.resource_cache.write().insert(resource.clone());
        Ok(resource)
    }
}

// Bad
// Loading large resources without caching or sharing
```

### 2. Async Operations

```rust
// Good
impl Module {
    async fn load_resources(&self) -> Result<()> {
        let futures: Vec<_> = self.resource_paths()
            .into_iter()
            .map(|path| self.load_resource(path))
            .collect();
            
        try_join_all(futures).await?;
        Ok(())
    }
}

// Bad
// Loading resources sequentially
``` 