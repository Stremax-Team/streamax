pub mod loader;
pub mod resolver;
pub mod cache;
pub mod types;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::core::{Error, Result, ModuleId, Symbol};

pub use self::loader::ModuleLoader;
pub use self::resolver::ModuleResolver;
pub use self::cache::ModuleCache;
pub use self::types::*;

/// Module representation
#[derive(Debug)]
pub struct Module {
    id: ModuleId,
    path: PathBuf,
    source: Option<String>,
    exports: HashMap<Symbol, Export>,
    imports: Vec<Import>,
    dependencies: Vec<ModuleId>,
    visibility: HashMap<Symbol, Visibility>,
    cache: ModuleCache,
}

impl Module {
    pub fn new(id: ModuleId, path: PathBuf) -> Self {
        Module {
            id,
            path,
            source: None,
            exports: HashMap::new(),
            imports: Vec::new(),
            dependencies: Vec::new(),
            visibility: HashMap::new(),
            cache: ModuleCache::new(),
        }
    }
    
    pub fn with_source(id: ModuleId, path: PathBuf, source: String) -> Self {
        let mut module = Self::new(id, path);
        module.source = Some(source);
        module
    }
    
    pub fn id(&self) -> &ModuleId {
        &self.id
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    
    pub fn add_export(&mut self, name: Symbol, export: Export) {
        self.exports.insert(name.clone(), export);
        self.visibility.insert(name, Visibility::Public);
    }
    
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }
    
    pub fn add_dependency(&mut self, module_id: ModuleId) {
        if !self.dependencies.contains(&module_id) {
            self.dependencies.push(module_id);
        }
    }
    
    pub fn get_export(&self, name: &Symbol) -> Option<&Export> {
        self.exports.get(name)
    }
    
    pub fn get_symbol(&self, name: &Symbol) -> Option<Arc<SymbolData>> {
        self.cache.get_symbol(name)
    }
    
    pub fn get_type(&self, name: &Symbol) -> Option<Arc<TypeData>> {
        self.cache.get_type(name)
    }
    
    pub fn get_function(&self, name: &Symbol) -> Option<Arc<FunctionData>> {
        self.cache.get_function(name)
    }
    
    pub fn get_constant(&self, name: &Symbol) -> Option<Arc<ConstantData>> {
        self.cache.get_constant(name)
    }
    
    pub fn exports(&self) -> impl Iterator<Item = (&Symbol, &Export)> {
        self.exports.iter()
    }
    
    pub fn imports(&self) -> impl Iterator<Item = &Import> {
        self.imports.iter()
    }
    
    pub fn dependencies(&self) -> impl Iterator<Item = &ModuleId> {
        self.dependencies.iter()
    }
    
    pub fn get_visibility(&self, name: &Symbol) -> Visibility {
        self.visibility.get(name).copied().unwrap_or(Visibility::Private)
    }
    
    pub fn set_visibility(&mut self, name: Symbol, visibility: Visibility) {
        self.visibility.insert(name, visibility);
    }
    
    pub fn cache_symbol(&mut self, name: Symbol, data: SymbolData) {
        self.cache.cache_symbol(name, data);
    }
    
    pub fn cache_type(&mut self, name: Symbol, data: TypeData) {
        self.cache.cache_type(name, data);
    }
    
    pub fn cache_function(&mut self, name: Symbol, data: FunctionData) {
        self.cache.cache_function(name, data);
    }
    
    pub fn cache_constant(&mut self, name: Symbol, data: ConstantData) {
        self.cache.cache_constant(name, data);
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}