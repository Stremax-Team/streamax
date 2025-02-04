use std::collections::HashSet;
use semver::Version;
use crate::core::{ModuleId, Symbol};
use super::{Module, ModuleError, Result};

/// Module validation system
pub trait ModuleValidator: Send + Sync {
    fn validate(&self, module: &Module) -> Result<()>;
}

/// Default module validator implementation
#[derive(Default)]
pub struct DefaultModuleValidator {
    max_dependencies: usize,
    max_exports: usize,
    required_exports: HashSet<Symbol>,
}

impl DefaultModuleValidator {
    pub fn new() -> Self {
        Self {
            max_dependencies: 100,
            max_exports: 1000,
            required_exports: HashSet::new(),
        }
    }

    pub fn with_max_dependencies(mut self, max: usize) -> Self {
        self.max_dependencies = max;
        self
    }

    pub fn with_max_exports(mut self, max: usize) -> Self {
        self.max_exports = max;
        self
    }

    pub fn require_export(mut self, symbol: Symbol) -> Self {
        self.required_exports.insert(symbol);
        self
    }

    fn check_circular_dependencies(&self, module: &Module, visited: &mut HashSet<ModuleId>, path: &mut Vec<ModuleId>) -> Result<()> {
        if !visited.insert(module.id().clone()) {
            if path.contains(module.id()) {
                return Err(ModuleError::CircularDependency {
                    path: path.clone(),
                });
            }
            return Ok(());
        }

        path.push(module.id().clone());
        
        for dep_id in module.dependencies() {
            // In a real implementation, we'd need to get the actual Module instance
            // This is just for demonstration
            if path.contains(dep_id) {
                return Err(ModuleError::CircularDependency {
                    path: path.clone(),
                });
            }
        }

        path.pop();
        Ok(())
    }

    fn validate_exports(&self, module: &Module) -> Result<()> {
        // Check maximum exports
        if module.exports().count() > self.max_exports {
            return Err(ModuleError::ValidationError {
                module: module.id().clone(),
                message: format!("Too many exports: {} (max: {})", 
                    module.exports().count(), self.max_exports),
            });
        }

        // Check required exports
        for required in &self.required_exports {
            if !module.get_export(required).is_some() {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Missing required export: {}", required.0),
                });
            }
        }

        // Validate each export
        for (name, export) in module.exports() {
            // Validate export visibility
            if export.visibility == super::types::Visibility::Private && export.deprecated {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Private export {} cannot be deprecated", name.0),
                });
            }

            // Validate export kind
            match &export.kind {
                super::types::ExportKind::Function(f) => {
                    if f.is_unsafe && !f.is_public {
                        return Err(ModuleError::ValidationError {
                            module: module.id().clone(),
                            message: format!("Unsafe function {} must be public", name.0),
                        });
                    }
                }
                super::types::ExportKind::Type(t) => {
                    if t.constructors.is_empty() && t.is_public {
                        return Err(ModuleError::ValidationError {
                            module: module.id().clone(),
                            message: format!("Public type {} must have at least one constructor", name.0),
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn validate_imports(&self, module: &Module) -> Result<()> {
        let mut seen_imports = HashSet::new();

        for import in module.imports() {
            // Check for duplicate imports
            if !seen_imports.insert((&import.module, &import.name)) {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Duplicate import: {} from {}", import.name.0, import.module.0),
                });
            }

            // Validate import visibility
            if import.visibility == super::types::Visibility::Private && import.is_reexport {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Private import {} cannot be re-exported", import.name.0),
                });
            }
        }

        Ok(())
    }

    fn validate_version(&self, module: &Module) -> Result<()> {
        if let Some(version_str) = module.version() {
            if let Err(e) = Version::parse(&version_str) {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Invalid version format: {}", e),
                });
            }
        }
        Ok(())
    }
}

impl ModuleValidator for DefaultModuleValidator {
    fn validate(&self, module: &Module) -> Result<()> {
        // Check number of dependencies
        if module.dependencies().count() > self.max_dependencies {
            return Err(ModuleError::ValidationError {
                module: module.id().clone(),
                message: format!("Too many dependencies: {} (max: {})", 
                    module.dependencies().count(), self.max_dependencies),
            });
        }

        // Check for circular dependencies
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        self.check_circular_dependencies(module, &mut visited, &mut path)?;

        // Validate exports
        self.validate_exports(module)?;

        // Validate imports
        self.validate_imports(module)?;

        // Validate version
        self.validate_version(module)?;

        Ok(())
    }
}