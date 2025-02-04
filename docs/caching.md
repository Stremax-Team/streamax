 # Caching System

The Stremax module system implements a sophisticated caching system that optimizes performance through intelligent symbol and module caching.

## Overview

The caching system provides:
- LRU-based caching for symbols, types, functions, and constants
- Performance metrics tracking
- Thread-safe concurrent access
- Automatic cache eviction
- Memory usage optimization

## Components

### Cache Statistics

```rust
#[derive(Debug, Default)]
pub struct CacheStats {
    hits: usize,
    misses: usize,
    evictions: usize,
    total_lookup_time: Duration,
    last_clear: Instant,
}
```

### Optimized Module Cache

```rust
pub struct OptimizedModuleCache {
    symbols: RwLock<LruCache<Symbol, Arc<SymbolData>>>,
    types: RwLock<LruCache<Symbol, Arc<TypeData>>>,
    functions: RwLock<LruCache<Symbol, Arc<FunctionData>>>,
    constants: RwLock<LruCache<Symbol, Arc<ConstantData>>>,
    stats: RwLock<CacheStats>,
    max_size: usize,
}
```

## Usage

### Basic Cache Operations

```rust
// Create a new cache with specified size
let cache = OptimizedModuleCache::new(1000);

// Cache a symbol
cache.cache_symbol(symbol, data);

// Lookup a symbol
if let Some(data) = cache.get_symbol(&symbol) {
    // Use cached data
}

// Remove from cache
cache.remove_symbol(&symbol);
```

### Custom Cache Sizes

```rust
let cache = OptimizedModuleCache::with_sizes(
    symbol_size: 1000,    // Symbols cache size
    type_size: 500,       // Types cache size
    function_size: 200,   // Functions cache size
    constant_size: 100,   // Constants cache size
);
```

### Performance Monitoring

```rust
// Get cache statistics
let stats = cache.get_stats();
println!("Cache hit rate: {:.2}%", stats.hit_rate() * 100.0);
println!("Average lookup time: {:?}", stats.average_lookup_time());
println!("Total evictions: {}", stats.evictions);
```

## Cache Policies

### 1. LRU Eviction

```rust
impl OptimizedModuleCache {
    fn evict_if_needed(&self, cache: &mut LruCache<Symbol, Arc<SymbolData>>) {
        if cache.len() >= self.max_size {
            if let Some((key, _)) = cache.pop_lru() {
                self.stats.write().evictions += 1;
                log::debug!("Evicted symbol: {}", key.0);
            }
        }
    }
}
```

### 2. Selective Caching

```rust
impl OptimizedModuleCache {
    pub fn should_cache(&self, data: &SymbolData) -> bool {
        match data.kind {
            // Always cache frequently used symbols
            SymbolKind::Function(_) => true,
            // Cache types only if they have constructors
            SymbolKind::Type(ref t) => !t.constructors.is_empty(),
            // Cache small constants
            SymbolKind::Constant(ref c) => c.value.len() < 1024,
            // Don't cache large modules
            SymbolKind::Module(_) => false,
        }
    }
}
```

### 3. Cache Invalidation

```rust
impl OptimizedModuleCache {
    pub fn invalidate_module(&self, module_id: &ModuleId) {
        let mut symbols = self.symbols.write();
        symbols.retain(|_, data| data.module_id != module_id);
        
        let mut types = self.types.write();
        types.retain(|_, data| data.module_id != module_id);
        
        // Similar for functions and constants
    }
}
```

## Memory Management

### 1. Reference Counting

```rust
// Use Arc for thread-safe reference counting
type CachedSymbol = Arc<SymbolData>;

impl OptimizedModuleCache {
    pub fn get_symbol(&self, name: &Symbol) -> Option<Arc<SymbolData>> {
        // Clone the Arc, increasing the reference count
        self.symbols.read().get(name).cloned()
    }
}
```

### 2. Memory Limits

```rust
impl OptimizedModuleCache {
    pub fn estimate_memory_usage(&self) -> usize {
        let symbols = self.symbols.read();
        let types = self.types.read();
        let functions = self.functions.read();
        let constants = self.constants.read();

        symbols.len() * std::mem::size_of::<SymbolData>() +
        types.len() * std::mem::size_of::<TypeData>() +
        functions.len() * std::mem::size_of::<FunctionData>() +
        constants.len() * std::mem::size_of::<ConstantData>()
    }

    pub fn enforce_memory_limit(&self, limit: usize) {
        while self.estimate_memory_usage() > limit {
            self.evict_least_used();
        }
    }
}
```

## Performance Optimization

### 1. Concurrent Access

```rust
impl OptimizedModuleCache {
    pub fn get_or_compute<F>(&self, key: Symbol, compute: F) -> Arc<SymbolData>
    where
        F: FnOnce() -> SymbolData,
    {
        // Try read lock first
        if let Some(value) = self.symbols.read().get(&key).cloned() {
            return value;
        }

        // If not found, acquire write lock and compute
        let mut cache = self.symbols.write();
        if let Some(value) = cache.get(&key).cloned() {
            return value;
        }

        let value = Arc::new(compute());
        cache.put(key, value.clone());
        value
    }
}
```

### 2. Batch Operations

```rust
impl OptimizedModuleCache {
    pub fn cache_batch(&self, symbols: Vec<(Symbol, SymbolData)>) {
        let mut cache = self.symbols.write();
        for (key, value) in symbols {
            self.evict_if_needed(&mut cache);
            cache.put(key, Arc::new(value));
        }
    }
}
```

### 3. Prefetching

```rust
impl OptimizedModuleCache {
    pub async fn prefetch_module(&self, module: &Module) {
        let symbols = module.collect_exported_symbols();
        let futures: Vec<_> = symbols.into_iter()
            .map(|symbol| self.prefetch_symbol(symbol))
            .collect();
        
        join_all(futures).await;
    }
}
```

## Best Practices

1. **Monitor Cache Performance**:
   ```rust
   // Regularly check cache metrics
   let stats = cache.get_stats();
   if stats.hit_rate() < 0.5 {
       log::warn!("Low cache hit rate: {:.2}%", stats.hit_rate() * 100.0);
   }
   ```

2. **Tune Cache Sizes**:
   ```rust
   // Adjust cache sizes based on usage patterns
   if stats.evictions > 1000 && stats.hit_rate() < 0.8 {
       cache.resize(cache.max_size() * 2);
   }
   ```

3. **Handle Cache Pressure**:
   ```rust
   // Clear cache under memory pressure
   if system_memory_pressure() {
       cache.clear();
       log::info!("Cache cleared due to memory pressure");
   }
   ```