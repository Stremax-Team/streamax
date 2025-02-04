use crate::core::{Result, Error};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Workspace configuration
#[derive(Serialize, Deserialize)]
pub struct WorkspaceConfig {
    name: String,
    version: String,
    members: Vec<String>,
    default_members: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    metadata: Option<WorkspaceMetadata>,
}

/// Workspace metadata
#[derive(Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    authors: Vec<String>,
    description: Option<String>,
    documentation: Option<String>,
    repository: Option<String>,
    license: Option<String>,
}

/// Workspace manager
pub struct WorkspaceManager {
    config: WorkspaceConfig,
    root_path: PathBuf,
    members: HashMap<String, WorkspaceMember>,
    virtual_packages: HashMap<String, VirtualPackage>,
}

/// Workspace member
pub struct WorkspaceMember {
    name: String,
    path: PathBuf,
    manifest: Manifest,
    dependencies: HashSet<String>,
}

/// Virtual package for workspace-level dependencies
pub struct VirtualPackage {
    name: String,
    version: semver::Version,
    dependencies: HashMap<String, Dependency>,
}

impl WorkspaceManager {
    pub fn new(root_path: PathBuf) -> Result<Self> {
        // Load workspace config
        let config_path = root_path.join("Workspace.toml");
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            toml::from_str(&content)?
        } else {
            WorkspaceConfig {
                name: root_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("workspace")
                    .to_string(),
                version: "0.1.0".to_string(),
                members: vec!["*".to_string()],
                default_members: None,
                exclude: None,
                metadata: None,
            }
        };
        
        Ok(WorkspaceManager {
            config,
            root_path,
            members: HashMap::new(),
            virtual_packages: HashMap::new(),
        })
    }
    
    pub fn load(&mut self) -> Result<()> {
        // 1. Discover members
        let members = self.discover_members()?;
        
        // 2. Load member manifests
        for member_path in members {
            self.load_member(&member_path)?;
        }
        
        // 3. Create virtual packages
        self.create_virtual_packages()?;
        
        // 4. Validate workspace
        self.validate()?;
        
        Ok(())
    }
    
    pub fn build(&self, targets: Option<Vec<String>>) -> Result<()> {
        let targets = targets.unwrap_or_else(|| {
            self.config.default_members
                .clone()
                .unwrap_or_else(|| self.members.keys().cloned().collect())
        });
        
        for target in targets {
            if let Some(member) = self.members.get(&target) {
                self.build_member(member)?;
            }
        }
        
        Ok(())
    }
    
    pub fn test(&self, targets: Option<Vec<String>>) -> Result<()> {
        let targets = targets.unwrap_or_else(|| {
            self.config.default_members
                .clone()
                .unwrap_or_else(|| self.members.keys().cloned().collect())
        });
        
        for target in targets {
            if let Some(member) = self.members.get(&target) {
                self.test_member(member)?;
            }
        }
        
        Ok(())
    }
    
    pub fn add_member(&mut self, path: PathBuf) -> Result<()> {
        // 1. Validate path
        if !path.exists() {
            return Err(Error::PathNotFound);
        }
        
        // 2. Load manifest
        self.load_member(&path)?;
        
        // 3. Update workspace config
        let relative_path = path.strip_prefix(&self.root_path)?;
        self.config.members.push(relative_path.to_string_lossy().into_owned());
        
        // 4. Save workspace config
        self.save_config()?;
        
        Ok(())
    }
    
    pub fn remove_member(&mut self, name: &str) -> Result<()> {
        // 1. Remove from members
        if let Some(member) = self.members.remove(name) {
            // 2. Update workspace config
            let relative_path = member.path.strip_prefix(&self.root_path)?;
            self.config.members.retain(|m| m != &relative_path.to_string_lossy());
            
            // 3. Save workspace config
            self.save_config()?;
        }
        
        Ok(())
    }
    
    // Private methods
    
    fn discover_members(&self) -> Result<Vec<PathBuf>> {
        let mut members = Vec::new();
        
        for pattern in &self.config.members {
            let glob_pattern = self.root_path.join(pattern).to_string_lossy().into_owned();
            for entry in glob::glob(&glob_pattern)? {
                if let Ok(path) = entry {
                    if path.join("Package.toml").exists() {
                        members.push(path);
                    }
                }
            }
        }
        
        // Apply exclusions
        if let Some(exclude) = &self.config.exclude {
            for pattern in exclude {
                let glob_pattern = self.root_path.join(pattern).to_string_lossy().into_owned();
                for entry in glob::glob(&glob_pattern)? {
                    if let Ok(path) = entry {
                        members.retain(|m| m != &path);
                    }
                }
            }
        }
        
        Ok(members)
    }
    
    fn load_member(&mut self, path: &Path) -> Result<()> {
        let manifest_path = path.join("Package.toml");
        let manifest = Manifest::load(&manifest_path)?;
        
        let name = manifest.name.clone();
        let dependencies = manifest.dependencies.keys().cloned().collect();
        
        self.members.insert(name.clone(), WorkspaceMember {
            name,
            path: path.to_path_buf(),
            manifest,
            dependencies,
        });
        
        Ok(())
    }
    
    fn create_virtual_packages(&mut self) -> Result<()> {
        // Create virtual packages for workspace-level dependencies
        for member in self.members.values() {
            for (name, dep) in &member.manifest.dependencies {
                if !self.members.contains_key(name) && !self.virtual_packages.contains_key(name) {
                    match dep {
                        Dependency::Version(req) => {
                            self.virtual_packages.insert(name.clone(), VirtualPackage {
                                name: name.clone(),
                                version: req.to_string().parse()?,
                                dependencies: HashMap::new(),
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn validate(&self) -> Result<()> {
        // Check for dependency cycles
        for member in self.members.values() {
            let mut visited = HashSet::new();
            self.check_cycles(&member.name, &mut visited)?;
        }
        
        Ok(())
    }
    
    fn check_cycles(&self, name: &str, visited: &mut HashSet<String>) -> Result<()> {
        if !visited.insert(name.to_string()) {
            return Err(Error::DependencyCycle);
        }
        
        if let Some(member) = self.members.get(name) {
            for dep in &member.dependencies {
                self.check_cycles(dep, visited)?;
            }
        }
        
        visited.remove(name);
        Ok(())
    }
    
    fn build_member(&self, member: &WorkspaceMember) -> Result<()> {
        // Build member package
        unimplemented!()
    }
    
    fn test_member(&self, member: &WorkspaceMember) -> Result<()> {
        // Run tests for member package
        unimplemented!()
    }
    
    fn save_config(&self) -> Result<()> {
        let config_path = self.root_path.join("Workspace.toml");
        let content = toml::to_string_pretty(&self.config)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }
} 