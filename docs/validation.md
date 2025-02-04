# Module Validation

The Stremax module system implements a comprehensive validation system to ensure module correctness and maintain system integrity.

## Overview

The validation system provides:
- Dependency cycle detection
- Export/import validation
- Type safety checks
- Version compatibility verification
- Custom validation rules

## Components

### Module Validator Interface

```rust
pub trait ModuleValidator: Send + Sync {
    fn validate(&self, module: &Module) -> Result<()>;
}
```

### Default Validator

```rust
pub struct DefaultModuleValidator {
    max_dependencies: usize,
    max_exports: usize,
    required_exports: HashSet<Symbol>,
}
```

## Validation Rules

### 1. Dependency Validation

```rust
impl DefaultModuleValidator {
    fn check_circular_dependencies(&self, module: &Module) -> Result<()> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        self.check_dependency_cycle(module, &mut visited, &mut path)
    }

    fn check_dependency_cycle(
        &self,
        module: &Module,
        visited: &mut HashSet<ModuleId>,
        path: &mut Vec<ModuleId>
    ) -> Result<()> {
        if !visited.insert(module.id().clone()) {
            if path.contains(module.id()) {
                return Err(ModuleError::CircularDependency {
                    path: path.clone(),
                });
            }
            return Ok(());
        }

        path.push(module.id().clone());
        // Check dependencies recursively
        path.pop();
        Ok(())
    }
}
```

### 2. Export Validation

```rust
impl DefaultModuleValidator {
    fn validate_exports(&self, module: &Module) -> Result<()> {
        // Check maximum exports
        if module.exports().count() > self.max_exports {
            return Err(ModuleError::ValidationError {
                module: module.id().clone(),
                message: format!("Too many exports: {}", module.exports().count()),
            });
        }

        // Validate each export
        for (name, export) in module.exports() {
            self.validate_export(module, name, export)?;
        }

        Ok(())
    }

    fn validate_export(&self, module: &Module, name: &Symbol, export: &Export) -> Result<()> {
        // Check visibility rules
        if export.visibility == Visibility::Private && export.deprecated {
            return Err(ModuleError::ValidationError {
                module: module.id().clone(),
                message: format!("Private export {} cannot be deprecated", name.0),
            });
        }

        // Validate based on export kind
        match &export.kind {
            ExportKind::Function(f) => self.validate_function_export(f)?,
            ExportKind::Type(t) => self.validate_type_export(t)?,
            ExportKind::Constant(c) => self.validate_constant_export(c)?,
            ExportKind::Module(m) => self.validate_module_export(m)?,
        }

        Ok(())
    }
}
```

### 3. Import Validation

```rust
impl DefaultModuleValidator {
    fn validate_imports(&self, module: &Module) -> Result<()> {
        let mut seen_imports = HashSet::new();

        for import in module.imports() {
            // Check for duplicate imports
            if !seen_imports.insert((&import.module, &import.name)) {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Duplicate import: {}", import.name.0),
                });
            }

            // Validate import visibility
            if import.visibility == Visibility::Private && import.is_reexport {
                return Err(ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Private import cannot be re-exported"),
                });
            }
        }

        Ok(())
    }
}
```

## Custom Validation Rules

### 1. Creating Custom Validators

```rust
pub struct SecurityValidator {
    allowed_unsafe: bool,
    trusted_modules: HashSet<ModuleId>,
}

impl ModuleValidator for SecurityValidator {
    fn validate(&self, module: &Module) -> Result<()> {
        // Check if unsafe code is allowed
        if !self.allowed_unsafe {
            for (name, export) in module.exports() {
                if let ExportKind::Function(f) = &export.kind {
                    if f.is_unsafe {
                        return Err(ModuleError::ValidationError {
                            module: module.id().clone(),
                            message: format!("Unsafe code is not allowed: {}", name.0),
                        });
                    }
                }
            }
        }

        // Check if module is trusted
        if !self.trusted_modules.contains(module.id()) {
            // Additional security checks
        }

        Ok(())
    }
}
```

### 2. Combining Validators

```rust
pub struct CompositeValidator {
    validators: Vec<Box<dyn ModuleValidator>>,
}

impl ModuleValidator for CompositeValidator {
    fn validate(&self, module: &Module) -> Result<()> {
        for validator in &self.validators {
            validator.validate(module)?;
        }
        Ok(())
    }
}
```

## Version Validation

```rust
impl DefaultModuleValidator {
    fn validate_version(&self, module: &Module) -> Result<()> {
        if let Some(version) = module.version() {
            // Parse version
            let version = semver::Version::parse(&version)
                .map_err(|e| ModuleError::ValidationError {
                    module: module.id().clone(),
                    message: format!("Invalid version format: {}", e),
                })?;

            // Check compatibility with dependencies
            for dep_id in module.dependencies() {
                if let Some(dep) = self.get_module(dep_id) {
                    if let Some(dep_version) = dep.version() {
                        self.check_version_compatibility(&version, &dep_version)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn check_version_compatibility(
        &self,
        version: &semver::Version,
        dep_version: &str,
    ) -> Result<()> {
        let req = semver::VersionReq::parse(dep_version)
            .map_err(|e| ModuleError::ValidationError {
                module: self.id().clone(),
                message: format!("Invalid dependency version requirement: {}", e),
            })?;

        if !req.matches(version) {
            return Err(ModuleError::VersionMismatch {
                module: self.id().clone(),
                required: dep_version.to_string(),
                actual: version.to_string(),
            });
        }

        Ok(())
    }
}
```

## Best Practices

1. **Progressive Validation**:
   ```rust
   impl ModuleValidator {
       fn validate_progressive(&self, module: &Module) -> Result<()> {
           // Start with basic validation
           self.validate_basic(module)?;
           
           // Progress to more complex checks
           self.validate_dependencies(module)?;
           self.validate_exports(module)?;
           self.validate_imports(module)?;
           
           // Finally, perform expensive validations
           self.validate_types(module)?;
           
           Ok(())
       }
   }
   ```

2. **Early Validation**:
   ```rust
   impl Module {
       pub fn new_validated(id: ModuleId, validator: &dyn ModuleValidator) -> Result<Self> {
           let module = Self::new(id);
           validator.validate(&module)?;
           Ok(module)
       }
   }
   ```

3. **Validation Caching**:
   ```rust
   impl ModuleValidator {
       fn validate_with_cache(&self, module: &Module) -> Result<()> {
           let cache_key = self.compute_validation_cache_key(module);
           if let Some(result) = self.validation_cache.get(&cache_key) {
               return result.clone();
           }
           
           let result = self.validate(module);
           self.validation_cache.insert(cache_key, result.clone());
           result
       }
   }
   ``` 