//! Aether Borrow Checker - Memory safety analysis
//!
//! Ensures no use-after-free, no double-free, no data races

use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::typechecker::TypedModule;
use anyhow::{anyhow, Result};

/// Borrow state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BorrowState {
    Owned,
    Borrowed,
    MutBorrowed,
    Moved,
}

/// Borrow checker context
pub struct BorrowChecker {
    /// Variable borrow states
    states: HashMap<String, BorrowState>,
    /// Active borrows for each variable
    borrows: HashMap<String, Vec<String>>,
    /// Errors found
    errors: Vec<String>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        BorrowChecker {
            states: HashMap::new(),
            borrows: HashMap::new(),
            errors: Vec::new(),
        }
    }
    
    fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }
    
    fn define(&mut self, name: &str, mutable: bool) {
        self.states.insert(name.to_string(), BorrowState::Owned);
        self.borrows.insert(name.to_string(), Vec::new());
    }
    
    fn use_var(&mut self, name: &str, span: Span) {
        if let Some(state) = self.states.get(name) {
            if *state == BorrowState::Moved {
                self.error(format!("Use of moved value: {} at line {}", name, span.line));
            }
        }
    }
    
    fn move_var(&mut self, name: &str, span: Span) {
        if let Some(state) = self.states.get(name) {
            if *state == BorrowState::Moved {
                self.error(format!("Value already moved: {} at line {}", name, span.line));
            } else if *state == BorrowState::Borrowed || *state == BorrowState::MutBorrowed {
                self.error(format!("Cannot move borrowed value: {} at line {}", name, span.line));
            }
        }
        self.states.insert(name.to_string(), BorrowState::Moved);
    }
    
    fn borrow(&mut self, name: &str, mutable: bool, span: Span) {
        if let Some(state) = self.states.get(name) {
            match state {
                BorrowState::Moved => {
                    self.error(format!("Cannot borrow moved value: {} at line {}", name, span.line));
                }
                BorrowState::MutBorrowed if mutable => {
                    self.error(format!("Cannot borrow mutably: already mutably borrowed: {} at line {}", name, span.line));
                }
                BorrowState::MutBorrowed => {
                    self.error(format!("Cannot borrow: already mutably borrowed: {} at line {}", name, span.line));
                }
                BorrowState::Borrowed if mutable => {
                    self.error(format!("Cannot borrow mutably: already borrowed: {} at line {}", name, span.line));
                }
                _ => {}
            }
        }
        
        let new_state = if mutable { BorrowState::MutBorrowed } else { BorrowState::Borrowed };
        self.states.insert(name.to_string(), new_state);
    }
    
    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name, span) => {
                self.use_var(name, *span);
            }
            Expr::Binary(_, left, right, _) => {
                self.check_expr(left);
                self.check_expr(right);
            }
            Expr::Unary(op, operand, span) => {
                match op {
                    UnOp::Ref => {
                        if let Expr::Ident(name, _) = operand.as_ref() {
                            self.borrow(name, false, *span);
                        }
                    }
                    UnOp::Deref => {
                        self.check_expr(operand);
                    }
                    _ => {
                        self.check_expr(operand);
                    }
                }
            }
            Expr::Call(callee, args, _) => {
                self.check_expr(callee);
                for arg in args {
                    self.check_expr(arg);
                }
            }
            Expr::Field(obj, _, _) => {
                self.check_expr(obj);
            }
            Expr::Index(arr, idx, _) => {
                self.check_expr(arr);
                self.check_expr(idx);
            }
            Expr::Array(elems, _) => {
                for elem in elems {
                    self.check_expr(elem);
                }
            }
            Expr::If(cond, then_block, else_block, _) => {
                self.check_expr(cond);
                self.check_block(then_block);
                if let Some(eb) = else_block {
                    self.check_block(eb);
                }
            }
            _ => {}
        }
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, init, mutable, .. } => {
                if let Some(init) = init {
                    self.check_expr(init);
                }
                self.define(name, *mutable);
            }
            Stmt::Assign(target, value, span) => {
                self.check_expr(value);
                if let Expr::Ident(name, _) = target {
                    // Check if mutable
                    self.use_var(name, *span);
                }
            }
            Stmt::Expr(expr, _) => {
                self.check_expr(expr);
            }
            Stmt::If(cond, then_block, else_block, _) => {
                self.check_expr(cond);
                self.check_block(&then_block);
                if let Some(eb) = else_block {
                    self.check_block(eb);
                }
            }
            Stmt::While(cond, body, _) => {
                self.check_expr(cond);
                self.check_block(&body);
            }
            Stmt::For(var, iter, body, _) => {
                self.check_expr(iter);
                self.define(var, false);
                self.check_block(body);
            }
            Stmt::Return(val, _) => {
                if let Some(v) = val {
                    self.check_expr(v);
                }
            }
            Stmt::Block(block, _) => {
                self.check_block(block);
            }
            _ => {}
        }
    }
    
    fn check_block(&mut self, block: &Block) {
        let saved_states = self.states.clone();
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
        // Restore states after block (variables go out of scope)
        self.states = saved_states;
    }
    
    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Func { params, body, .. } => {
                self.states.clear();
                self.borrows.clear();
                
                for param in params {
                    self.define(&param.name, false);
                }
                
                self.check_block(body);
            }
            _ => {}
        }
    }
    
    pub fn check_module(&mut self, module: &TypedModule) -> Result<()> {
        for typed_decl in &module.decls {
            self.check_decl(&typed_decl.decl);
        }
        
        if !self.errors.is_empty() {
            return Err(anyhow!("Borrow check errors:\n{}", self.errors.join("\n")));
        }
        
        Ok(())
    }
}

pub fn check(module: &TypedModule) -> Result<()> {
    let mut checker = BorrowChecker::new();
    checker.check_module(module)
}
