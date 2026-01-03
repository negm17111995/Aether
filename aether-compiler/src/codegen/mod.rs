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
    /// Local variable offsets
    locals: HashMap<String, i32>,
    /// Current stack offset
    stack_offset: i32,
    /// Optimization level
    opt_level: u8,
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
            stack_offset: 0,
            opt_level,
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
    
    fn is_arm64(&self) -> bool {
        self.target.contains("aarch64") || self.target.contains("arm64")
    }
    
    fn is_x86_64(&self) -> bool {
        self.target.contains("x86_64") || self.target.contains("x86-64")
    }
    
    /// Generate code for expression, result in r0/rax
    fn gen_expr(&mut self, expr: &Expr) {
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
                if let Some(off) = self.get_local(name) {
                    if self.is_arm64() {
                        arm64::emit_load_local(&mut self.asm, "x0", off);
                    } else {
                        x86_64::emit_load_local(&mut self.asm, "rax", off);
                    }
                } else {
                    // Assume it's a function reference or global
                    if self.is_arm64() {
                        arm64::emit_mov_imm(&mut self.asm, "x0", 0);
                    } else {
                        x86_64::emit_mov_imm(&mut self.asm, "rax", 0);
                    }
                }
            }
            
            Expr::Binary(op, left, right, _) => {
                // Evaluate left
                self.gen_expr(left);
                
                // Save left on stack
                if self.is_arm64() {
                    self.emit_asm("    str x0, [sp, #-16]!");
                } else {
                    self.emit_asm("    push rax");
                }
                
                // Evaluate right
                self.gen_expr(right);
                
                // Pop left to x1/rcx
                if self.is_arm64() {
                    self.emit_asm("    mov x1, x0");
                    self.emit_asm("    ldr x0, [sp], #16");
                    
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
                // Evaluate args
                for (i, arg) in args.iter().enumerate() {
                    self.gen_expr(arg);
                    
                    if self.is_arm64() {
                        // Save to callee-saved register or stack
                        let reg = format!("x{}", 19 + i);
                        self.emit_asm(&format!("    mov {}, x0", reg));
                    } else {
                        self.emit_asm("    push rax");
                    }
                }
                
                // Move args to proper registers
                if self.is_arm64() {
                    for i in 0..args.len().min(8) {
                        let src = format!("x{}", 19 + i);
                        let dst = format!("x{}", i);
                        self.emit_asm(&format!("    mov {}, {}", dst, src));
                    }
                } else {
                    // Pop args to registers (reverse order)
                    let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                    for i in (0..args.len().min(6)).rev() {
                        self.emit_asm(&format!("    pop {}", regs[i]));
                    }
                }
                
                // Call function
                if let Expr::Ident(name, _) = callee.as_ref() {
                    if name.starts_with("__builtin_") {
                        // Built-in function call via syscall
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
                if self.is_arm64() {
                    // write(1, buf, 1)
                    self.emit_asm("    str x0, [sp, #-16]!");
                    self.emit_asm("    mov x2, #1");
                    self.emit_asm("    mov x1, sp");
                    self.emit_asm("    mov x0, #1");
                    self.emit_asm("    mov x16, #4");
                    self.emit_asm("    svc #0x80");
                    self.emit_asm("    ldr x0, [sp], #16");
                } else {
                    self.emit_asm("    mov rdi, 1");
                    self.emit_asm("    lea rsi, [rsp]");
                    self.emit_asm("    mov rdx, 1");
                    self.emit_asm("    mov rax, 1");
                    self.emit_asm("    syscall");
                }
            }
            "__builtin_malloc" => {
                if self.is_arm64() {
                    // Simple bump allocator simulation
                    self.emit_asm("    // malloc");
                } else {
                    self.emit_asm("    // malloc stub");
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
            _ => {}
        }
    }
    
    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(init) = init {
                    self.gen_expr(init);
                }
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
                
                self.gen_expr(cond);
                
                if self.is_arm64() {
                    self.emit_asm(&format!("    cbz x0, {}", else_label));
                } else {
                    self.emit_asm("    test rax, rax");
                    self.emit_asm(&format!("    jz {}", else_label));
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
            }
            
            Stmt::Expr(expr, _) => {
                self.gen_expr(expr);
            }
            
            Stmt::Block(block, _) => {
                for s in &block.stmts {
                    self.gen_stmt(s);
                }
            }
            
            _ => {}
        }
    }
    
    fn gen_func(&mut self, decl: &Decl) {
        if let Decl::Func { name, params, body, .. } = decl {
            self.locals.clear();
            self.stack_offset = 0;
            
            // Function label
            if self.is_arm64() {
                self.emit_asm(&format!(".global _{}", name));
                self.emit_asm(".align 4");
                self.emit_asm(&format!("_{}:", name));
                arm64::emit_prologue(&mut self.asm);
            } else {
                self.emit_asm(&format!(".global {}", name));
                self.emit_asm(&format!("{}:", name));
                x86_64::emit_prologue(&mut self.asm);
            }
            
            // Store parameters
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
            
            // Default return
            if self.is_arm64() {
                self.emit_asm("    mov x0, #0");
                arm64::emit_epilogue(&mut self.asm, self.stack_offset);
            } else {
                self.emit_asm("    xor rax, rax");
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
