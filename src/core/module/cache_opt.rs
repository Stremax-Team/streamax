use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use lru::LruCache;
use parking_lot::RwLock;
use crate::core::{Error, Result, Symbol};
use super::types::*;

/// Cache statistics for monitoring and optimization
#[derive(Debug, Default)]
pub struct CacheStats {
    hits: usize,
    misses: usize,
    evictions: usize,
    total_lookup_time: Duration,
    last_clear: Instant,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    pub fn average_lookup_time(&self) -> Duration {
        let total = self.hits + self.misses;
        if total == 0 {
            Duration::default()
        } else {
            self.total_lookup_time / total as u32
        }
    }

    pub fn clear(&mut self) {
        *self = CacheStats {
            last_clear: Instant::now(),
            ..Default::default()
        };
    }
}

/// Optimized module cache with LRU eviction and statistics
pub struct OptimizedModuleCache {
    symbols: RwLock<LruCache<Symbol, Arc<SymbolData>>>,
    types: RwLock<LruCache<Symbol, Arc<TypeData>>>,
    functions: RwLock<LruCache<Symbol, Arc<FunctionData>>>,
    constants: RwLock<LruCache<Symbol, Arc<ConstantData>>>,
    stats: RwLock<CacheStats>,
    max_size: usize,
}

impl OptimizedModuleCache {
    pub fn new(max_size: usize) -> Self {
        OptimizedModuleCache {
            symbols: RwLock::new(LruCache::new(max_size)),
            types: RwLock::new(LruCache::new(max_size)),
            functions: RwLock::new(LruCache::new(max_size)),
            constants: RwLock::new(LruCache::new(max_size)),
            stats: RwLock::new(CacheStats::default()),
            max_size,
        }
    }

    pub fn with_sizes(
        symbol_size: usize,
        type_size: usize,
        function_size: usize,
        constant_size: usize,
    ) -> Self {
        OptimizedModuleCache {
            symbols: RwLock::new(LruCache::new(symbol_size)),
            types: RwLock::new(LruCache::new(type_size)),
            functions: RwLock::new(LruCache::new(function_size)),
            constants: RwLock::new(LruCache::new(constant_size)),
            stats: RwLock::new(CacheStats::default()),
            max_size: symbol_size,
        }
    }

    pub fn get_symbol(&self, name: &Symbol) -> Option<Arc<SymbolData>> {
        let start = Instant::now();
        let result = self.symbols.write().get(name).cloned();
        
        let mut stats = self.stats.write();
        stats.total_lookup_time += start.elapsed();
        
        if result.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
        
        result
    }

    pub fn get_type(&self, name: &Symbol) -> Option<Arc<TypeData>> {
        let start = Instant::now();
        let result = self.types.write().get(name).cloned();
        
        let mut stats = self.stats.write();
        stats.total_lookup_time += start.elapsed();
        
        if result.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
        
        result
    }

    pub fn get_function(&self, name: &Symbol) -> Option<Arc<FunctionData>> {
        let start = Instant::now();
        let result = self.functions.write().get(name).cloned();
        
        let mut stats = self.stats.write();
        stats.total_lookup_time += start.elapsed();
        
        if result.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
        
        result
    }

    pub fn get_constant(&self, name: &Symbol) -> Option<Arc<ConstantData>> {
        let start = Instant::now();
        let result = self.constants.write().get(name).cloned();
        
        let mut stats = self.stats.write();
        stats.total_lookup_time += start.elapsed();
        
        if result.is_some() {
            stats.hits += 1;
        } else {
            stats.misses += 1;
        }
        
        result
    }

    pub fn cache_symbol(&self, name: Symbol, data: SymbolData) {
        let mut symbols = self.symbols.write();
        if symbols.len() >= self.max_size {
            self.stats.write().evictions += 1;
        }
        symbols.put(name, Arc::new(data));
    }

    pub fn cache_type(&self, name: Symbol, data: TypeData) {
        let mut types = self.types.write();
        if types.len() >= self.max_size {
            self.stats.write().evictions += 1;
        }
        types.put(name, Arc::new(data));
    }

    pub fn cache_function(&self, name: Symbol, data: FunctionData) {
        let mut functions = self.functions.write();
        if functions.len() >= self.max_size {
            self.stats.write().evictions += 1;
        }
        functions.put(name, Arc::new(data));
    }

    pub fn cache_constant(&self, name: Symbol, data: ConstantData) {
        let mut constants = self.constants.write();
        if constants.len() >= self.max_size {
            self.stats.write().evictions += 1;
        }
        constants.put(name, Arc::new(data));
    }

    pub fn clear(&self) {
        self.symbols.write().clear();
        self.types.write().clear();
        self.functions.write().clear();
        self.constants.write().clear();
        self.stats.write().clear();
    }

    pub fn get_stats(&self) -> CacheStats {
        self.stats.read().clone()
    }

    pub fn resize(&self, new_size: usize) {
        let mut symbols = self.symbols.write();
        let mut types = self.types.write();
        let mut functions = self.functions.write();
        let mut constants = self.constants.write();

        *symbols = LruCache::new(new_size);
        *types = LruCache::new(new_size);
        *functions = LruCache::new(new_size);
        *constants = LruCache::new(new_size);
    }

    pub fn remove_symbol(&self, name: &Symbol) {
        self.symbols.write().pop(name);
    }

    pub fn remove_type(&self, name: &Symbol) {
        self.types.write().pop(name);
    }

    pub fn remove_function(&self, name: &Symbol) {
        self.functions.write().pop(name);
    }

    pub fn remove_constant(&self, name: &Symbol) {
        self.constants.write().pop(name);
    }

    pub fn contains_symbol(&self, name: &Symbol) -> bool {
        self.symbols.read().contains(name)
    }

    pub fn contains_type(&self, name: &Symbol) -> bool {
        self.types.read().contains(name)
    }

    pub fn contains_function(&self, name: &Symbol) -> bool {
        self.functions.read().contains(name)
    }

    pub fn contains_constant(&self, name: &Symbol) -> bool {
        self.constants.read().contains(name)
    }
}