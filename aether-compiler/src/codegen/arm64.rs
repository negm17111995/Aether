//! ARM64 code generation helpers

pub fn emit_prologue(asm: &mut String) {
    asm.push_str("    stp x29, x30, [sp, #-16]!\n");
    asm.push_str("    mov x29, sp\n");
    asm.push_str("    sub sp, sp, #256\n");
}

pub fn emit_epilogue(asm: &mut String, _stack_size: i32) {
    asm.push_str("    add sp, sp, #256\n");
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
