use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::sync::Semaphore;
use futures::future::join_all;
use crate::core::{Error, Result, ModuleId, Symbol};
use super::{Module, ModuleCache, ModuleValidator, DefaultModuleValidator};
use super::types::*;

/// Concurrent module loader for parallel loading and processing of modules
pub struct ConcurrentModuleLoader {
    modules: RwLock<HashMap<ModuleId, Arc<RwLock<Module>>>>,
    search_paths: RwLock<Vec<PathBuf>>,
    cache: RwLock<ModuleCache>,
    validator: Box<dyn ModuleValidator + Send + Sync>,
    max_concurrent_loads: Semaphore,
}

impl ConcurrentModuleLoader {
    pub fn new() -> Self {
        Self::with_validator(Box::new(DefaultModuleValidator::new()))
    }

    pub fn with_validator(validator: Box<dyn ModuleValidator + Send + Sync>) -> Self {
        ConcurrentModuleLoader {
            modules: RwLock::new(HashMap::new()),
            search_paths: RwLock::new(Vec::new()),
            cache: RwLock::new(ModuleCache::new()),
            validator,
            max_concurrent_loads: Semaphore::new(32), // Default to 32 concurrent loads
        }
    }

    pub fn add_search_path<P: Into<PathBuf>>(&self, path: P) {
        self.search_paths.write().push(path.into());
    }

    pub async fn load_module(&self, id: ModuleId) -> Result<Arc<RwLock<Module>>> {
        // First check if module is already loaded
        if let Some(module) = self.modules.read().get(&id) {
            return Ok(module.clone());
        }

        // Acquire semaphore permit for concurrent load limiting
        let _permit = self.max_concurrent_loads.acquire().await;

        // Double check after acquiring permit
        if let Some(module) = self.modules.read().get(&id) {
            return Ok(module.clone());
        }

        // Find and parse module
        let path = self.find_module(&id)?;
        let source = tokio::fs::read_to_string(&path).await
            .map_err(|e| ModuleError::IoError { 
                module: Some(id.clone()), 
                source: e 
            })?;

        let mut module = Module::with_source(id.clone(), path, source);

        // Validate module
        self.validator.validate(&module)?;

        // Load dependencies concurrently
        let deps: Vec<_> = module.dependencies().cloned().collect();
        let mut dep_futures = Vec::new();
        
        for dep_id in deps {
            dep_futures.push(self.load_module(dep_id));
        }

        let dep_results = join_all(dep_futures).await;
        for result in dep_results {
            let dep = result?;
            module.add_dependency(dep.read().id().clone());
        }

        let module = Arc::new(RwLock::new(module));
        self.modules.write().insert(id, module.clone());
        
        Ok(module)
    }

    pub async fn unload_module(&self, id: &ModuleId) -> Result<()> {
        let mut modules = self.modules.write();
        if let Some(module) = modules.remove(id) {
            // Validate no other modules depend on this one
            for other_module in modules.values() {
                let other = other_module.read();
                if other.dependencies().any(|dep_id| dep_id == id) {
                    return Err(ModuleError::ValidationError {
                        module: other.id().clone(),
                        message: format!("Module {} still depends on {}", other.id().0, id.0),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn get_module(&self, id: &ModuleId) -> Option<Arc<RwLock<Module>>> {
        self.modules.read().get(id).cloned()
    }

    pub async fn resolve_symbol(&self, module: &Module, name: &Symbol) -> Result<Arc<SymbolData>> {
        // Check module cache first
        if let Some(data) = module.get_symbol(name) {
            return Ok(data);
        }

        // Check imports
        for import in module.imports() {
            if import.name == *name {
                if let Some(module) = self.get_module(&import.module) {
                    let module = module.read();
                    if let Some(data) = module.get_symbol(&import.name) {
                        return Ok(data);
                    }
                }
            }
        }

        Err(ModuleError::SymbolNotFound(name.0.clone()))
    }

    fn find_module(&self, id: &ModuleId) -> Result<PathBuf> {
        let search_paths = self.search_paths.read();
        for search_path in search_paths.iter() {
            let module_path = search_path.join(format!("{}.strm", id.0));
            if module_path.exists() {
                return Ok(module_path);
            }
        }
        Err(ModuleError::NotFound(id.clone()))
    }

    pub async fn reload_module(&self, id: &ModuleId) -> Result<()> {
        // Remove from cache
        self.modules.write().remove(id);
        
        // Reload
        self.load_module(id.clone()).await?;
        Ok(())
    }

    pub fn clear_cache(&self) {
        self.cache.write().clear();
        let modules = self.modules.read();
        for module in modules.values() {
            module.write().clear_cache();
        }
    }

    pub fn get_loaded_modules(&self) -> Vec<Arc<RwLock<Module>>> {
        self.modules.read().values().cloned().collect()
    }

    pub fn get_search_paths(&self) -> Vec<PathBuf> {
        self.search_paths.read().clone()
    }
}