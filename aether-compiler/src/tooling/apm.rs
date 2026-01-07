//! Aether Package Manager (APM)
//! Simple git-based dependency management

use std::process::Command;
use std::path::Path;

pub struct Apm;

impl Apm {
    /// Install a package from git URL
    pub fn install(url: &str) {
        println!("📦 APM: Installing package from {}", url);
        
        // Extract repo name
        let name = url.split('/').last().unwrap_or("package")
            .trim_end_matches(".git");
            
        let target_dir = Path::new("deps").join(name);
        
        if target_dir.exists() {
            println!("   Updating existing package...");
            Command::new("git")
                .current_dir(&target_dir)
                .args(&["pull"])
                .status()
                .expect("Failed to update package");
        } else {
            println!("   Cloning new package...");
            Command::new("git")
                .args(&["clone", url, target_dir.to_str().unwrap()])
                .status()
                .expect("Failed to clone package");
        }
        
        println!("✅ Package installed: {}", name);
    }
    
    /// Initialize new project
    pub fn init(name: &str) {
        println!("📦 APM: Initializing project {}", name);
        // Create project structure
        // ...
    }
}
