use std::sync::Arc;
use crate::core::{Error, Result, Symbol};
use super::{Module, ModuleLoader};
use super::types::*;

/// Module resolver for resolving symbols between modules
pub struct ModuleResolver<'a> {
    loader: &'a ModuleLoader,
    current_module: &'a Module,
}

impl<'a> ModuleResolver<'a> {
    pub fn new(loader: &'a ModuleLoader, current_module: &'a Module) -> Self {
        ModuleResolver {
            loader,
            current_module,
        }
    }
    
    pub fn resolve_symbol(&self, name: &Symbol) -> Result<Arc<SymbolData>> {
        self.loader.resolve_symbol(self.current_module, name)
    }
    
    pub fn resolve_type(&self, name: &Symbol) -> Result<Arc<TypeData>> {
        self.resolve_symbol(name).and_then(|data| {
            match &data.kind {
                SymbolKind::Type(ty) => Ok(ty.clone()),
                _ => Err(Error::TypeError(format!("{} is not a type", name.0))),
            }
        })
    }
    
    pub fn resolve_function(&self, name: &Symbol) -> Result<Arc<FunctionData>> {
        self.resolve_symbol(name).and_then(|data| {
            match &data.kind {
                SymbolKind::Function(f) => Ok(f.clone()),
                _ => Err(Error::TypeError(format!("{} is not a function", name.0))),
            }
        })
    }
    
    pub fn resolve_constant(&self, name: &Symbol) -> Result<Arc<ConstantData>> {
        self.resolve_symbol(name).and_then(|data| {
            match &data.kind {
                SymbolKind::Constant(c) => Ok(c.clone()),
                _ => Err(Error::TypeError(format!("{} is not a constant", name.0))),
            }
        })
    }
    
    pub fn resolve_module(&self, name: &Symbol) -> Result<Arc<ModuleData>> {
        self.resolve_symbol(name).and_then(|data| {
            match &data.kind {
                SymbolKind::Module(m) => Ok(m.clone()),
                _ => Err(Error::TypeError(format!("{} is not a module", name.0))),
            }
        })
    }
    
    pub fn resolve_export(&self, name: &Symbol) -> Result<Export> {
        if let Some(export) = self.current_module.get_export(name) {
            Ok(export.clone())
        } else {
            for import in self.current_module.imports() {
                if import.name == *name {
                    if let Some(module) = self.loader.get_module(&import.module) {
                        if let Some(export) = module.get_export(&import.name) {
                            return Ok(export.clone());
                        }
                    }
                }
            }
            Err(Error::SymbolNotFound(name.0.clone()))
        }
    }
    
    pub fn resolve_import(&self, import: &Import) -> Result<Export> {
        if let Some(module) = self.loader.get_module(&import.module) {
            if let Some(export) = module.get_export(&import.name) {
                Ok(export.clone())
            } else {
                Err(Error::SymbolNotFound(import.name.0.clone()))
            }
        } else {
            Err(Error::ModuleNotFound(import.module.0.clone()))
        }
    }
    
    pub fn check_visibility(&self, name: &Symbol, visibility: Visibility) -> Result<()> {
        let export = self.resolve_export(name)?;
        if export.visibility >= visibility {
            Ok(())
        } else {
            Err(Error::TypeError(format!(
                "{} is not visible with {:?} visibility",
                name.0, visibility
            )))
        }
    }
} 