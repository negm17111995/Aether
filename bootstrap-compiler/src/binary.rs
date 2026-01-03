// AETHER BINARY - Uses system assembler and linker for proper binary generation

use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn write_macho(path: &str, code: &[u8]) {
    // Create temporary assembly file
    let asm_path = format!("{}.s", path);
    let obj_path = format!("{}.o", path);
    
    // Write assembly with code as hex bytes
    let mut asm = File::create(&asm_path).expect("Failed to create asm file");
    
    writeln!(asm, ".global _main").unwrap();
    writeln!(asm, ".align 4").unwrap();
    writeln!(asm, "_main:").unwrap();
    
    // Emit code as .word directives (4 bytes each for ARM64 instructions)
    for chunk in code.chunks(4) {
        if chunk.len() == 4 {
            let inst = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            writeln!(asm, "    .word 0x{:08X}", inst).unwrap();
        }
    }
    
    drop(asm);
    
    // Assemble using xcrun
    let as_result = Command::new("xcrun")
        .args(["as", "-o", &obj_path, &asm_path])
        .output();
    
    if let Err(e) = as_result {
        eprintln!("Failed to run assembler: {}", e);
        fallback_write(path, code);
        return;
    }
    
    let as_output = as_result.unwrap();
    if !as_output.status.success() {
        eprintln!("Assembler failed: {}", String::from_utf8_lossy(&as_output.stderr));
        fallback_write(path, code);
        return;
    }
    
    // Link using xcrun with SDK
    let ld_result = Command::new("xcrun")
        .args([
            "ld",
            "-o", path,
            "-e", "_main",
            "-lSystem",
            "-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib",
            "-syslibroot", "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk",
            &obj_path,
        ])
        .output();
    
    if let Err(e) = ld_result {
        eprintln!("Failed to run linker: {}", e);
        fallback_write(path, code);
        return;
    }
    
    let ld_output = ld_result.unwrap();
    if !ld_output.status.success() {
        eprintln!("Linker failed: {}", String::from_utf8_lossy(&ld_output.stderr));
        fallback_write(path, code);
        return;
    }
    
    // Clean up temp files
    let _ = std::fs::remove_file(&asm_path);
    let _ = std::fs::remove_file(&obj_path);
    
    println!("      Assembled and linked successfully");
}

// Fallback to direct binary write
fn fallback_write(path: &str, code: &[u8]) {
    println!("      Using direct binary write (may require manual codesigning)");
    
    let mut file = File::create(path).expect("Failed to create output file");
    
    let page_size: u64 = 0x4000;
    let code_size = code.len() as u64;
    let pagezero_size: u64 = 0x100000000;
    let text_vmaddr: u64 = 0x100000000;
    let text_vmsize: u64 = page_size * 2;
    let code_fileoff: u64 = page_size;
    
    let mach_header_size: u32 = 32;
    let segment_cmd_size: u32 = 72;
    let section_cmd_size: u32 = 80;
    let main_cmd_size: u32 = 24;
    
    let ncmds: u32 = 3;
    let sizeofcmds: u32 = segment_cmd_size + (segment_cmd_size + section_cmd_size) + main_cmd_size;
    
    let mut data = Vec::new();
    
    // Mach-O Header
    data.extend_from_slice(&0xFEEDFACFu32.to_le_bytes());
    data.extend_from_slice(&0x0100000Cu32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&2u32.to_le_bytes());
    data.extend_from_slice(&ncmds.to_le_bytes());
    data.extend_from_slice(&sizeofcmds.to_le_bytes());
    data.extend_from_slice(&0x00200001u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    
    // __PAGEZERO
    data.extend_from_slice(&0x19u32.to_le_bytes());
    data.extend_from_slice(&segment_cmd_size.to_le_bytes());
    data.extend_from_slice(b"__PAGEZERO\0\0\0\0\0\0");
    data.extend_from_slice(&0u64.to_le_bytes());
    data.extend_from_slice(&pagezero_size.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    
    // __TEXT
    data.extend_from_slice(&0x19u32.to_le_bytes());
    data.extend_from_slice(&(segment_cmd_size + section_cmd_size).to_le_bytes());
    data.extend_from_slice(b"__TEXT\0\0\0\0\0\0\0\0\0\0");
    data.extend_from_slice(&text_vmaddr.to_le_bytes());
    data.extend_from_slice(&text_vmsize.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    data.extend_from_slice(&text_vmsize.to_le_bytes());
    data.extend_from_slice(&5u32.to_le_bytes());
    data.extend_from_slice(&5u32.to_le_bytes());
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    
    // __text section
    data.extend_from_slice(b"__text\0\0\0\0\0\0\0\0\0\0");
    data.extend_from_slice(b"__TEXT\0\0\0\0\0\0\0\0\0\0");
    data.extend_from_slice(&(text_vmaddr + code_fileoff).to_le_bytes());
    data.extend_from_slice(&code_size.to_le_bytes());
    data.extend_from_slice(&(code_fileoff as u32).to_le_bytes());
    data.extend_from_slice(&2u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0x80000400u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    
    // LC_MAIN
    data.extend_from_slice(&0x80000028u32.to_le_bytes());
    data.extend_from_slice(&main_cmd_size.to_le_bytes());
    data.extend_from_slice(&code_fileoff.to_le_bytes());
    data.extend_from_slice(&0u64.to_le_bytes());
    
    while data.len() < page_size as usize {
        data.push(0);
    }
    
    data.extend_from_slice(code);
    
    while data.len() < (page_size * 2) as usize {
        data.push(0);
    }
    
    file.write_all(&data).expect("Failed to write output file");
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms).unwrap();
    }
}
