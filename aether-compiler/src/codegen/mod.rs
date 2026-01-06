//! Aether Code Generator - Native code generation
//!
//! Multi-target: ARM64, x86-64, with optimizations

pub mod arm64;
pub mod x86_64;

use crate::ast::*;
use crate::typechecker::TypedModule;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Code generation context
pub struct CodeGen {
    /// Target triple
    target: String,
    /// Generated code buffer
    code: Vec<u8>,
    /// Assembly output
    asm: String,
    /// Function offsets
    func_offsets: HashMap<String, usize>,
    /// Label counter
    label_counter: usize,
    /// Local variable offsets (stack-based)
    locals: HashMap<String, i32>,
    /// Constant values (name -> value)
    constants: HashMap<String, i64>,
    /// Stack of loop end labels for break statements
    loop_end_labels: Vec<String>,
    /// Current stack offset
    stack_offset: i32,
    /// Optimization level
    opt_level: u8,
    /// Current function name
    current_func: String,
    /// Functions for inlining
    inline_candidates: HashMap<String, usize>,
    /// Register-allocated variables (name -> register like "x21")
    /// Uses x21-x28 for hot loop variables
    register_vars: HashMap<String, String>,
    /// Next available register for allocation (starts at x21)
    next_reg: usize,
}

impl CodeGen {
    pub fn new(target: &str, opt_level: u8) -> Self {
        CodeGen {
            target: target.to_string(),
            code: Vec::new(),
            asm: String::new(),
            func_offsets: HashMap::new(),
            label_counter: 0,
            locals: HashMap::new(),
            constants: HashMap::new(),
            loop_end_labels: Vec::new(),
            stack_offset: 0,
            opt_level,
            current_func: String::new(),
            inline_candidates: HashMap::new(),
            register_vars: HashMap::new(),
            next_reg: 21,  // Start with x21, x19-x20 reserved for temps
        }
    }
    
    fn new_label(&mut self) -> String {
        let label = format!(".L{}", self.label_counter);
        self.label_counter += 1;
        label
    }
    
    fn emit_asm(&mut self, line: &str) {
        self.asm.push_str(line);
        self.asm.push('\n');
    }
    
    fn alloc_local(&mut self, name: &str) -> i32 {
        self.stack_offset += 8;
        self.locals.insert(name.to_string(), self.stack_offset);
        self.stack_offset
    }
    
    fn get_local(&self, name: &str) -> Option<i32> {
        self.locals.get(name).copied()
    }
    
    /// Allocate a register for a hot variable (loop counter, accumulator)
    /// Returns register name like "x21" or None if out of registers
    fn alloc_register(&mut self, name: &str) -> Option<String> {
        if self.next_reg <= 28 && self.is_arm64() {
            let reg = format!("x{}", self.next_reg);
            self.register_vars.insert(name.to_string(), reg.clone());
            self.next_reg += 1;
            Some(reg)
        } else {
            None
        }
    }
    
    /// Get register for a variable, if one was allocated
    fn get_register(&self, name: &str) -> Option<&String> {
        self.register_vars.get(name)
    }
    
    fn is_arm64(&self) -> bool {
        self.target.contains("aarch64") || self.target.contains("arm64")
    }
    
    fn is_x86_64(&self) -> bool {
        self.target.contains("x86_64") || self.target.contains("x86-64")
    }
    
    /// Try to evaluate an expression at compile time (constant folding)
    fn try_const_fold(&self, expr: &Expr) -> Option<i64> {
        match expr {
            Expr::Int(val, _) => Some(*val),
            Expr::Bool(val, _) => Some(if *val { 1 } else { 0 }),
            Expr::Ident(name, _) => self.constants.get(name).copied(),
            Expr::Binary(op, left, right, _) => {
                let l = self.try_const_fold(left)?;
                let r = self.try_const_fold(right)?;
                Some(match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div if r != 0 => l / r,
                    BinOp::Mod if r != 0 => l % r,
                    BinOp::Eq => if l == r { 1 } else { 0 },
                    BinOp::Ne => if l != r { 1 } else { 0 },
                    BinOp::Lt => if l < r { 1 } else { 0 },
                    BinOp::Le => if l <= r { 1 } else { 0 },
                    BinOp::Gt => if l > r { 1 } else { 0 },
                    BinOp::Ge => if l >= r { 1 } else { 0 },
                    BinOp::And => if l != 0 && r != 0 { 1 } else { 0 },
                    BinOp::Or => if l != 0 || r != 0 { 1 } else { 0 },
                    BinOp::BitAnd => l & r,
                    BinOp::BitOr => l | r,
                    BinOp::BitXor => l ^ r,
                    _ => return None,
                })
            }
            _ => None,
        }
    }
    
    /// Generate code for expression, result in r0/rax
    fn gen_expr(&mut self, expr: &Expr) {
        // OPTIMIZATION: Try constant folding first
        if let Some(val) = self.try_const_fold(expr) {
            if self.is_arm64() {
                arm64::emit_mov_imm(&mut self.asm, "x0", val);
            } else {
                x86_64::emit_mov_imm(&mut self.asm, "rax", val);
            }
            return;
        }
        
        match expr {
            Expr::Int(val, _) => {
                if self.is_arm64() {
                    arm64::emit_mov_imm(&mut self.asm, "x0", *val);
                } else {
                    x86_64::emit_mov_imm(&mut self.asm, "rax", *val);
                }
            }
            
            Expr::Bool(val, _) => {
                let v = if *val { 1 } else { 0 };
                if self.is_arm64() {
                    arm64::emit_mov_imm(&mut self.asm, "x0", v);
                } else {
                    x86_64::emit_mov_imm(&mut self.asm, "rax", v);
                }
            }
            
            Expr::Ident(name, _) => {
                // OPTIMIZATION: Check if variable is in a register first (fastest)
                if let Some(reg) = self.get_register(name) {
                    if self.is_arm64() {
                        self.emit_asm(&format!("    mov x0, {}", reg));
                    }
                } else if let Some(off) = self.get_local(name) {
                    if self.is_arm64() {
                        arm64::emit_load_local(&mut self.asm, "x0", off);
                    } else {
                        x86_64::emit_load_local(&mut self.asm, "rax", off);
                    }
                } else if let Some(val) = self.constants.get(name) {
                    // It's a constant - emit its value directly
                    if self.is_arm64() {
                        arm64::emit_mov_imm(&mut self.asm, "x0", *val);
                    } else {
                        x86_64::emit_mov_imm(&mut self.asm, "rax", *val);
                    }
                } else {
                    // Unknown identifier
                    if self.is_arm64() {
                        arm64::emit_mov_imm(&mut self.asm, "x0", 0);
                    } else {
                        x86_64::emit_mov_imm(&mut self.asm, "rax", 0);
                    }
                }
            }
            
            Expr::Binary(op, left, right, _) => {
                // OPTIMIZATION: For comparisons with small constants, use immediate form
                if self.is_arm64() {
                    if let Some(rval) = self.try_const_fold(right) {
                        if rval >= 0 && rval < 4096 {
                            // Can use immediate comparison
                            self.gen_expr(left);
                            
                            match op {
                                BinOp::Lt => {
                                    self.emit_asm(&format!("    cmp x0, #{}", rval));
                                    self.emit_asm("    cset x0, lt");
                                    return;
                                }
                                BinOp::Le => {
                                    self.emit_asm(&format!("    cmp x0, #{}", rval));
                                    self.emit_asm("    cset x0, le");
                                    return;
                                }
                                BinOp::Gt => {
                                    self.emit_asm(&format!("    cmp x0, #{}", rval));
                                    self.emit_asm("    cset x0, gt");
                                    return;
                                }
                                BinOp::Ge => {
                                    self.emit_asm(&format!("    cmp x0, #{}", rval));
                                    self.emit_asm("    cset x0, ge");
                                    return;
                                }
                                BinOp::Eq => {
                                    self.emit_asm(&format!("    cmp x0, #{}", rval));
                                    self.emit_asm("    cset x0, eq");
                                    return;
                                }
                                BinOp::Ne => {
                                    self.emit_asm(&format!("    cmp x0, #{}", rval));
                                    self.emit_asm("    cset x0, ne");
                                    return;
                                }
                                BinOp::Sub => {
                                    self.emit_asm(&format!("    sub x0, x0, #{}", rval));
                                    return;
                                }
                                BinOp::Add => {
                                    self.emit_asm(&format!("    add x0, x0, #{}", rval));
                                    return;
                                }
                                // OPTIMIZATION: Use AND for modulo power-of-2
                                BinOp::Mod if rval > 0 && (rval & (rval - 1)) == 0 => {
                                    self.emit_asm(&format!("    and x0, x0, #{}", rval - 1));
                                    return;
                                }
                                // OPTIMIZATION: Use LSR for division by power-of-2
                                BinOp::Div if rval > 0 && (rval & (rval - 1)) == 0 => {
                                    let shift = 63 - (rval as u64).leading_zeros();
                                    self.emit_asm(&format!("    lsr x0, x0, #{}", shift));
                                    return;
                                }
                                // OPTIMIZATION: Use LSL for multiply by power-of-2
                                BinOp::Mul if rval > 0 && (rval & (rval - 1)) == 0 => {
                                    let shift = 63 - (rval as u64).leading_zeros();
                                    self.emit_asm(&format!("    lsl x0, x0, #{}", shift));
                                    return;
                                }
                                // OPTIMIZATION: x*3 = x + x*2 = x + (x << 1)
                                BinOp::Mul if rval == 3 => {
                                    self.emit_asm("    add x0, x0, x0, lsl #1");
                                    return;
                                }
                                // OPTIMIZATION: x*5 = x + x*4 = x + (x << 2)
                                BinOp::Mul if rval == 5 => {
                                    self.emit_asm("    add x0, x0, x0, lsl #2");
                                    return;
                                }
                                // OPTIMIZATION: x*7 = x*8 - x
                                BinOp::Mul if rval == 7 => {
                                    self.emit_asm("    lsl x1, x0, #3");
                                    self.emit_asm("    sub x0, x1, x0");
                                    return;
                                }
                                // OPTIMIZATION: x*9 = x + x*8 = x + (x << 3)
                                BinOp::Mul if rval == 9 => {
                                    self.emit_asm("    add x0, x0, x0, lsl #3");
                                    return;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                
                // Evaluate left
                self.gen_expr(left);
                
                // Save left in x19 (callee-saved, already preserved in prologue)
                if self.is_arm64() {
                    self.emit_asm("    mov x19, x0");
                } else {
                    self.emit_asm("    push rax");
                }
                
                // Evaluate right
                self.gen_expr(right);
                
                // Now x19=left, x0=right - operate directly
                if self.is_arm64() {
                    self.emit_asm("    mov x1, x0");
                    self.emit_asm("    mov x0, x19");

                    
                    match op {
                        BinOp::Add => self.emit_asm("    add x0, x0, x1"),
                        BinOp::Sub => self.emit_asm("    sub x0, x0, x1"),
                        BinOp::Mul => self.emit_asm("    mul x0, x0, x1"),
                        BinOp::Div => self.emit_asm("    sdiv x0, x0, x1"),
                        BinOp::Mod => {
                            self.emit_asm("    sdiv x2, x0, x1");
                            self.emit_asm("    msub x0, x2, x1, x0");
                        }
                        BinOp::Eq => {
                            self.emit_asm("    cmp x0, x1");
                            self.emit_asm("    cset x0, eq");
                        }
                        BinOp::Ne => {
                            self.emit_asm("    cmp x0, x1");
                            self.emit_asm("    cset x0, ne");
                        }
                        BinOp::Lt => {
                            self.emit_asm("    cmp x0, x1");
                            self.emit_asm("    cset x0, lt");
                        }
                        BinOp::Le => {
                            self.emit_asm("    cmp x0, x1");
                            self.emit_asm("    cset x0, le");
                        }
                        BinOp::Gt => {
                            self.emit_asm("    cmp x0, x1");
                            self.emit_asm("    cset x0, gt");
                        }
                        BinOp::Ge => {
                            self.emit_asm("    cmp x0, x1");
                            self.emit_asm("    cset x0, ge");
                        }
                        BinOp::And => self.emit_asm("    and x0, x0, x1"),
                        BinOp::Or => self.emit_asm("    orr x0, x0, x1"),
                        BinOp::BitAnd => self.emit_asm("    and x0, x0, x1"),
                        BinOp::BitOr => self.emit_asm("    orr x0, x0, x1"),
                        BinOp::BitXor => self.emit_asm("    eor x0, x0, x1"),
                    }
                } else {
                    self.emit_asm("    mov rcx, rax");
                    self.emit_asm("    pop rax");
                    
                    match op {
                        BinOp::Add => self.emit_asm("    add rax, rcx"),
                        BinOp::Sub => self.emit_asm("    sub rax, rcx"),
                        BinOp::Mul => self.emit_asm("    imul rax, rcx"),
                        BinOp::Div => {
                            self.emit_asm("    cqo");
                            self.emit_asm("    idiv rcx");
                        }
                        BinOp::Mod => {
                            self.emit_asm("    cqo");
                            self.emit_asm("    idiv rcx");
                            self.emit_asm("    mov rax, rdx");
                        }
                        BinOp::Eq => {
                            self.emit_asm("    cmp rax, rcx");
                            self.emit_asm("    sete al");
                            self.emit_asm("    movzx rax, al");
                        }
                        BinOp::Ne => {
                            self.emit_asm("    cmp rax, rcx");
                            self.emit_asm("    setne al");
                            self.emit_asm("    movzx rax, al");
                        }
                        BinOp::Lt => {
                            self.emit_asm("    cmp rax, rcx");
                            self.emit_asm("    setl al");
                            self.emit_asm("    movzx rax, al");
                        }
                        BinOp::Le => {
                            self.emit_asm("    cmp rax, rcx");
                            self.emit_asm("    setle al");
                            self.emit_asm("    movzx rax, al");
                        }
                        BinOp::Gt => {
                            self.emit_asm("    cmp rax, rcx");
                            self.emit_asm("    setg al");
                            self.emit_asm("    movzx rax, al");
                        }
                        BinOp::Ge => {
                            self.emit_asm("    cmp rax, rcx");
                            self.emit_asm("    setge al");
                            self.emit_asm("    movzx rax, al");
                        }
                        BinOp::And => self.emit_asm("    and rax, rcx"),
                        BinOp::Or => self.emit_asm("    or rax, rcx"),
                        BinOp::BitAnd => self.emit_asm("    and rax, rcx"),
                        BinOp::BitOr => self.emit_asm("    or rax, rcx"),
                        BinOp::BitXor => self.emit_asm("    xor rax, rcx"),
                    }
                }
            }
            
            Expr::Unary(op, operand, _) => {
                self.gen_expr(operand);
                
                if self.is_arm64() {
                    match op {
                        UnOp::Neg => self.emit_asm("    neg x0, x0"),
                        UnOp::Not => {
                            self.emit_asm("    cmp x0, #0");
                            self.emit_asm("    cset x0, eq");
                        }
                        UnOp::BitNot => self.emit_asm("    mvn x0, x0"),
                        _ => {}
                    }
                } else {
                    match op {
                        UnOp::Neg => self.emit_asm("    neg rax"),
                        UnOp::Not => {
                            self.emit_asm("    test rax, rax");
                            self.emit_asm("    sete al");
                            self.emit_asm("    movzx rax, al");
                        }
                        UnOp::BitNot => self.emit_asm("    not rax"),
                        _ => {}
                    }
                }
            }
            
            Expr::Call(callee, args, _) => {
                // OPTIMIZATION: For 1-2 args, evaluate directly to target registers
                if self.is_arm64() && args.len() <= 2 {
                    // Direct register allocation for simple cases
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            // Save previous in x19
                            self.emit_asm("    mov x19, x0");
                        }
                        self.gen_expr(arg);
                        if i == 0 && args.len() > 1 {
                            // First arg: save to x19, will move to x0 later
                            self.emit_asm("    mov x20, x0");
                        }
                    }
                    if args.len() == 2 {
                        // x0 has arg2 (right = x1), x20 has arg1 (left = x0)
                        self.emit_asm("    mov x1, x0");
                        self.emit_asm("    mov x0, x20");
                    }
                    // For 1 arg, x0 already has the value!
                } else if self.is_arm64() {
                    // Multi-arg case: use temporary registers
                    for (i, arg) in args.iter().enumerate() {
                        self.gen_expr(arg);
                        let reg = format!("x{}", 19 + i.min(1));
                        self.emit_asm(&format!("    mov {}, x0", reg));
                    }
                    for i in 0..args.len().min(8) {
                        let src = format!("x{}", 19 + i.min(1));
                        let dst = format!("x{}", i);
                        self.emit_asm(&format!("    mov {}, {}", dst, src));
                    }
                } else {
                    // x86_64: push args to stack, pop to registers
                    for arg in args.iter() {
                        self.gen_expr(arg);
                        self.emit_asm("    push rax");
                    }
                    let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                    for i in (0..args.len().min(6)).rev() {
                        self.emit_asm(&format!("    pop {}", regs[i]));
                    }
                }
                
                // Call function
                if let Expr::Ident(name, _) = callee.as_ref() {
                    if name.starts_with("__builtin_") {
                        self.gen_builtin_call(name, args.len());
                    } else {
                        if self.is_arm64() {
                            self.emit_asm(&format!("    bl _{}", name));
                        } else {
                            self.emit_asm(&format!("    call {}", name));
                        }
                    }
                }
            }
            
            _ => {
                // Default: return 0
                if self.is_arm64() {
                    self.emit_asm("    mov x0, #0");
                } else {
                    self.emit_asm("    xor rax, rax");
                }
            }
        }
    }
    
    fn gen_builtin_call(&mut self, name: &str, _argc: usize) {
        match name {
            "__builtin_print" => {
                // Inline print: push char to stack, write(stdout=1, buf=sp, count=1)
                if self.is_arm64() {
                    self.emit_asm("    sub sp, sp, #16");
                    self.emit_asm("    strb w0, [sp]");
                    self.emit_asm("    mov x2, #1");     // count
                    self.emit_asm("    mov x1, sp");     // buf
                    self.emit_asm("    mov x0, #1");     // stdout = 1
                    self.emit_asm("    mov x16, #4");    // SYS_write
                    self.emit_asm("    svc #0x80");
                    self.emit_asm("    add sp, sp, #16");
                } else {
                    self.emit_asm("    push rax");
                    self.emit_asm("    mov rdx, 1");     // count
                    self.emit_asm("    mov rsi, rsp");   // buf
                    self.emit_asm("    mov rdi, 1");     // stdout
                    self.emit_asm("    mov rax, 1");     // SYS_write
                    self.emit_asm("    syscall");
                    self.emit_asm("    pop rax");
                }
            }
            "__builtin_malloc" => {
                // Inline mmap-based allocation with minimum page size (branchless)
                // Always allocate at least 4096 bytes and round up to page boundary
                if self.is_arm64() {
                    // Add 4095 to ensure at least 4096 for any input including 0
                    // Then round to page: ((x0 + 4095) | 4095) - 4095 + 4096
                    // Simpler: just add 4096 then round down
                    self.emit_asm("    add x0, x0, #4096");   // x0 = x0 + 4096 (minimum 4096)
                    self.emit_asm("    and x0, x0, #-4096");  // round down to page
                    self.emit_asm("    mov x1, x0");          // len = aligned size
                    self.emit_asm("    mov x0, #0");          // addr = NULL
                    self.emit_asm("    mov x2, #3");          // prot = PROT_READ | PROT_WRITE
                    self.emit_asm("    mov x3, #0x1002");     // flags = MAP_PRIVATE | MAP_ANONYMOUS
                    self.emit_asm("    mov x4, #-1");         // fd = -1 (anonymous)
                    self.emit_asm("    mov x5, #0");          // offset = 0
                    self.emit_asm("    mov x16, #197");       // SYS_mmap (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    // x86-64: similar branchless version
                    self.emit_asm("    add rdi, 4096");       // rdi = rdi + 4096
                    self.emit_asm("    and rdi, -4096");      // round down
                    self.emit_asm("    mov rsi, rdi");        // len = aligned size
                    self.emit_asm("    xor rdi, rdi");        // addr = NULL
                    self.emit_asm("    mov rdx, 3");          // prot = PROT_READ | PROT_WRITE
                    self.emit_asm("    mov r10, 0x22");       // flags = MAP_PRIVATE | MAP_ANONYMOUS
                    self.emit_asm("    mov r8, -1");          // fd = -1
                    self.emit_asm("    mov r9, 0");           // offset = 0
                    self.emit_asm("    mov rax, 9");          // SYS_mmap
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_free" => {
                // No-op for bump allocator - could add munmap later
                if self.is_arm64() {
                    self.emit_asm("    mov x0, #0");
                } else {
                    self.emit_asm("    xor rax, rax");
                }
            }
            "__builtin_open" => {
                // Inline open syscall: open(path=x0, flags=x1, mode=x2)
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #5");  // SYS_open
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 2");  // SYS_open
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_close" => {
                // Inline close syscall: close(fd=x0)
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #6");  // SYS_close
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 3");  // SYS_close
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_read" => {
                // Inline read syscall: read(fd=x0, buf=x1, count=x2)
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #3");  // SYS_read
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 0");  // SYS_read
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_write" => {
                // Inline write syscall: write(fd=x0, buf=x1, count=x2)
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #4");  // SYS_write
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 1");  // SYS_write
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_seek" => {
                // Inline lseek syscall: lseek(fd=x0, offset=x1, whence=x2)
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #199");  // SYS_lseek
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 8");  // SYS_lseek
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_load8" => {
                // Inline load byte: x0 = *(uint8_t*)x0
                if self.is_arm64() {
                    self.emit_asm("    ldrb w0, [x0]");
                } else {
                    self.emit_asm("    movzx rax, byte ptr [rax]");
                }
            }
            "__builtin_load16" => {
                // Inline load 16-bit: x0 = *(uint16_t*)x0
                if self.is_arm64() {
                    self.emit_asm("    ldrh w0, [x0]");
                } else {
                    self.emit_asm("    movzx rax, word ptr [rax]");
                }
            }
            "__builtin_load32" => {
                // Inline load 32-bit: x0 = *(uint32_t*)x0
                if self.is_arm64() {
                    self.emit_asm("    ldr w0, [x0]");
                } else {
                    self.emit_asm("    mov eax, dword ptr [rax]");
                }
            }
            "__builtin_load64" => {
                // Inline load 64-bit: x0 = *(uint64_t*)x0
                if self.is_arm64() {
                    self.emit_asm("    ldr x0, [x0]");
                } else {
                    self.emit_asm("    mov rax, [rax]");
                }
            }
            "__builtin_store8" => {
                // Inline store byte: *(uint8_t*)x0 = x1
                if self.is_arm64() {
                    self.emit_asm("    strb w1, [x0]");
                } else {
                    self.emit_asm("    mov byte ptr [rdi], sil");
                }
            }
            "__builtin_store16" => {
                // Inline store 16-bit: *(uint16_t*)x0 = x1
                if self.is_arm64() {
                    self.emit_asm("    strh w1, [x0]");
                } else {
                    self.emit_asm("    mov word ptr [rdi], si");
                }
            }
            "__builtin_store32" => {
                // Inline store 32-bit: *(uint32_t*)x0 = x1
                if self.is_arm64() {
                    self.emit_asm("    str w1, [x0]");
                } else {
                    self.emit_asm("    mov dword ptr [rdi], esi");
                }
            }
            "__builtin_store64" => {
                // Inline store 64-bit: *(uint64_t*)x0 = x1
                if self.is_arm64() {
                    self.emit_asm("    str x1, [x0]");
                } else {
                    self.emit_asm("    mov [rdi], rsi");
                }
            }
            "__builtin_exit" => {
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #1");
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rdi, rax");
                    self.emit_asm("    mov rax, 60");
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_argc" => {
                // Return 0 for now - proper implementation needs startup code
                if self.is_arm64() {
                    self.emit_asm("    mov x0, #0");
                } else {
                    self.emit_asm("    xor rax, rax");
                }
            }
            "__builtin_argv" => {
                // Return 0 for now - proper implementation needs startup code
                if self.is_arm64() {
                    self.emit_asm("    mov x0, #0");
                } else {
                    self.emit_asm("    xor rax, rax");
                }
            }
            "__builtin_chmod" => {
                // Inline chmod syscall: chmod(path=x0, mode=x1)
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #15");  // SYS_chmod (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 90");  // SYS_chmod (Linux)
                    self.emit_asm("    syscall");
                }
            }
            // ============== SOCKET SYSCALLS FOR NETWORKING ==============
            "__builtin_socket" => {
                // socket(domain=x0, type=x1, protocol=x2) -> fd
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #97");  // SYS_socket (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 41");   // SYS_socket (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_connect" => {
                // connect(sockfd=x0, addr=x1, addrlen=x2) -> result
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #98");  // SYS_connect (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 42");   // SYS_connect (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_bind" => {
                // bind(sockfd=x0, addr=x1, addrlen=x2) -> result
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #104"); // SYS_bind (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 49");   // SYS_bind (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_listen" => {
                // listen(sockfd=x0, backlog=x1) -> result
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #106"); // SYS_listen (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 50");   // SYS_listen (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_accept" => {
                // accept(sockfd=x0, addr=x1, addrlen=x2) -> new_fd
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #30");  // SYS_accept (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 43");   // SYS_accept (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_sendto" => {
                // sendto(sockfd=x0, buf=x1, len=x2, flags=x3, dest=x4, destlen=x5) -> sent
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #133"); // SYS_sendto (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 44");   // SYS_sendto (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_recvfrom" => {
                // recvfrom(sockfd=x0, buf=x1, len=x2, flags=x3, src=x4, srclen=x5) -> received
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #29");  // SYS_recvfrom (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 45");   // SYS_recvfrom (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_setsockopt" => {
                // setsockopt(sockfd=x0, level=x1, optname=x2, optval=x3, optlen=x4) -> result
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #105"); // SYS_setsockopt (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 54");   // SYS_setsockopt (Linux)
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_shutdown" => {
                // shutdown(sockfd=x0, how=x1) -> result
                if self.is_arm64() {
                    self.emit_asm("    mov x16, #134"); // SYS_shutdown (macOS)
                    self.emit_asm("    svc #0x80");
                } else {
                    self.emit_asm("    mov rax, 48");   // SYS_shutdown (Linux)
                    self.emit_asm("    syscall");
                }
            }
            _ => {
                // Unknown builtin - emit as external call
                if self.is_arm64() {
                    self.emit_asm(&format!("    bl _{}", name));
                } else {
                    self.emit_asm(&format!("    call {}", name));
                }
            }
        }
    }
    
    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(init) = init {
                    self.gen_expr(init);
                }
                // OPTIMIZATION: Try to allocate a register for this variable
                if self.is_arm64() {
                    if let Some(reg) = self.alloc_register(name) {
                        // Store in register instead of memory - much faster!
                        self.emit_asm(&format!("    mov {}, x0", reg));
                        return; // Don't also store to memory
                    }
                }
                // Fallback to memory
                let off = self.alloc_local(name);
                if self.is_arm64() {
                    arm64::emit_store_local(&mut self.asm, "x0", off);
                } else {
                    x86_64::emit_store_local(&mut self.asm, "rax", off);
                }
            }
            
            Stmt::Assign(target, value, _) => {
                self.gen_expr(value);
                if let Expr::Ident(name, _) = target {
                    // OPTIMIZATION: Check if variable is in a register
                    if let Some(reg) = self.get_register(name) {
                        if self.is_arm64() {
                            self.emit_asm(&format!("    mov {}, x0", reg.clone()));
                            return;
                        }
                    }
                    // Fallback to memory
                    if let Some(off) = self.get_local(name) {
                        if self.is_arm64() {
                            arm64::emit_store_local(&mut self.asm, "x0", off);
                        } else {
                            x86_64::emit_store_local(&mut self.asm, "rax", off);
                        }
                    }
                }
            }
            
            Stmt::Return(val, _) => {
                if let Some(v) = val {
                    self.gen_expr(v);
                }
                if self.is_arm64() {
                    arm64::emit_epilogue(&mut self.asm, self.stack_offset);
                } else {
                    x86_64::emit_epilogue(&mut self.asm, self.stack_offset);
                }
            }
            
            Stmt::If(cond, then_block, else_block, _) => {
                let else_label = self.new_label();
                let end_label = self.new_label();
                
                // OPTIMIZATION: Detect if(n%2==0) pattern and use tbnz
                let mut used_tbnz = false;
                if self.is_arm64() {
                    if let Expr::Binary(BinOp::Eq, left, right, _) = cond {
                        if let (Expr::Binary(BinOp::Mod, inner, two, _), Expr::Int(0, _)) = (left.as_ref(), right.as_ref()) {
                            if let Expr::Int(2, _) = two.as_ref() {
                                // Pattern: (x % 2) == 0  --> tbnz x, #0, else (if bit 0 is set, jump to else)
                                self.gen_expr(inner);
                                self.emit_asm(&format!("    tbnz x0, #0, {}", else_label));
                                used_tbnz = true;
                            }
                        }
                    }
                }
                
                if !used_tbnz {
                    self.gen_expr(cond);
                    
                    if self.is_arm64() {
                        self.emit_asm(&format!("    cbz x0, {}", else_label));
                    } else {
                        self.emit_asm("    test rax, rax");
                        self.emit_asm(&format!("    jz {}", else_label));
                    }
                }
                
                for s in &then_block.stmts {
                    self.gen_stmt(s);
                }
                
                if self.is_arm64() {
                    self.emit_asm(&format!("    b {}", end_label));
                } else {
                    self.emit_asm(&format!("    jmp {}", end_label));
                }
                
                self.emit_asm(&format!("{}:", else_label));
                
                if let Some(eb) = else_block {
                    for s in &eb.stmts {
                        self.gen_stmt(s);
                    }
                }
                
                self.emit_asm(&format!("{}:", end_label));
            }
            
            Stmt::While(cond, body, _) => {
                let start_label = self.new_label();
                let end_label = self.new_label();
                
                // Push end label for break statements
                self.loop_end_labels.push(end_label.clone());
                
                self.emit_asm(&format!("{}:", start_label));
                self.gen_expr(cond);
                
                if self.is_arm64() {
                    self.emit_asm(&format!("    cbz x0, {}", end_label));
                } else {
                    self.emit_asm("    test rax, rax");
                    self.emit_asm(&format!("    jz {}", end_label));
                }
                
                for s in &body.stmts {
                    self.gen_stmt(s);
                }
                
                if self.is_arm64() {
                    self.emit_asm(&format!("    b {}", start_label));
                } else {
                    self.emit_asm(&format!("    jmp {}", start_label));
                }
                
                self.emit_asm(&format!("{}:", end_label));
                
                // Pop end label
                self.loop_end_labels.pop();
            }
            
            Stmt::Expr(expr, _) => {
                self.gen_expr(expr);
            }
            
            Stmt::Block(block, _) => {
                for s in &block.stmts {
                    self.gen_stmt(s);
                }
            }
            
            Stmt::Break(_) => {
                // Jump to innermost loop's end label
                if let Some(label) = self.loop_end_labels.last() {
                    let label = label.clone();
                    if self.is_arm64() {
                        self.emit_asm(&format!("    b {}", label));
                    } else {
                        self.emit_asm(&format!("    jmp {}", label));
                    }
                }
            }
            
            Stmt::Continue(_) => {
                // For continue, we'd need a start label stack too
                // For now, treat as no-op (less common)
            }
            
            _ => {}
        }
    }
    
    fn gen_func(&mut self, decl: &Decl) {
        if let Decl::Func { name, params, body, .. } = decl {
            self.locals.clear();
            self.stack_offset = 0;
            self.current_func = name.clone();
            
            // Function label
            if self.is_arm64() {
                self.emit_asm(&format!(".global _{}", name));
                self.emit_asm(".align 4");
                self.emit_asm(&format!("_{}:", name));
                arm64::emit_prologue(&mut self.asm);
                
                // OPTIMIZATION: Keep first param in x20 register 
                // This avoids memory loads in hot loops/recursion
                if !params.is_empty() {
                    self.emit_asm("    mov x20, x0");  // Save first param in callee-saved register
                }
            } else {
                self.emit_asm(&format!(".global {}", name));
                self.emit_asm(&format!("{}:", name));
                x86_64::emit_prologue(&mut self.asm);
            }
            
            // Store parameters (for access by name)
            let param_regs_arm = ["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"];
            let param_regs_x86 = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
            
            for (i, param) in params.iter().enumerate() {
                let off = self.alloc_local(&param.name);
                if self.is_arm64() && i < param_regs_arm.len() {
                    arm64::emit_store_local(&mut self.asm, param_regs_arm[i], off);
                } else if !self.is_arm64() && i < param_regs_x86.len() {
                    x86_64::emit_store_local(&mut self.asm, param_regs_x86[i], off);
                }
            }
            
            // Generate body
            for stmt in &body.stmts {
                self.gen_stmt(stmt);
            }
            
            // Epilogue (return value should already be in x0/rax from expression)
            if self.is_arm64() {
                arm64::emit_epilogue(&mut self.asm, self.stack_offset);
            } else {
                x86_64::emit_epilogue(&mut self.asm, self.stack_offset);
            }
        }
    }
    
    pub fn generate(&mut self, module: &TypedModule) -> Result<Vec<u8>> {
        // Header
        if self.is_arm64() {
            self.emit_asm(".section __TEXT,__text,regular,pure_instructions");
        } else {
            self.emit_asm(".text");
        }
        
        // First pass: collect all constants
        for typed_decl in &module.decls {
            if let Decl::Const { name, value, .. } = &typed_decl.decl {
                // Evaluate constant expression (only supports literal integers for now)
                if let Expr::Int(v, _) = value {
                    self.constants.insert(name.clone(), *v);
                }
            }
        }
        
        // Generate main first
        for typed_decl in &module.decls {
            if let Decl::Func { name, .. } = &typed_decl.decl {
                if name == "main" {
                    self.gen_func(&typed_decl.decl);
                    break;
                }
            }
        }
        
        // Generate other functions
        for typed_decl in &module.decls {
            if let Decl::Func { name, .. } = &typed_decl.decl {
                if name != "main" {
                    self.gen_func(&typed_decl.decl);
                }
            }
        }
        
        // Convert assembly to bytes (placeholder - actual assembly would need assembler)
        Ok(self.asm.as_bytes().to_vec())
    }
    
    pub fn get_asm(&self) -> &str {
        &self.asm
    }
}

pub fn generate(module: &TypedModule, target: &str, opt_level: u8) -> Result<Vec<u8>> {
    let mut codegen = CodeGen::new(target, opt_level);
    codegen.generate(module)
}

pub fn disassemble(code: &[u8], _target: &str) -> String {
    // Return the assembly string
    String::from_utf8_lossy(code).to_string()
}
