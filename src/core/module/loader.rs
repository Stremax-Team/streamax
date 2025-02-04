use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::core::{Error, Result, ModuleId, Symbol};
use super::{Module, ModuleCache};
use super::types::*;

/// Module hook for customizing module loading behavior
pub trait ModuleHook: Send + Sync {
    fn on_load(&self, module: &mut Module) -> Result<()>;
    fn on_unload(&self, module: &Module) -> Result<()>;
    fn on_resolve(&self, name: &Symbol, module: &Module) -> Option<Arc<SymbolData>>;
}

/// Module loader for loading and linking modules
pub struct ModuleLoader {
    modules: HashMap<ModuleId, Arc<Module>>,
    search_paths: Vec<PathBuf>,
    cache: ModuleCache,
    hooks: Vec<Box<dyn ModuleHook>>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        ModuleLoader {
            modules: HashMap::new(),
            search_paths: Vec::new(),
            cache: ModuleCache::new(),
            hooks: Vec::new(),
        }
    }
    
    pub fn add_search_path<P: Into<PathBuf>>(&mut self, path: P) {
        self.search_paths.push(path.into());
    }
    
    pub fn add_hook<H: ModuleHook + 'static>(&mut self, hook: H) {
        self.hooks.push(Box::new(hook));
    }
    
    pub fn load_module(&mut self, id: ModuleId) -> Result<Arc<Module>> {
        // Check cache first
        if let Some(module) = self.modules.get(&id) {
            return Ok(module.clone());
        }
        
        // Find and parse module
        let path = self.find_module(&id)?;
        let source = std::fs::read_to_string(&path)?;
        let mut module = Module::with_source(id.clone(), path, source);
        
        // Run hooks
        for hook in &self.hooks {
            hook.on_load(&mut module)?;
        }
        
        // Load dependencies
        let deps: Vec<_> = module.dependencies().cloned().collect();
        for dep_id in deps {
            let dep = self.load_module(dep_id)?;
            module.add_dependency(dep.id().clone());
        }
        
        let module = Arc::new(module);
        self.modules.insert(id, module.clone());
        Ok(module)
    }
    
    pub fn unload_module(&mut self, id: &ModuleId) -> Result<()> {
        if let Some(module) = self.modules.remove(id) {
            for hook in &self.hooks {
                hook.on_unload(&module)?;
            }
        }
        Ok(())
    }
    
    pub fn get_module(&self, id: &ModuleId) -> Option<Arc<Module>> {
        self.modules.get(id).cloned()
    }
    
    pub fn resolve_symbol(&self, module: &Module, name: &Symbol) -> Result<Arc<SymbolData>> {
        // Check hooks first
        for hook in &self.hooks {
            if let Some(data) = hook.on_resolve(name, module) {
                return Ok(data);
            }
        }
        
        // Check module cache
        if let Some(data) = module.get_symbol(name) {
            return Ok(data);
        }
        
        // Check imports
        for import in module.imports() {
            if import.name == *name {
                if let Some(module) = self.get_module(&import.module) {
                    if let Some(data) = module.get_symbol(&import.name) {
                        return Ok(data);
                    }
                }
            }
        }
        
        Err(Error::SymbolNotFound(name.0.clone()))
    }
    
    fn find_module(&self, id: &ModuleId) -> Result<PathBuf> {
        for search_path in &self.search_paths {
            let module_path = search_path.join(format!("{}.strm", id.0));
            if module_path.exists() {
                return Ok(module_path);
            }
        }
        Err(Error::ModuleNotFound(id.0.clone()))
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        for module in self.modules.values() {
            // Can't modify Arc<Module> directly, so this is a limitation
            // In practice, we might want to use Arc<RwLock<Module>> instead
            // module.clear_cache();
        }
    }
    
    pub fn get_loaded_modules(&self) -> impl Iterator<Item = &Arc<Module>> {
        self.modules.values()
    }
    
    pub fn get_search_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.search_paths.iter()
    }
}