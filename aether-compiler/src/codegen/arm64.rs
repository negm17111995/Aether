//! ARM64 code generation helpers

pub fn emit_prologue(asm: &mut String) {
    // Save frame pointer (x29), link register (x30)
    asm.push_str("    stp x29, x30, [sp, #-16]!\n");
    // Save callee-saved registers x19-x28 (we use x19-x21 extensively)
    // Save them in pairs for efficiency
    asm.push_str("    stp x19, x20, [sp, #-16]!\n");
    asm.push_str("    stp x21, x22, [sp, #-16]!\n");
    asm.push_str("    stp x23, x24, [sp, #-16]!\n");
    asm.push_str("    stp x25, x26, [sp, #-16]!\n");
    asm.push_str("    stp x27, x28, [sp, #-16]!\n");
    // Allocate local variable space (256 bytes)
    asm.push_str("    sub sp, sp, #256\n");
    // Set frame pointer AFTER all saves and local allocation
    // This way [x29, #-8] etc point to actual local variable space
    asm.push_str("    add x29, sp, #256\n");
}

pub fn emit_epilogue(asm: &mut String, _stack_size: i32) {
    // Deallocate local variable space
    asm.push_str("    add sp, sp, #256\n");
    // Restore callee-saved registers (in reverse order)
    asm.push_str("    ldp x27, x28, [sp], #16\n");
    asm.push_str("    ldp x25, x26, [sp], #16\n");
    asm.push_str("    ldp x23, x24, [sp], #16\n");
    asm.push_str("    ldp x21, x22, [sp], #16\n");
    asm.push_str("    ldp x19, x20, [sp], #16\n");
    // Restore frame pointer and link register
    asm.push_str("    ldp x29, x30, [sp], #16\n");
    asm.push_str("    ret\n");
}

pub fn emit_mov_imm(asm: &mut String, reg: &str, val: i64) {
    if val >= 0 && val <= 65535 {
        asm.push_str(&format!("    mov {}, #{}\n", reg, val));
    } else if val < 0 && val >= -65536 {
        asm.push_str(&format!("    mov {}, #{}\n", reg, val));
    } else {
        // Need movz/movk for larger values
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
