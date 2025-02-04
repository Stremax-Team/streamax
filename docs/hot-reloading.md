 # Hot Reloading

The Stremax module system provides a robust hot reloading mechanism that allows modules to be updated at runtime without requiring application restart.

## Overview

The hot reloading system consists of:
1. File system watching
2. Change detection
3. Module reloading
4. Event notification
5. State management

## Components

### ModuleChangeEvent

Events emitted when modules change:

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

### HotReloadManager

The main hot reload management component:

```rust
pub struct HotReloadManager {
    loader: Arc<ConcurrentModuleLoader>,
    watcher: notify::RecommendedWatcher,
    file_times: RwLock<HashMap<PathBuf, SystemTime>>,
    tx: broadcast::Sender<ModuleChangeEvent>,
    watched_paths: RwLock<HashMap<PathBuf, ModuleId>>,
}
```

## Usage

### Basic Setup

```rust
use stremax::core::module::{ConcurrentModuleLoader, HotReloadManager};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create loader
    let loader = Arc::new(ConcurrentModuleLoader::new());
    
    // Create hot reload manager
    let (hot_reload, mut rx) = HotReloadManager::new(loader.clone())?;
    
    // Start watching for changes
    hot_reload.start_watching().await?;
    
    // Handle events
    tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            match event {
                ModuleChangeEvent::Modified(id) => {
                    println!("Module {} was modified", id.0);
                }
                ModuleChangeEvent::Reloaded(id) => {
                    println!("Module {} was reloaded", id.0);
                }
                // ...
            }
        }
    });
}
```

### Watching Specific Modules

```rust
// Load a module
let module = loader.load_module(module_id).await?;

// Start watching it
hot_reload.watch_module(&module.read())?;

// Stop watching when done
hot_reload.unwatch_module(&module.read())?;
```

### Manual Update Checking

```rust
// Check for updates
let events = hot_reload.check_updates().await?;

for event in events {
    match event {
        ModuleChangeEvent::Reloaded(id) => {
            println!("Module {} was reloaded", id.0);
            // Handle reload
        }
        ModuleChangeEvent::Error { module, error } => {
            eprintln!("Error reloading module {}: {}", module.0, error);
            // Handle error
        }
        // ...
    }
}
```

## State Management

### Handling State During Reloads

```rust
// Store module state before reload
let state = module.read().get_state();

// Subscribe to reload events
let mut rx = hot_reload.subscribe();

tokio::spawn(async move {
    while let Ok(event) = rx.recv().await {
        if let ModuleChangeEvent::Reloaded(id) = event {
            if let Some(module) = loader.get_module(&id) {
                let mut module = module.write();
                // Restore state after reload
                module.restore_state(state.clone());
            }
        }
    }
});
```

### Preserving References

```rust
// Use Arc<RwLock<Module>> to maintain references across reloads
let module = Arc::new(RwLock::new(module));

// References remain valid after reload
let module_ref = module.clone();
tokio::spawn(async move {
    // Module can be safely used here even after reloads
    let data = module_ref.read().get_data();
});
```

## Best Practices

### 1. Graceful Handling of Reload Failures

```rust
impl HotReloadManager {
    async fn reload_module(&self, id: &ModuleId) -> Result<()> {
        match self.loader.reload_module(id).await {
            Ok(()) => Ok(()),
            Err(e) => {
                // Log error
                log::error!("Failed to reload module {}: {}", id.0, e);
                
                // Try to recover
                if let Some(old_module) = self.loader.get_module(id) {
                    log::info!("Keeping old module version");
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }
}
```

### 2. Debouncing Changes

```rust
use tokio::time::{Duration, Instant};

impl HotReloadManager {
    async fn handle_change(&self, path: &Path, last_change: &mut Instant) {
        let now = Instant::now();
        if now.duration_since(*last_change) < Duration::from_millis(100) {
            return; // Debounce rapid changes
        }
        *last_change = now;
        
        if let Some(module_id) = self.path_to_module_id(path) {
            self.reload_module(&module_id).await?;
        }
    }
}
```

### 3. Versioning Support

```rust
#[derive(Clone, Debug)]
struct ModuleVersion {
    id: ModuleId,
    version: semver::Version,
    timestamp: SystemTime,
}

impl HotReloadManager {
    fn should_reload(&self, module: &Module, new_version: &ModuleVersion) -> bool {
        if let Some(current_version) = module.version() {
            new_version.version > current_version
        } else {
            true
        }
    }
}
```

### 4. Resource Cleanup

```rust
impl Drop for HotReloadManager {
    fn drop(&mut self) {
        // Stop all file watchers
        let paths: Vec<_> = self.watched_paths.read().keys().cloned().collect();
        for path in paths {
            let _ = self.watcher.unwatch(&path);
        }
        
        // Clear internal state
        self.watched_paths.write().clear();
        self.file_times.write().clear();
    }
}
```

## Performance Considerations

1. **File System Watching**:
   - Use non-recursive watching when possible
   - Implement debouncing for rapid changes
   - Limit the number of watched files

2. **Memory Management**:
   - Clear old module versions after reload
   - Implement cache eviction for unused modules
   - Use weak references where appropriate

3. **Concurrency**:
   - Use bounded channels for event broadcasting
   - Implement timeouts for reload operations
   - Handle concurrent reload requests safely
   