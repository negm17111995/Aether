//! ELF Binary Format (Linux)

use std::path::Path;
use anyhow::Result;

/// Write ELF binary
pub fn write(path: &Path, code: &[u8]) -> Result<()> {
    // For now, write assembly and use system linker
    let asm_path = path.with_extension("s");
    std::fs::write(&asm_path, code)?;
    
    let obj_path = path.with_extension("o");
    
    // Assemble
    let status = std::process::Command::new("as")
        .arg("-o")
        .arg(&obj_path)
        .arg(&asm_path)
        .status()?;
    
    if status.success() {
        // Link
        let status = std::process::Command::new("ld")
            .arg("-o")
            .arg(path)
            .arg(&obj_path)
            .args(["-lc", "-dynamic-linker", "/lib64/ld-linux-x86-64.so.2"])
            .status()?;
        
        // Cleanup
        let _ = std::fs::remove_file(&obj_path);
        let _ = std::fs::remove_file(&asm_path);
        
        if !status.success() {
            anyhow::bail!("Linker failed");
        }
    } else {
        anyhow::bail!("Assembler failed");
    }
    
    Ok(())
}
