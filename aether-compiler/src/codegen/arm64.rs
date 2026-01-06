//! ARM64 code generation helpers - OPTIMIZED AND STABLE

/// Emit optimized function prologue
/// Saves x19, x20 for temps, minimal stack allocation
pub fn emit_prologue(asm: &mut String) {
    // Save fp, lr, x19, x20 in one operation (32 bytes)
    asm.push_str("    stp x29, x30, [sp, #-32]!\n");
    asm.push_str("    stp x19, x20, [sp, #16]\n");
    asm.push_str("    mov x29, sp\n");
    // Allocate minimal space for locals
    asm.push_str("    sub sp, sp, #32\n");
}

/// Emit optimized function epilogue  
pub fn emit_epilogue(asm: &mut String, _stack_size: i32) {
    asm.push_str("    add sp, sp, #32\n");
    asm.push_str("    ldp x19, x20, [sp, #16]\n");
    asm.push_str("    ldp x29, x30, [sp], #32\n");
    asm.push_str("    ret\n");
}

/// Emit leaf function prologue
pub fn emit_leaf_prologue(asm: &mut String) {
    asm.push_str("    stp x29, x30, [sp, #-16]!\n");
    asm.push_str("    mov x29, sp\n");
}

/// Emit leaf function epilogue
pub fn emit_leaf_epilogue(asm: &mut String) {
    asm.push_str("    ldp x29, x30, [sp], #16\n");
    asm.push_str("    ret\n");
}

pub fn emit_mov_imm(asm: &mut String, reg: &str, val: i64) {
    if val >= 0 && val <= 65535 {
        asm.push_str(&format!("    mov {}, #{}\n", reg, val));
    } else if val < 0 && val >= -65536 {
        asm.push_str(&format!("    mov {}, #{}\n", reg, val));
    } else {
        asm.push_str(&format!("    movz {}, #{}\n", reg, (val & 0xFFFF) as u16));
        if val > 65535 || val < 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #16\n", reg, ((val >> 16) & 0xFFFF) as u16));
        }
    }
}

pub fn emit_load_local(asm: &mut String, reg: &str, offset: i32) {
    asm.push_str(&format!("    ldur {}, [x29, #-{}]\n", reg, offset));
}

pub fn emit_store_local(asm: &mut String, reg: &str, offset: i32) {
    asm.push_str(&format!("    stur {}, [x29, #-{}]\n", reg, offset));
}
