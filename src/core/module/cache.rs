use std::collections::HashMap;
use std::sync::Arc;
use crate::core::Symbol;
use super::types::*;

/// Module cache for storing resolved symbols
#[derive(Debug, Default)]
pub struct ModuleCache {
    symbols: HashMap<Symbol, Arc<SymbolData>>,
    types: HashMap<Symbol, Arc<TypeData>>,
    functions: HashMap<Symbol, Arc<FunctionData>>,
    constants: HashMap<Symbol, Arc<ConstantData>>,
}

impl ModuleCache {
    pub fn new() -> Self {
        ModuleCache {
            symbols: HashMap::new(),
            types: HashMap::new(),
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }
    
    pub fn get_symbol(&self, name: &Symbol) -> Option<Arc<SymbolData>> {
        self.symbols.get(name).cloned()
    }
    
    pub fn get_type(&self, name: &Symbol) -> Option<Arc<TypeData>> {
        self.types.get(name).cloned()
    }
    
    pub fn get_function(&self, name: &Symbol) -> Option<Arc<FunctionData>> {
        self.functions.get(name).cloned()
    }
    
    pub fn get_constant(&self, name: &Symbol) -> Option<Arc<ConstantData>> {
        self.constants.get(name).cloned()
    }
    
    pub fn cache_symbol(&mut self, name: Symbol, data: SymbolData) {
        let data = Arc::new(data);
        self.symbols.insert(name, data);
    }
    
    pub fn cache_type(&mut self, name: Symbol, data: TypeData) {
        let data = Arc::new(data);
        self.types.insert(name, data);
    }
    
    pub fn cache_function(&mut self, name: Symbol, data: FunctionData) {
        let data = Arc::new(data);
        self.functions.insert(name, data);
    }
    
    pub fn cache_constant(&mut self, name: Symbol, data: ConstantData) {
        let data = Arc::new(data);
        self.constants.insert(name, data);
    }
    
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.types.clear();
        self.functions.clear();
        self.constants.clear();
    }
    
    pub fn remove_symbol(&mut self, name: &Symbol) {
        self.symbols.remove(name);
    }
    
    pub fn remove_type(&mut self, name: &Symbol) {
        self.types.remove(name);
    }
    
    pub fn remove_function(&mut self, name: &Symbol) {
        self.functions.remove(name);
    }
    
    pub fn remove_constant(&mut self, name: &Symbol) {
        self.constants.remove(name);
    }
    
    pub fn contains_symbol(&self, name: &Symbol) -> bool {
        self.symbols.contains_key(name)
    }
    
    pub fn contains_type(&self, name: &Symbol) -> bool {
        self.types.contains_key(name)
    }
    
    pub fn contains_function(&self, name: &Symbol) -> bool {
        self.functions.contains_key(name)
    }
    
    pub fn contains_constant(&self, name: &Symbol) -> bool {
        self.constants.contains_key(name)
    }
    
    pub fn symbols(&self) -> impl Iterator<Item = (&Symbol, &Arc<SymbolData>)> {
        self.symbols.iter()
    }
    
    pub fn types(&self) -> impl Iterator<Item = (&Symbol, &Arc<TypeData>)> {
        self.types.iter()
    }
    
    pub fn functions(&self) -> impl Iterator<Item = (&Symbol, &Arc<FunctionData>)> {
        self.functions.iter()
    }
    
    pub fn constants(&self) -> impl Iterator<Item = (&Symbol, &Arc<ConstantData>)> {
        self.constants.iter()
    }
} 