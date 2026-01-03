//! x86-64 code generation helpers

pub fn emit_prologue(asm: &mut String) {
    asm.push_str("    push rbp\n");
    asm.push_str("    mov rbp, rsp\n");
    asm.push_str("    sub rsp, 256\n");
}

pub fn emit_epilogue(asm: &mut String, _stack_size: i32) {
    asm.push_str("    add rsp, 256\n");
    asm.push_str("    pop rbp\n");
    asm.push_str("    ret\n");
}

pub fn emit_mov_imm(asm: &mut String, reg: &str, val: i64) {
    if val == 0 {
        asm.push_str(&format!("    xor {}, {}\n", reg, reg));
    } else {
        asm.push_str(&format!("    mov {}, {}\n", reg, val));
    }
}

pub fn emit_load_local(asm: &mut String, reg: &str, offset: i32) {
    asm.push_str(&format!("    mov {}, [rbp - {}]\n", reg, offset));
}

pub fn emit_store_local(asm: &mut String, reg: &str, offset: i32) {
    asm.push_str(&format!("    mov [rbp - {}], {}\n", offset, reg));
}
