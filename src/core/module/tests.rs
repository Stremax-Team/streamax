use std::path::PathBuf;
use std::sync::Arc;
use tokio::test;
use super::*;
use super::types::*;
use super::error::*;

#[test]
async fn test_module_basic() {
    let id = ModuleId("test".to_string());
    let path = PathBuf::from("test.strm");
    let module = Module::new(id.clone(), path);
    
    assert_eq!(module.id(), &id);
    assert!(module.exports().count() == 0);
    assert!(module.imports().count() == 0);
    assert!(module.dependencies().count() == 0);
}

#[test]
async fn test_module_exports() {
    let mut module = Module::new(
        ModuleId("test".to_string()),
        PathBuf::from("test.strm"),
    );
    
    let export = Export {
        name: Symbol("test_fn".to_string()),
        visibility: Visibility::Public,
        kind: ExportKind::Function(FunctionExport {
            signature: "fn test() -> u32".to_string(),
            is_public: true,
            is_async: false,
            is_unsafe: false,
        }),
        docs: Some("Test function".to_string()),
        deprecated: false,
    };
    
    module.add_export(Symbol("test_fn".to_string()), export.clone());
    
    assert_eq!(module.exports().count(), 1);
    assert_eq!(module.get_export(&Symbol("test_fn".to_string())), Some(&export));
}

#[test]
async fn test_module_imports() {
    let mut module = Module::new(
        ModuleId("test".to_string()),
        PathBuf::from("test.strm"),
    );
    
    let import = Import {
        module: ModuleId("other".to_string()),
        name: Symbol("other_fn".to_string()),
        alias: None,
        visibility: Visibility::Public,
        is_reexport: false,
    };
    
    module.add_import(import.clone());
    
    assert_eq!(module.imports().count(), 1);
    assert!(module.imports().any(|i| i == &import));
}

#[test]
async fn test_module_validation() {
    let mut validator = DefaultModuleValidator::new()
        .with_max_dependencies(2)
        .with_max_exports(2)
        .require_export(Symbol("required".to_string()));
        
    let mut module = Module::new(
        ModuleId("test".to_string()),
        PathBuf::from("test.strm"),
    );
    
    // Should fail - missing required export
    assert!(validator.validate(&module).is_err());
    
    // Add required export
    module.add_export(
        Symbol("required".to_string()),
        Export {
            name: Symbol("required".to_string()),
            visibility: Visibility::Public,
            kind: ExportKind::Function(FunctionExport {
                signature: "fn required()".to_string(),
                is_public: true,
                is_async: false,
                is_unsafe: false,
            }),
            docs: None,
            deprecated: false,
        },
    );
    
    // Should pass now
    assert!(validator.validate(&module).is_ok());
    
    // Add too many exports
    for i in 0..3 {
        module.add_export(
            Symbol(format!("test{}", i)),
            Export {
                name: Symbol(format!("test{}", i)),
                visibility: Visibility::Public,
                kind: ExportKind::Function(FunctionExport {
                    signature: format!("fn test{}()", i),
                    is_public: true,
                    is_async: false,
                    is_unsafe: false,
                }),
                docs: None,
                deprecated: false,
            },
        );
    }
    
    // Should fail - too many exports
    assert!(validator.validate(&module).is_err());
}

#[test]
async fn test_module_cache() {
    let cache = OptimizedModuleCache::new(2);
    
    let symbol = Symbol("test".to_string());
    let data = SymbolData {
        name: symbol.clone(),
        kind: SymbolKind::Function(Arc::new(FunctionData {
            signature: "fn test()".to_string(),
            body: None,
            attributes: vec![],
        })),
        location: SourceLocation {
            file: PathBuf::from("test.strm"),
            line: 1,
            column: 1,
            length: 1,
        },
        docs: None,
    };
    
    // Test caching
    cache.cache_symbol(symbol.clone(), data);
    assert!(cache.contains_symbol(&symbol));
    
    // Test eviction
    let symbol2 = Symbol("test2".to_string());
    let data2 = SymbolData {
        name: symbol2.clone(),
        kind: SymbolKind::Function(Arc::new(FunctionData {
            signature: "fn test2()".to_string(),
            body: None,
            attributes: vec![],
        })),
        location: SourceLocation {
            file: PathBuf::from("test.strm"),
            line: 2,
            column: 1,
            length: 1,
        },
        docs: None,
    };
    
    cache.cache_symbol(symbol2.clone(), data2);
    
    let symbol3 = Symbol("test3".to_string());
    let data3 = SymbolData {
        name: symbol3.clone(),
        kind: SymbolKind::Function(Arc::new(FunctionData {
            signature: "fn test3()".to_string(),
            body: None,
            attributes: vec![],
        })),
        location: SourceLocation {
            file: PathBuf::from("test.strm"),
            line: 3,
            column: 1,
            length: 1,
        },
        docs: None,
    };
    
    cache.cache_symbol(symbol3.clone(), data3);
    
    // First symbol should be evicted
    assert!(!cache.contains_symbol(&symbol));
    assert!(cache.contains_symbol(&symbol2));
    assert!(cache.contains_symbol(&symbol3));
}

#[test]
async fn test_concurrent_module_loader() {
    let loader = Arc::new(ConcurrentModuleLoader::new());
    
    // Add a search path
    loader.add_search_path("test_modules");
    
    // Create a test module file
    std::fs::create_dir_all("test_modules").unwrap();
    std::fs::write(
        "test_modules/test.strm",
        "// Test module content"
    ).unwrap();
    
    // Load module
    let module_id = ModuleId("test".to_string());
    let module = loader.load_module(module_id.clone()).await.unwrap();
    
    assert_eq!(module.read().id(), &module_id);
    
    // Clean up
    std::fs::remove_dir_all("test_modules").unwrap();
}

#[test]
async fn test_hot_reload() {
    let loader = Arc::new(ConcurrentModuleLoader::new());
    let (hot_reload, mut rx) = HotReloadManager::new(loader.clone()).unwrap();
    
    // Add a search path
    loader.add_search_path("test_modules");
    
    // Create a test module file
    std::fs::create_dir_all("test_modules").unwrap();
    std::fs::write(
        "test_modules/test.strm",
        "// Initial content"
    ).unwrap();
    
    // Load and watch module
    let module_id = ModuleId("test".to_string());
    let module = loader.load_module(module_id.clone()).await.unwrap();
    hot_reload.watch_module(&module.read()).unwrap();
    
    // Modify the file
    std::fs::write(
        "test_modules/test.strm",
        "// Modified content"
    ).unwrap();
    
    // Wait for event
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    let events = hot_reload.check_updates().await.unwrap();
    assert!(!events.is_empty());
    
    // Clean up
    std::fs::remove_dir_all("test_modules").unwrap();
}

#[test]
async fn test_module_resolver() {
    let loader = Arc::new(ConcurrentModuleLoader::new());
    
    // Create test modules
    let mut module1 = Module::new(
        ModuleId("test1".to_string()),
        PathBuf::from("test1.strm"),
    );
    
    let export = Export {
        name: Symbol("test_fn".to_string()),
        visibility: Visibility::Public,
        kind: ExportKind::Function(FunctionExport {
            signature: "fn test_fn()".to_string(),
            is_public: true,
            is_async: false,
            is_unsafe: false,
        }),
        docs: None,
        deprecated: false,
    };
    
    module1.add_export(Symbol("test_fn".to_string()), export);
    
    let mut module2 = Module::new(
        ModuleId("test2".to_string()),
        PathBuf::from("test2.strm"),
    );
    
    let import = Import {
        module: ModuleId("test1".to_string()),
        name: Symbol("test_fn".to_string()),
        alias: None,
        visibility: Visibility::Public,
        is_reexport: false,
    };
    
    module2.add_import(import);
    
    // Test resolution
    let resolver = ModuleResolver::new(&loader, &module2);
    assert!(resolver.resolve_symbol(&Symbol("test_fn".to_string())).is_err());
}