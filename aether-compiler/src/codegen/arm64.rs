/// Emit adaptive function prologue
/// Saves minimal registers and allocates exact stack needed
pub fn emit_adaptive_prologue(asm: &mut String, regs_used: u32, locals_size: i32) {
    // Calculate save area size
    // Base: x29, x30, x19, x20 = 4 registers (32 bytes)
    // Extra pairs: (regs_used + 1) / 2 * 16 bytes
    // e.g. regs_used=1 (x21) -> 1 pair (x21,x22) -> +16 bytes -> Total 48
    let extra_pairs = (regs_used + 1) / 2;
    let save_area_size = 32 + (extra_pairs * 16);
    
    // Total frame (must be 16-byte aligned)
    let total_frame = save_area_size + (locals_size as u32);
    let total_frame = (total_frame + 15) & !15; // Align up to 16
    
    // Allocate stack and save FP/LR
    asm.push_str(&format!("    stp x29, x30, [sp, #-{}]!\n", total_frame));
    asm.push_str("    stp x19, x20, [sp, #16]\n");
    
    // Save extra registers
    if regs_used > 0 {
        asm.push_str("    stp x21, x22, [sp, #32]\n");
    }
    if regs_used > 2 {
        asm.push_str("    stp x23, x24, [sp, #48]\n");
    }
    if regs_used > 4 {
        asm.push_str("    stp x25, x26, [sp, #64]\n");
    }
    if regs_used > 6 {
        asm.push_str("    stp x27, x28, [sp, #80]\n");
    }
    
    asm.push_str("    mov x29, sp\n");
}

/// Emit adaptive function epilogue
pub fn emit_adaptive_epilogue(asm: &mut String, locals_size: i32, regs_used: u32) {
    // Re-calculate frame size to deallocate correctly
    let extra_pairs = (regs_used + 1) / 2;
    let save_area_size = 32 + (extra_pairs * 16);
    let total_frame = save_area_size + (locals_size as u32);
    let total_frame = (total_frame + 15) & !15;

    // Restore registers
    if regs_used > 6 {
        asm.push_str("    ldp x27, x28, [sp, #80]\n");
    }
    if regs_used > 4 {
        asm.push_str("    ldp x25, x26, [sp, #64]\n");
    }
    if regs_used > 2 {
        asm.push_str("    ldp x23, x24, [sp, #48]\n");
    }
    if regs_used > 0 {
        asm.push_str("    ldp x21, x22, [sp, #32]\n");
    }
    
    asm.push_str("    ldp x19, x20, [sp, #16]\n");
    asm.push_str(&format!("    ldp x29, x30, [sp], #{}\n", total_frame));
    asm.push_str("    ret\n");
}

pub fn emit_prologue(_asm: &mut String) {
    // Deprecated for internal use, kept for compatibility if needed
    panic!("Use adaptive prologue");
}

pub fn emit_epilogue(_asm: &mut String, _stack: i32) {
    panic!("Use adaptive epilogue");
}


/// Emit leaf function prologue (no calls, minimal overhead)
pub fn emit_leaf_prologue(asm: &mut String) {
    asm.push_str("    stp x29, x30, [sp, #-16]!\n");
    asm.push_str("    mov x29, sp\n");
}

/// Emit leaf function epilogue
pub fn emit_leaf_epilogue(asm: &mut String) {
    asm.push_str("    ldp x29, x30, [sp], #16\n");
    asm.push_str("    ret\n");
}

/// Emit recursive function prologue (minimal for speed)
pub fn emit_recursive_prologue(asm: &mut String) {
    // Ultra-minimal for recursive calls - just save lr and fp
    asm.push_str("    stp x29, x30, [sp, #-32]!\n");
    asm.push_str("    stp x19, x20, [sp, #16]\n");
    asm.push_str("    mov x29, sp\n");
}

/// Emit recursive function epilogue
pub fn emit_recursive_epilogue(asm: &mut String) {
    asm.push_str("    ldp x19, x20, [sp, #16]\n");
    asm.push_str("    ldp x29, x30, [sp], #32\n");
    asm.push_str("    ret\n");
}

pub fn emit_mov_imm(asm: &mut String, reg: &str, val: i64) {
    if val >= 0 && val <= 65535 {
        asm.push_str(&format!("    mov {}, #{}\n", reg, val));
    } else if val < 0 && val >= -65536 {
        asm.push_str(&format!("    mov {}, #{}\n", reg, val));
    } else {
        // Handle large immediates with movz/movk sequence
        asm.push_str(&format!("    movz {}, #{}\n", reg, (val & 0xFFFF) as u16));
        if (val >> 16) & 0xFFFF != 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #16\n", reg, ((val >> 16) & 0xFFFF) as u16));
        }
        if (val >> 32) & 0xFFFF != 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #32\n", reg, ((val >> 32) & 0xFFFF) as u16));
        }
        if (val >> 48) & 0xFFFF != 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #48\n", reg, ((val >> 48) & 0xFFFF) as u16));
        }
    }
}

pub fn emit_load_local(asm: &mut String, reg: &str, offset: i32) {
    asm.push_str(&format!("    ldur {}, [x29, #-{}]\n", reg, offset));
}

pub fn emit_store_local(asm: &mut String, reg: &str, offset: i32) {
    asm.push_str(&format!("    stur {}, [x29, #-{}]\n", reg, offset));
}

/// Emit optimized loop structure
pub fn emit_loop_header(asm: &mut String, counter_reg: &str, limit_reg: &str, end_label: &str) {
    asm.push_str(&format!("    cmp {}, {}\n", counter_reg, limit_reg));
    asm.push_str(&format!("    b.ge {}\n", end_label));
}

/// Emit optimized increment
pub fn emit_increment(asm: &mut String, reg: &str) {
    asm.push_str(&format!("    add {}, {}, #1\n", reg, reg));
}
