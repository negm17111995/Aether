//! Mach-O Binary Format (macOS)

use std::path::Path;
use anyhow::Result;

/// Write Mach-O binary
pub fn write(path: &Path, code: &[u8]) -> Result<()> {
    let asm_path = path.with_extension("s");
    std::fs::write(&asm_path, code)?;
    
    let obj_path = path.with_extension("o");
    
    // Assemble with xcrun
    let status = std::process::Command::new("xcrun")
        .args(["as", "-o"])
        .arg(&obj_path)
        .arg(&asm_path)
        .status()?;
    
    if status.success() {
        // Link with xcrun
        let status = std::process::Command::new("xcrun")
            .args(["ld", "-o"])
            .arg(path)
            .args(["-e", "_main", "-lSystem"])
            .args(["-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib"])
            .args(["-syslibroot", "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"])
            .arg(&obj_path)
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
