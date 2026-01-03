//! Aether Stdlib - Standard library module loader

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::Result;

/// Standard library module loader
pub struct StdlibLoader {
    /// Root path to stdlib
    root: PathBuf,
    /// Cached module contents
    cache: HashMap<String, String>,
}

impl StdlibLoader {
    pub fn new(root: PathBuf) -> Self {
        StdlibLoader {
            root,
            cache: HashMap::new(),
        }
    }
    
    /// Resolve import path to file
    pub fn resolve(&self, path: &[String]) -> Option<PathBuf> {
        let mut file_path = self.root.clone();
        for component in path {
            file_path.push(component);
        }
        file_path.set_extension("aether");
        
        if file_path.exists() {
            Some(file_path)
        } else {
            None
        }
    }
    
    /// Load module source
    pub fn load(&mut self, path: &[String]) -> Result<String> {
        let key = path.join(".");
        
        if let Some(cached) = self.cache.get(&key) {
            return Ok(cached.clone());
        }
        
        if let Some(file_path) = self.resolve(path) {
            let content = std::fs::read_to_string(&file_path)?;
            self.cache.insert(key, content.clone());
            Ok(content)
        } else {
            anyhow::bail!("Module not found: {}", key)
        }
    }
    
    /// List all available modules
    pub fn list_modules(&self) -> Vec<String> {
        let mut modules = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.root) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".aether") {
                        modules.push(name.trim_end_matches(".aether").to_string());
                    }
                }
            }
        }
        modules
    }
}

/// Default stdlib paths
pub fn stdlib_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    // Relative to executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            paths.push(parent.join("stdlib"));
            paths.push(parent.join("../stdlib"));
        }
    }
    
    // Current directory
    paths.push(PathBuf::from("stdlib"));
    
    // Environment variable
    if let Ok(path) = std::env::var("AETHER_STDLIB") {
        paths.push(PathBuf::from(path));
    }
    
    paths
}

/// Find stdlib root
pub fn find_stdlib() -> Option<PathBuf> {
    for path in stdlib_paths() {
        if path.exists() && path.is_dir() {
            return Some(path);
        }
    }
    None
}
