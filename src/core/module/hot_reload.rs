use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use notify::{Watcher, RecursiveMode, Event};
use parking_lot::RwLock;
use tokio::sync::broadcast;
use crate::core::{Error, Result, ModuleId, Symbol};
use super::{Module, ConcurrentModuleLoader};

/// Event emitted when a module changes
#[derive(Debug, Clone)]
pub enum ModuleChangeEvent {
    Created(ModuleId),
    Modified(ModuleId),
    Removed(ModuleId),
    Reloaded(ModuleId),
    Error { module: ModuleId, error: Arc<Error> },
}

/// Hot reload manager for modules
pub struct HotReloadManager {
    loader: Arc<ConcurrentModuleLoader>,
    watcher: notify::RecommendedWatcher,
    file_times: RwLock<HashMap<PathBuf, SystemTime>>,
    tx: broadcast::Sender<ModuleChangeEvent>,
    watched_paths: RwLock<HashMap<PathBuf, ModuleId>>,
}

impl HotReloadManager {
    pub fn new(loader: Arc<ConcurrentModuleLoader>) -> Result<(Self, broadcast::Receiver<ModuleChangeEvent>)> {
        let (tx, rx) = broadcast::channel(100);
        let tx_clone = tx.clone();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    if let Err(e) = Self::handle_fs_event(event, &tx_clone) {
                        eprintln!("Error handling fs event: {}", e);
                    }
                }
                Err(e) => eprintln!("Watch error: {}", e),
            }
        })?;

        Ok((
            HotReloadManager {
                loader,
                watcher,
                file_times: RwLock::new(HashMap::new()),
                tx,
                watched_paths: RwLock::new(HashMap::new()),
            },
            rx
        ))
    }

    fn handle_fs_event(event: Event, tx: &broadcast::Sender<ModuleChangeEvent>) -> Result<()> {
        use notify::EventKind::*;

        match event.kind {
            Create(notify::event::CreateKind::File) => {
                for path in event.paths {
                    if let Some(module_id) = Self::path_to_module_id(&path) {
                        let _ = tx.send(ModuleChangeEvent::Created(module_id));
                    }
                }
            }
            Modify(notify::event::ModifyKind::Data(_)) => {
                for path in event.paths {
                    if let Some(module_id) = Self::path_to_module_id(&path) {
                        let _ = tx.send(ModuleChangeEvent::Modified(module_id));
                    }
                }
            }
            Remove(notify::event::RemoveKind::File) => {
                for path in event.paths {
                    if let Some(module_id) = Self::path_to_module_id(&path) {
                        let _ = tx.send(ModuleChangeEvent::Removed(module_id));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn path_to_module_id(path: &Path) -> Option<ModuleId> {
        path.file_stem()
            .and_then(|s| s.to_str())
            .map(|s| ModuleId(s.to_string()))
    }

    pub fn watch_module(&self, module: &Module) -> Result<()> {
        let path = module.path().to_owned();
        let id = module.id().clone();
        
        self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
        self.watched_paths.write().insert(path.clone(), id);
        
        if let Ok(metadata) = std::fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                self.file_times.write().insert(path, modified);
            }
        }
        
        Ok(())
    }

    pub fn unwatch_module(&self, module: &Module) -> Result<()> {
        let path = module.path();
        self.watcher.unwatch(path)?;
        self.watched_paths.write().remove(path);
        self.file_times.write().remove(path);
        Ok(())
    }

    pub async fn check_updates(&self) -> Result<Vec<ModuleChangeEvent>> {
        let mut events = Vec::new();
        let paths = self.watched_paths.read().clone();
        
        for (path, module_id) in paths {
            if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    let last_modified = self.file_times.read().get(&path).cloned();
                    
                    if let Some(last) = last_modified {
                        if modified > last {
                            // File was modified, trigger reload
                            match self.reload_module(&module_id).await {
                                Ok(()) => {
                                    events.push(ModuleChangeEvent::Reloaded(module_id.clone()));
                                    self.file_times.write().insert(path, modified);
                                }
                                Err(e) => {
                                    events.push(ModuleChangeEvent::Error {
                                        module: module_id.clone(),
                                        error: Arc::new(e),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(events)
    }

    async fn reload_module(&self, id: &ModuleId) -> Result<()> {
        // Unload and reload the module
        self.loader.unload_module(id).await?;
        self.loader.load_module(id.clone()).await?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ModuleChangeEvent> {
        self.tx.subscribe()
    }

    pub async fn start_watching(&self) -> Result<()> {
        // Watch all search paths
        for path in self.loader.get_search_paths() {
            self.watcher.watch(&path, RecursiveMode::Recursive)?;
        }
        Ok(())
    }

    pub async fn stop_watching(&self) -> Result<()> {
        // Unwatch all paths
        let paths: Vec<_> = self.watched_paths.read().keys().cloned().collect();
        for path in paths {
            self.watcher.unwatch(&path)?;
        }
        self.watched_paths.write().clear();
        self.file_times.write().clear();
        Ok(())
    }
}