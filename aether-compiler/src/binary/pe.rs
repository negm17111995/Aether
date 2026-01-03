//! PE Binary Format (Windows)

use std::path::Path;
use anyhow::Result;

/// Write PE binary (stub - needs Windows toolchain)
pub fn write(path: &Path, code: &[u8]) -> Result<()> {
    // Write assembly file
    let asm_path = path.with_extension("s");
    std::fs::write(&asm_path, code)?;
    
    // On Windows, would use ml64 and link
    // For now just create the asm file
    
    anyhow::bail!("Windows PE output requires Windows toolchain (ml64/link)")
}
