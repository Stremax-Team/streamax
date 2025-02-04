# API Reference

This document provides a comprehensive reference for the Stremax module system API.

## Core Types

### ModuleId

```rust
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModuleId(pub String);
```

Unique identifier for modules.

### Symbol

```rust
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Symbol(pub String);
```

Represents a symbol name within a module.

### Module

```rust
pub struct Module {
    pub fn new(id: ModuleId, path: PathBuf) -> Self;
    pub fn with_source(id: ModuleId, path: PathBuf, source: String) -> Self;
    
    // Accessors
    pub fn id(&self) -> &ModuleId;
    pub fn path(&self) -> &Path;
    pub fn source(&self) -> Option<&str>;
    pub fn version(&self) -> Option<&str>;
    
    // Exports
    pub fn add_export(&mut self, name: Symbol, export: Export);
    pub fn get_export(&self, name: &Symbol) -> Option<&Export>;
    pub fn exports(&self) -> impl Iterator<Item = (&Symbol, &Export)>;
    
    // Imports
    pub fn add_import(&mut self, import: Import);
    pub fn imports(&self) -> impl Iterator<Item = &Import>;
    
    // Dependencies
    pub fn add_dependency(&mut self, module_id: ModuleId);
    pub fn dependencies(&self) -> impl Iterator<Item = &ModuleId>;
    
    // Cache operations
    pub fn clear_cache(&mut self);
}
```

### Export

```rust
#[derive(Debug, Clone)]
pub struct Export {
    pub name: Symbol,
    pub visibility: Visibility,
    pub kind: ExportKind,
    pub docs: Option<String>,
    pub deprecated: bool,
}

#[derive(Debug, Clone)]
pub enum ExportKind {
    Function(FunctionExport),
    Type(TypeExport),
    Constant(ConstantExport),
    Module(ModuleExport),
}
```

### Import

```rust
#[derive(Debug, Clone)]
pub struct Import {
    pub module: ModuleId,
    pub name: Symbol,
    pub alias: Option<Symbol>,
    pub visibility: Visibility,
    pub is_reexport: bool,
}
```

## Module Loading

### ConcurrentModuleLoader

```rust
pub struct ConcurrentModuleLoader {
    pub fn new() -> Self;
    pub fn with_validator(validator: Box<dyn ModuleValidator>) -> Self;
    
    // Path management
    pub fn add_search_path<P: Into<PathBuf>>(&self, path: P);
    pub fn get_search_paths(&self) -> Vec<PathBuf>;
    
    // Module operations
    pub async fn load_module(&self, id: ModuleId) -> Result<Arc<RwLock<Module>>>;
    pub async fn unload_module(&self, id: &ModuleId) -> Result<()>;
    pub fn get_module(&self, id: &ModuleId) -> Option<Arc<RwLock<Module>>>;
    
    // Cache operations
    pub fn clear_cache(&self);
    pub fn get_loaded_modules(&self) -> Vec<Arc<RwLock<Module>>>;
}
```

## Caching

### OptimizedModuleCache

```rust
pub struct OptimizedModuleCache {
    pub fn new(max_size: usize) -> Self;
    pub fn with_sizes(
        symbol_size: usize,
        type_size: usize,
        function_size: usize,
        constant_size: usize,
    ) -> Self;
    
    // Cache operations
    pub fn get_symbol(&self, name: &Symbol) -> Option<Arc<SymbolData>>;
    pub fn cache_symbol(&self, name: Symbol, data: SymbolData);
    pub fn remove_symbol(&self, name: &Symbol);
    
    // Statistics
    pub fn get_stats(&self) -> CacheStats;
    pub fn clear(&self);
}
```

## Validation

### ModuleValidator

```rust
pub trait ModuleValidator: Send + Sync {
    fn validate(&self, module: &Module) -> Result<()>;
}

pub struct DefaultModuleValidator {
    pub fn new() -> Self;
    pub fn with_max_dependencies(self, max: usize) -> Self;
    pub fn with_max_exports(self, max: usize) -> Self;
    pub fn require_export(self, symbol: Symbol) -> Self;
}
```

## Hot Reloading

### HotReloadManager

```rust
pub struct HotReloadManager {
    pub fn new(loader: Arc<ConcurrentModuleLoader>) 
        -> Result<(Self, broadcast::Receiver<ModuleChangeEvent>)>;
    
    // Watching operations
    pub fn watch_module(&self, module: &Module) -> Result<()>;
    pub fn unwatch_module(&self, module: &Module) -> Result<()>;
    
    // Update handling
    pub async fn check_updates(&self) -> Result<Vec<ModuleChangeEvent>>;
    pub fn subscribe(&self) -> broadcast::Receiver<ModuleChangeEvent>;
    
    // Control
    pub async fn start_watching(&self) -> Result<()>;
    pub async fn stop_watching(&self) -> Result<()>;
}
```

## Error Handling

### ModuleError

```rust
#[derive(Debug, Clone)]
pub enum ModuleError {
    NotFound(ModuleId),
    ParseError { module: ModuleId, message: String },
    CircularDependency { path: Vec<ModuleId> },
    InvalidSymbol { module: ModuleId, symbol: Symbol, reason: String },
    VisibilityViolation { module: ModuleId, symbol: Symbol, required: String, actual: String },
    TypeError { module: ModuleId, message: String },
    IoError { module: Option<ModuleId>, source: std::io::Error },
    VersionMismatch { module: ModuleId, required: String, actual: String },
    CacheError { module: Option<ModuleId>, message: String },
    ValidationError { module: ModuleId, message: String },
}
```

## Events

### ModuleChangeEvent

```rust
#[derive(Debug, Clone)]
pub enum ModuleChangeEvent {
    Created(ModuleId),
    Modified(ModuleId),
    Removed(ModuleId),
    Reloaded(ModuleId),
    Error { module: ModuleId, error: Arc<Error> },
}
```

## Helper Types

### SourceLocation

```rust
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
```

### Visibility

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}
```

## Common Patterns

### Loading a Module

```rust
let loader = Arc::new(ConcurrentModuleLoader::new());
loader.add_search_path("modules");

let module_id = ModuleId("my_module".to_string());
let module = loader.load_module(module_id).await?;
```

### Using Hot Reload

```rust
let (hot_reload, mut rx) = HotReloadManager::new(loader.clone())?;
hot_reload.start_watching().await?;

tokio::spawn(async move {
    while let Ok(event) = rx.recv().await {
        match event {
            ModuleChangeEvent::Modified(id) => println!("Module {} modified", id.0),
            ModuleChangeEvent::Reloaded(id) => println!("Module {} reloaded", id.0),
            // Handle other events
        }
    }
});
```


### Custom Validation

```rust
struct MyValidator {
    allowed_extensions: HashSet<String>,
}

impl ModuleValidator for MyValidator {
    fn validate(&self, module: &Module) -> Result<()> {
        // Ensure module has .strx extension
        if !module.path().to_str()
            .map(|p| p.ends_with(".strx"))
            .unwrap_or(false) 
        {
            return Err(ModuleError::ValidationError {
                module: module.id().clone(),
                message: "Module must have .strx extension".to_string(),
            });
        }
        Ok(())
    }
}

let mut validator = MyValidator {
    allowed_extensions: HashSet::from_iter(vec![".strx".to_string()]),
};
let loader = ConcurrentModuleLoader::with_validator(Box::new(validator));
```
``` 