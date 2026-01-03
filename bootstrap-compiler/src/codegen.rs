// AETHER CODE GENERATOR - Generates ARM64 machine code

use crate::parser::{Module, Decl, Func, Stmt, Expr, BinOp, UnaryOp};
use std::collections::HashMap;

pub struct CodeGen {
    code: Vec<u8>,
    strings: Vec<(usize, String)>,  // (offset, string)
    functions: HashMap<String, usize>,  // name -> offset
    locals: HashMap<String, i32>,  // name -> stack offset
    stack_size: i32,
    label_counter: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            code: Vec::new(),
            strings: Vec::new(),
            functions: HashMap::new(),
            locals: HashMap::new(),
            stack_size: 0,
            label_counter: 0,
        }
    }
    
    fn emit(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }
    
    fn emit_u32(&mut self, val: u32) {
        self.code.extend_from_slice(&val.to_le_bytes());
    }
    
    fn current_offset(&self) -> usize {
        self.code.len()
    }
    
    // ARM64 instruction encoders
    fn arm_mov_imm(&mut self, rd: u8, imm: u16) {
        // MOVZ Xd, #imm
        let inst = 0xD2800000 | ((imm as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_mov_imm64(&mut self, rd: u8, imm: i64) {
        // Load 64-bit immediate using MOVZ + MOVK sequence
        let uimm = imm as u64;
        
        // MOVZ Xd, #imm[15:0]
        self.arm_mov_imm(rd, (uimm & 0xFFFF) as u16);
        
        if uimm > 0xFFFF {
            // MOVK Xd, #imm[31:16], LSL #16
            let inst = 0xF2A00000 | (((uimm >> 16) & 0xFFFF) as u32) << 5 | (rd as u32);
            self.emit_u32(inst);
        }
        if uimm > 0xFFFFFFFF {
            // MOVK Xd, #imm[47:32], LSL #32
            let inst = 0xF2C00000 | (((uimm >> 32) & 0xFFFF) as u32) << 5 | (rd as u32);
            self.emit_u32(inst);
        }
        if uimm > 0xFFFFFFFFFFFF {
            // MOVK Xd, #imm[63:48], LSL #48
            let inst = 0xF2E00000 | (((uimm >> 48) & 0xFFFF) as u32) << 5 | (rd as u32);
            self.emit_u32(inst);
        }
    }
    
    fn arm_mov_reg(&mut self, rd: u8, rm: u8) {
        // Handle SP specially - use ADD Xd, SP, #0
        if rm == 31 {
            // ADD Xd, SP, #0
            let inst = 0x91000000 | (31 << 5) | (rd as u32);
            self.emit_u32(inst);
        } else {
            // MOV Xd, Xm (alias for ORR Xd, XZR, Xm)
            let inst = 0xAA0003E0 | ((rm as u32) << 16) | (rd as u32);
            self.emit_u32(inst);
        }
    }
    
    fn arm_add_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // ADD Xd, Xn, Xm
        let inst = 0x8B000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_sub_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // SUB Xd, Xn, Xm
        let inst = 0xCB000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_mul_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // MUL Xd, Xn, Xm (alias for MADD Xd, Xn, Xm, XZR)
        let inst = 0x9B007C00 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_sdiv_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // SDIV Xd, Xn, Xm
        let inst = 0x9AC00C00 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_and_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // AND Xd, Xn, Xm
        let inst = 0x8A000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_orr_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // ORR Xd, Xn, Xm
        let inst = 0xAA000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_eor_reg(&mut self, rd: u8, rn: u8, rm: u8) {
        // EOR Xd, Xn, Xm
        let inst = 0xCA000000 | ((rm as u32) << 16) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_neg_reg(&mut self, rd: u8, rm: u8) {
        // NEG Xd, Xm (alias for SUB Xd, XZR, Xm)
        let inst = 0xCB0003E0 | ((rm as u32) << 16) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_cmp_reg(&mut self, rn: u8, rm: u8) {
        // CMP Xn, Xm (alias for SUBS XZR, Xn, Xm)
        let inst = 0xEB00001F | ((rm as u32) << 16) | ((rn as u32) << 5);
        self.emit_u32(inst);
    }
    
    fn arm_cmp_imm(&mut self, rn: u8, imm: u16) {
        // CMP Xn, #imm
        let inst = 0xF100001F | ((imm as u32) << 10) | ((rn as u32) << 5);
        self.emit_u32(inst);
    }
    
    fn arm_cset(&mut self, rd: u8, cond: u8) {
        // CSET Xd, cond (alias for CSINC Xd, XZR, XZR, invert(cond))
        let inv_cond = cond ^ 1;
        let inst = 0x9A9F07E0 | ((inv_cond as u32) << 12) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_b(&mut self, offset: i32) {
        // B offset
        let imm26 = ((offset >> 2) as u32) & 0x03FFFFFF;
        let inst = 0x14000000 | imm26;
        self.emit_u32(inst);
    }
    
    fn arm_b_cond(&mut self, cond: u8, offset: i32) {
        // B.cond offset
        let imm19 = ((offset >> 2) as u32) & 0x7FFFF;
        let inst = 0x54000000 | (imm19 << 5) | (cond as u32);
        self.emit_u32(inst);
    }
    
    fn arm_bl(&mut self, offset: i32) {
        // BL offset
        let imm26 = ((offset >> 2) as u32) & 0x03FFFFFF;
        let inst = 0x94000000 | imm26;
        self.emit_u32(inst);
    }
    
    fn arm_ret(&mut self) {
        // RET (uses X30/LR)
        self.emit_u32(0xD65F03C0);
    }
    
    fn arm_stp(&mut self, rt1: u8, rt2: u8, rn: u8, imm: i16) {
        // STP Xt1, Xt2, [Xn, #imm]!
        let imm7 = ((imm >> 3) as u32) & 0x7F;
        let inst = 0xA9800000 | (imm7 << 15) | ((rt2 as u32) << 10) | ((rn as u32) << 5) | (rt1 as u32);
        self.emit_u32(inst);
    }
    
    fn arm_ldp(&mut self, rt1: u8, rt2: u8, rn: u8, imm: i16) {
        // LDP Xt1, Xt2, [Xn], #imm
        let imm7 = ((imm >> 3) as u32) & 0x7F;
        let inst = 0xA8C00000 | (imm7 << 15) | ((rt2 as u32) << 10) | ((rn as u32) << 5) | (rt1 as u32);
        self.emit_u32(inst);
    }
    
    fn arm_str(&mut self, rt: u8, rn: u8, offset: i16) {
        if offset < 0 {
            // STUR Xt, [Xn, #offset] for negative offsets
            let imm9 = (offset as i32 & 0x1FF) as u32;
            let inst = 0xF8000000 | (imm9 << 12) | ((rn as u32) << 5) | (rt as u32);
            self.emit_u32(inst);
        } else {
            // STR Xt, [Xn, #offset] for positive offsets
            let uoffset = ((offset as u32) >> 3) & 0xFFF;
            let inst = 0xF9000000 | (uoffset << 10) | ((rn as u32) << 5) | (rt as u32);
            self.emit_u32(inst);
        }
    }
    
    fn arm_ldr(&mut self, rt: u8, rn: u8, offset: i16) {
        if offset < 0 {
            // LDUR Xt, [Xn, #offset] for negative offsets
            let imm9 = (offset as i32 & 0x1FF) as u32;
            let inst = 0xF8400000 | (imm9 << 12) | ((rn as u32) << 5) | (rt as u32);
            self.emit_u32(inst);
        } else {
            // LDR Xt, [Xn, #offset] for positive offsets
            let uoffset = ((offset as u32) >> 3) & 0xFFF;
            let inst = 0xF9400000 | (uoffset << 10) | ((rn as u32) << 5) | (rt as u32);
            self.emit_u32(inst);
        }
    }
    
    fn arm_sub_imm(&mut self, rd: u8, rn: u8, imm: u16) {
        // SUB Xd, Xn, #imm
        let inst = 0xD1000000 | ((imm as u32) << 10) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_add_imm(&mut self, rd: u8, rn: u8, imm: u16) {
        // ADD Xd, Xn, #imm
        let inst = 0x91000000 | ((imm as u32) << 10) | ((rn as u32) << 5) | (rd as u32);
        self.emit_u32(inst);
    }
    
    fn arm_svc(&mut self, imm: u16) {
        // SVC #imm
        let inst = 0xD4000001 | ((imm as u32) << 5);
        self.emit_u32(inst);
    }
    
    // Code generation
    fn alloc_local(&mut self, name: &str) -> i32 {
        self.stack_size += 8;
        let offset = self.stack_size;
        self.locals.insert(name.to_string(), offset);
        offset
    }
    
    fn get_local(&self, name: &str) -> Option<i32> {
        self.locals.get(name).copied()
    }
    
    fn emit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLit(n) => {
                self.arm_mov_imm64(0, *n);
            }
            Expr::StrLit(s) => {
                // Store string address (will be patched later)
                let offset = self.current_offset();
                self.strings.push((offset, s.clone()));
                self.arm_mov_imm64(0, 0); // Placeholder
            }
            Expr::Ident(name) => {
                if let Some(offset) = self.get_local(name) {
                    // Load from stack
                    self.arm_ldr(0, 29, -(offset as i16));
                } else {
                    // Could be a global or function - return 0 for now
                    self.arm_mov_imm(0, 0);
                }
            }
            Expr::Binary(left, op, right) => {
                // Evaluate left -> X0
                self.emit_expr(left);
                // Push X0
                self.arm_sub_imm(31, 31, 16);
                self.arm_str(0, 31, 0);
                // Evaluate right -> X0
                self.emit_expr(right);
                // Move right to X1
                self.arm_mov_reg(1, 0);
                // Pop left to X0
                self.arm_ldr(0, 31, 0);
                self.arm_add_imm(31, 31, 16);
                
                // Perform operation
                match op {
                    BinOp::Add => self.arm_add_reg(0, 0, 1),
                    BinOp::Sub => self.arm_sub_reg(0, 0, 1),
                    BinOp::Mul => self.arm_mul_reg(0, 0, 1),
                    BinOp::Div => self.arm_sdiv_reg(0, 0, 1),
                    BinOp::Mod => {
                        // X2 = X0 / X1
                        self.arm_sdiv_reg(2, 0, 1);
                        // X2 = X2 * X1
                        self.arm_mul_reg(2, 2, 1);
                        // X0 = X0 - X2
                        self.arm_sub_reg(0, 0, 2);
                    }
                    BinOp::Eq => { self.arm_cmp_reg(0, 1); self.arm_cset(0, 0); } // EQ
                    BinOp::Ne => { self.arm_cmp_reg(0, 1); self.arm_cset(0, 1); } // NE
                    BinOp::Lt => { self.arm_cmp_reg(0, 1); self.arm_cset(0, 11); } // LT
                    BinOp::Le => { self.arm_cmp_reg(0, 1); self.arm_cset(0, 13); } // LE
                    BinOp::Gt => { self.arm_cmp_reg(0, 1); self.arm_cset(0, 12); } // GT
                    BinOp::Ge => { self.arm_cmp_reg(0, 1); self.arm_cset(0, 10); } // GE
                    BinOp::And => { self.arm_cmp_imm(0, 0); self.arm_cset(0, 1); 
                                    self.arm_cmp_imm(1, 0); self.arm_cset(1, 1);
                                    self.arm_and_reg(0, 0, 1); }
                    BinOp::Or => { self.arm_orr_reg(0, 0, 1); self.arm_cmp_imm(0, 0); self.arm_cset(0, 1); }
                    BinOp::BitAnd => self.arm_and_reg(0, 0, 1),
                    BinOp::BitOr => self.arm_orr_reg(0, 0, 1),
                    BinOp::BitXor => self.arm_eor_reg(0, 0, 1),
                }
            }
            Expr::Unary(op, inner) => {
                self.emit_expr(inner);
                match op {
                    UnaryOp::Neg => self.arm_neg_reg(0, 0),
                    UnaryOp::Not => { self.arm_cmp_imm(0, 0); self.arm_cset(0, 0); }
                    UnaryOp::BitNot => { 
                        self.arm_mov_imm64(1, -1);
                        self.arm_eor_reg(0, 0, 1);
                    }
                }
            }
            Expr::Call(name, args) => {
                // Push arguments
                for (i, arg) in args.iter().enumerate().take(8) {
                    self.emit_expr(arg);
                    if i < 7 {
                        self.arm_mov_reg(i as u8, 0);
                    }
                }
                
                // Branch to function (placeholder - will need to be patched)
                // For now, just emit a NOP
                self.emit_u32(0xD503201F); // NOP
            }
            Expr::BuiltinCall(name, args) => {
                if name == "__builtin_print" && args.len() >= 1 {
                    // Write char to stdout
                    // X0 = char to print
                    self.emit_expr(&args[0]);
                    // Store char on stack
                    self.arm_sub_imm(31, 31, 16);
                    self.arm_str(0, 31, 0);
                    
                    // syscall: write(1, &char, 1)
                    self.arm_mov_imm(0, 1);      // fd = stdout
                    self.arm_mov_reg(1, 31);    // buf = sp
                    self.arm_mov_imm(2, 1);      // count = 1
                    self.arm_mov_imm(16, 4);     // write syscall number (macOS ARM64)
                    self.arm_svc(0x80);
                    
                    self.arm_add_imm(31, 31, 16);
                } else if name == "__builtin_exit" && args.len() >= 1 {
                    self.emit_expr(&args[0]);
                    self.arm_mov_imm(16, 1);     // exit syscall
                    self.arm_svc(0x80);
                } else if name == "__builtin_malloc" && args.len() >= 1 {
                    // Simple bump allocator using mmap
                    self.emit_expr(&args[0]);
                    self.arm_mov_reg(1, 0);      // size
                    self.arm_mov_imm(0, 0);      // addr = NULL
                    self.arm_mov_imm(2, 3);      // prot = PROT_READ | PROT_WRITE
                    self.arm_mov_imm(3, 0x1002); // flags = MAP_PRIVATE | MAP_ANON
                    self.arm_mov_imm64(4, -1);   // fd = -1
                    self.arm_mov_imm(5, 0);      // offset = 0
                    self.arm_mov_imm(16, 197);   // mmap syscall (macOS ARM64)
                    self.arm_svc(0x80);
                } else if name == "__builtin_store8" && args.len() >= 2 {
                    self.emit_expr(&args[1]); // value
                    self.arm_mov_reg(1, 0);
                    self.emit_expr(&args[0]); // addr
                    // STRB W1, [X0]
                    self.emit_u32(0x39000001);
                } else if name == "__builtin_load8" && args.len() >= 1 {
                    self.emit_expr(&args[0]); // addr
                    // LDRB W0, [X0]
                    self.emit_u32(0x39400000);
                } else if name == "__builtin_store64" && args.len() >= 2 {
                    self.emit_expr(&args[1]); // value
                    self.arm_mov_reg(1, 0);
                    self.emit_expr(&args[0]); // addr
                    // STR X1, [X0]
                    self.emit_u32(0xF9000001);
                } else if name == "__builtin_load64" && args.len() >= 1 {
                    self.emit_expr(&args[0]); // addr
                    // LDR X0, [X0]
                    self.emit_u32(0xF9400000);
                } else {
                    // Unknown builtin - return 0
                    self.arm_mov_imm(0, 0);
                }
            }
        }
    }
    
    fn emit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, init } => {
                let offset = self.alloc_local(name);
                if let Some(expr) = init {
                    self.emit_expr(expr);
                    self.arm_str(0, 29, -(offset as i16));
                }
            }
            Stmt::Assign { target, value } => {
                self.emit_expr(value);
                if let Some(offset) = self.get_local(target) {
                    self.arm_str(0, 29, -(offset as i16));
                }
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.emit_expr(e);
                }
                // Epilogue will be emitted by emit_func
            }
            Stmt::If { cond, then_block, else_block } => {
                self.emit_expr(cond);
                self.arm_cmp_imm(0, 0);
                
                let branch_pos = self.current_offset();
                self.arm_b_cond(0, 0); // B.EQ placeholder
                
                for s in then_block {
                    self.emit_stmt(s);
                }
                
                if let Some(else_stmts) = else_block {
                    let jump_pos = self.current_offset();
                    self.arm_b(0); // B placeholder
                    
                    // Patch else branch
                    let else_start = self.current_offset();
                    let offset = (else_start as i32 - branch_pos as i32) as i32;
                    let imm19 = ((offset >> 2) as u32) & 0x7FFFF;
                    let inst = 0x54000000 | (imm19 << 5) | 0; // B.EQ
                    self.code[branch_pos..branch_pos+4].copy_from_slice(&inst.to_le_bytes());
                    
                    for s in else_stmts {
                        self.emit_stmt(s);
                    }
                    
                    // Patch end jump
                    let end = self.current_offset();
                    let offset = (end as i32 - jump_pos as i32) as i32;
                    let imm26 = ((offset >> 2) as u32) & 0x03FFFFFF;
                    let inst = 0x14000000 | imm26;
                    self.code[jump_pos..jump_pos+4].copy_from_slice(&inst.to_le_bytes());
                } else {
                    // Patch branch
                    let end = self.current_offset();
                    let offset = (end as i32 - branch_pos as i32) as i32;
                    let imm19 = ((offset >> 2) as u32) & 0x7FFFF;
                    let inst = 0x54000000 | (imm19 << 5) | 0;
                    self.code[branch_pos..branch_pos+4].copy_from_slice(&inst.to_le_bytes());
                }
            }
            Stmt::While { cond, body } => {
                let loop_start = self.current_offset();
                
                self.emit_expr(cond);
                self.arm_cmp_imm(0, 0);
                
                let branch_pos = self.current_offset();
                self.arm_b_cond(0, 0); // B.EQ placeholder
                
                for s in body {
                    self.emit_stmt(s);
                }
                
                // Jump back
                let back_offset = (loop_start as i32 - self.current_offset() as i32) as i32;
                self.arm_b(back_offset);
                
                // Patch exit branch
                let end = self.current_offset();
                let offset = (end as i32 - branch_pos as i32) as i32;
                let imm19 = ((offset >> 2) as u32) & 0x7FFFF;
                let inst = 0x54000000 | (imm19 << 5) | 0;
                self.code[branch_pos..branch_pos+4].copy_from_slice(&inst.to_le_bytes());
            }
            Stmt::Expr(e) => {
                self.emit_expr(e);
            }
        }
    }
    
    fn emit_func(&mut self, func: &Func) {
        self.functions.insert(func.name.clone(), self.current_offset());
        self.locals.clear();
        self.stack_size = 0;
        
        // Prologue: STP X29, X30, [SP, #-16]!;  MOV X29, SP
        self.arm_stp(29, 30, 31, -16);
        self.arm_mov_reg(29, 31);
        
        // Allocate stack space (placeholder - will patch)
        let sub_sp_pos = self.current_offset();
        self.arm_sub_imm(31, 31, 0); // Placeholder
        
        // Store parameters
        for (i, param) in func.params.iter().enumerate().take(8) {
            let offset = self.alloc_local(&param.name);
            self.arm_str(i as u8, 29, -(offset as i16));
        }
        
        // Emit body
        for stmt in &func.body {
            self.emit_stmt(stmt);
        }
        
        // Default return 0
        self.arm_mov_imm(0, 0);
        
        // Epilogue
        let aligned_stack = (self.stack_size + 15) & !15;
        self.arm_add_imm(31, 31, aligned_stack as u16);
        self.arm_ldp(29, 30, 31, 16);
        self.arm_ret();
        
        // Patch stack allocation
        if aligned_stack > 0 {
            let inst = 0xD1000000 | ((aligned_stack as u32) << 10) | (31 << 5) | 31;
            self.code[sub_sp_pos..sub_sp_pos+4].copy_from_slice(&inst.to_le_bytes());
        }
    }
    
    pub fn generate(&mut self, module: &Module) -> Vec<u8> {
        // Find main function first
        let mut main_idx = None;
        for (i, decl) in module.decls.iter().enumerate() {
            if let Decl::Func(f) = decl {
                if f.name == "main" {
                    main_idx = Some(i);
                }
            }
        }
        
        // Emit all functions
        for decl in &module.decls {
            if let Decl::Func(f) = decl {
                self.emit_func(f);
            }
        }
        
        self.code.clone()
    }
}

pub fn generate(module: &Module) -> Vec<u8> {
    let mut codegen = CodeGen::new();
    codegen.generate(module)
}
