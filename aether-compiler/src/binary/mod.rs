//! Aether Binary Output - Platform-specific binary formats
//!
//! ELF (Linux), Mach-O (macOS), PE (Windows)

pub mod elf;
pub mod macho;
pub mod pe;

use std::path::Path;
use anyhow::{anyhow, Result};

/// Write binary file in appropriate format for target
pub fn write(path: &Path, code: &[u8], target: &str) -> Result<()> {
    if target.contains("linux") {
        elf::write(path, code)
    } else if target.contains("darwin") || target.contains("apple") {
        macho::write(path, code)
    } else if target.contains("windows") {
        pe::write(path, code)
    } else {
        // Default: write assembly for external assembler
        write_asm(path, code)
    }
}

/// Write assembly file for external assembler/linker
fn write_asm(path: &Path, code: &[u8]) -> Result<()> {
    let asm_path = path.with_extension("s");
    std::fs::write(&asm_path, code)?;
    
    // Try to assemble with xcrun/as
    #[cfg(target_os = "macos")]
    {
        let obj_path = path.with_extension("o");
        let status = std::process::Command::new("xcrun")
            .args(["as", "-o"])
            .arg(&obj_path)
            .arg(&asm_path)
            .status()?;
        
        if status.success() {
            // Link
            let status = std::process::Command::new("xcrun")
                .args(["ld", "-o"])
                .arg(path)
                .args(["-e", "_main", "-lSystem"])
                .args(["-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib"])
                .args(["-syslibroot", "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"])
                .arg(&obj_path)
                .status()?;
            
            if !status.success() {
                return Err(anyhow!("Linker failed"));
            }
            
            // Cleanup
            let _ = std::fs::remove_file(&obj_path);
            let _ = std::fs::remove_file(&asm_path);
        } else {
            return Err(anyhow!("Assembler failed"));
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        let obj_path = path.with_extension("o");
        let status = std::process::Command::new("as")
            .args(["-o"])
            .arg(&obj_path)
            .arg(&asm_path)
            .status()?;
        
        if status.success() {
            let status = std::process::Command::new("ld")
                .args(["-o"])
                .arg(path)
                .arg(&obj_path)
                .args(["-lc", "-dynamic-linker", "/lib64/ld-linux-x86-64.so.2"])
                .status()?;
            
            if !status.success() {
                return Err(anyhow!("Linker failed"));
            }
        }
    }
    
    Ok(())
}
