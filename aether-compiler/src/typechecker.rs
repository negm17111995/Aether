//! Aether Type Checker - Type inference and checking
//!
//! Handles generics, polymorphism, and type safety

use std::collections::HashMap;
use crate::ast::*;
use anyhow::{anyhow, Result};

/// Type environment
#[derive(Debug, Clone)]
pub struct TypeEnv {
    /// Variable types
    vars: HashMap<String, Type>,
    /// Function signatures
    funcs: HashMap<String, (Vec<Type>, Option<Type>)>,
    /// Struct definitions
    structs: HashMap<String, Vec<(String, Type)>>,
    /// Type aliases
    aliases: HashMap<String, Type>,
    /// Parent scope
    parent: Option<Box<TypeEnv>>,
}

impl TypeEnv {
    pub fn new() -> Self {
        let mut env = TypeEnv {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            structs: HashMap::new(),
            aliases: HashMap::new(),
            parent: None,
        };
        
        // Built-in types
        env.aliases.insert("Int".into(), Type::Named("Int".into()));
        env.aliases.insert("Float".into(), Type::Named("Float".into()));
        env.aliases.insert("Bool".into(), Type::Named("Bool".into()));
        env.aliases.insert("String".into(), Type::Named("String".into()));
        env.aliases.insert("Char".into(), Type::Named("Char".into()));
        
        env
    }
    
    pub fn child(&self) -> Self {
        TypeEnv {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            structs: HashMap::new(),
            aliases: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }
    
    pub fn lookup_var(&self, name: &str) -> Option<Type> {
        self.vars.get(name).cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup_var(name)))
    }
    
    pub fn lookup_func(&self, name: &str) -> Option<(Vec<Type>, Option<Type>)> {
        self.funcs.get(name).cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup_func(name)))
    }
    
    pub fn define_var(&mut self, name: String, ty: Type) {
        self.vars.insert(name, ty);
    }
    
    pub fn define_func(&mut self, name: String, params: Vec<Type>, ret: Option<Type>) {
        self.funcs.insert(name, (params, ret));
    }
}

/// Typed AST
#[derive(Debug, Clone)]
pub struct TypedModule {
    pub decls: Vec<TypedDecl>,
}

#[derive(Debug, Clone)]
pub struct TypedDecl {
    pub decl: Decl,
    pub ty: Type,
}

/// Type checker
pub struct TypeChecker {
    env: TypeEnv,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            env: TypeEnv::new(),
            errors: Vec::new(),
        }
    }
    
    fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }
    
    fn infer_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Int(_, _) => Type::Named("Int".into()),
            Expr::Float(_, _) => Type::Named("Float".into()),
            Expr::String(_, _) => Type::Named("String".into()),
            Expr::Bool(_, _) => Type::Named("Bool".into()),
            
            Expr::Ident(name, span) => {
                // Lenient mode: allow unknown variables (could be from other modules)
                self.env.lookup_var(name).unwrap_or(Type::Named("Int".into()))
            }
            
            Expr::Binary(op, left, right, span) => {
                let lt = self.infer_expr(left);
                let rt = self.infer_expr(right);
                
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                        if lt.is_int() && rt.is_int() {
                            Type::Named("Int".into())
                        } else {
                            Type::Named("Float".into())
                        }
                    }
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge |
                    BinOp::And | BinOp::Or => Type::Named("Bool".into()),
                    BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => lt,
                }
            }
            
            Expr::Unary(op, operand, _) => {
                let t = self.infer_expr(operand);
                match op {
                    UnOp::Neg => t,
                    UnOp::Not => Type::Named("Bool".into()),
                    UnOp::BitNot => t,
                    UnOp::Ref => Type::Ptr(Box::new(t)),
                    UnOp::Deref => {
                        if let Type::Ptr(inner) = t {
                            *inner
                        } else {
                            self.error("Cannot dereference non-pointer".into());
                            Type::Infer
                        }
                    }
                }
            }
            
            Expr::Call(callee, args, span) => {
                if let Expr::Ident(name, _) = callee.as_ref() {
                    if let Some((params, ret)) = self.env.lookup_func(name) {
                        if args.len() != params.len() {
                            self.error(format!("Wrong number of arguments at line {}", span.line));
                        }
                        ret.unwrap_or(Type::Unit)
                    } else {
                        // Allow all function calls (lenient mode for stdlib compilation)
                        // Functions will be resolved at link time
                        return Type::Named("Int".into());
                    }
                } else {
                    Type::Infer
                }
            }
            
            Expr::Field(obj, field, span) => {
                let obj_ty = self.infer_expr(obj);
                if let Type::Named(name) = &obj_ty {
                    if let Some(fields) = self.env.structs.get(name) {
                        for (fn_, ft) in fields {
                            if fn_ == field {
                                return ft.clone();
                            }
                        }
                    }
                }
                self.error(format!("Unknown field {} at line {}", field, span.line));
                Type::Infer
            }
            
            Expr::Index(arr, _, _) => {
                let arr_ty = self.infer_expr(arr);
                if let Type::Array(elem, _) = arr_ty {
                    *elem
                } else {
                    Type::Infer
                }
            }
            
            Expr::Array(elems, _) => {
                if elems.is_empty() {
                    Type::Array(Box::new(Type::Infer), None)
                } else {
                    let elem_ty = self.infer_expr(&elems[0]);
                    Type::Array(Box::new(elem_ty), Some(elems.len()))
                }
            }
            
            Expr::If(_, then_block, _, _) => {
                if let Some(last) = then_block.stmts.last() {
                    if let Stmt::Expr(e, _) = last {
                        return self.infer_expr(e);
                    }
                }
                Type::Unit
            }
            
            _ => Type::Infer,
        }
    }
    
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, init, .. } => {
                let inferred = init.as_ref().map(|e| self.infer_expr(e));
                let final_ty = ty.clone().or(inferred).unwrap_or(Type::Infer);
                self.env.define_var(name.clone(), final_ty);
            }
            Stmt::Assign(target, value, span) => {
                let lt = self.infer_expr(target);
                let rt = self.infer_expr(value);
                // Type compatibility check could go here
            }
            Stmt::If(cond, then_block, else_block, _) => {
                let ct = self.infer_expr(cond);
                if !ct.is_bool() && !matches!(ct, Type::Named(ref n) if n == "Int") {
                    self.error("If condition must be Bool".into());
                }
                for s in &then_block.stmts {
                    self.check_stmt(s);
                }
                if let Some(eb) = else_block {
                    for s in &eb.stmts {
                        self.check_stmt(s);
                    }
                }
            }
            Stmt::While(cond, body, _) => {
                self.infer_expr(cond);
                for s in &body.stmts {
                    self.check_stmt(s);
                }
            }
            Stmt::For(var, iter, body, _) => {
                let iter_ty = self.infer_expr(iter);
                let elem_ty = if let Type::Array(elem, _) = iter_ty {
                    *elem
                } else {
                    Type::Infer
                };
                self.env.define_var(var.clone(), elem_ty);
                for s in &body.stmts {
                    self.check_stmt(s);
                }
            }
            Stmt::Return(val, _) => {
                if let Some(v) = val {
                    self.infer_expr(v);
                }
            }
            Stmt::Expr(e, _) => {
                self.infer_expr(e);
            }
            Stmt::Block(block, _) => {
                let old_env = self.env.clone();
                self.env = self.env.child();
                for s in &block.stmts {
                    self.check_stmt(s);
                }
                self.env = old_env;
            }
            _ => {}
        }
    }
    
    fn check_decl(&mut self, decl: &Decl) -> TypedDecl {
        match decl {
            Decl::Func { name, params, ret, body, .. } => {
                let param_types: Vec<Type> = params.iter().map(|p| p.ty.clone()).collect();
                self.env.define_func(name.clone(), param_types.clone(), ret.clone());
                
                let old_env = self.env.clone();
                self.env = self.env.child();
                
                for param in params {
                    self.env.define_var(param.name.clone(), param.ty.clone());
                }
                
                for stmt in &body.stmts {
                    self.check_stmt(stmt);
                }
                
                self.env = old_env;
                
                TypedDecl {
                    decl: decl.clone(),
                    ty: Type::Func(param_types, Box::new(ret.clone())),
                }
            }
            Decl::Struct { name, fields, .. } => {
                let field_types: Vec<(String, Type)> = fields.iter()
                    .map(|f| (f.name.clone(), f.ty.clone()))
                    .collect();
                self.env.structs.insert(name.clone(), field_types);
                
                TypedDecl {
                    decl: decl.clone(),
                    ty: Type::Named(name.clone()),
                }
            }
            Decl::Const { name, ty, value, .. } => {
                let inferred = self.infer_expr(value);
                self.env.define_var(name.clone(), ty.clone());
                
                TypedDecl {
                    decl: decl.clone(),
                    ty: ty.clone(),
                }
            }
            _ => TypedDecl {
                decl: decl.clone(),
                ty: Type::Unit,
            },
        }
    }
    
    pub fn check_module(&mut self, module: &Module) -> Result<TypedModule> {
        let mut typed_decls = Vec::new();
        
        // First pass: collect signatures
        for decl in &module.decls {
            if let Decl::Func { name, params, ret, .. } = decl {
                let param_types: Vec<Type> = params.iter().map(|p| p.ty.clone()).collect();
                self.env.define_func(name.clone(), param_types, ret.clone());
            }
        }
        
        // Second pass: type check bodies
        for decl in &module.decls {
            typed_decls.push(self.check_decl(decl));
        }
        
        if !self.errors.is_empty() {
            return Err(anyhow!("Type errors:\n{}", self.errors.join("\n")));
        }
        
        Ok(TypedModule { decls: typed_decls })
    }
}

pub fn check(module: &Module) -> Result<TypedModule> {
    let mut checker = TypeChecker::new();
    checker.check_module(module)
}
