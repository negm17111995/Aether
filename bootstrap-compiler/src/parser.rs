// AETHER PARSER - Builds AST from tokens

use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    StrLit(String),
    Ident(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
    BuiltinCall(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, BitAnd, BitOr, BitXor,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg, Not, BitNot,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, init: Option<Expr> },
    Assign { target: String, value: Expr },
    Return(Option<Expr>),
    If { cond: Expr, then_block: Vec<Stmt>, else_block: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub typ: String,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Func(Func),
    Const { name: String, value: Expr },
    Import(String),
}

#[derive(Debug)]
pub struct Module {
    pub decls: Vec<Decl>,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    fn current(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }
    
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
    
    fn expect(&mut self, kind: TokenKind) -> bool {
        if std::mem::discriminant(&self.current().kind) == std::mem::discriminant(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn parse_expr(&mut self) -> Expr {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        while matches!(self.current().kind, TokenKind::PipePipe) {
            self.advance();
            let right = self.parse_and();
            left = Expr::Binary(Box::new(left), BinOp::Or, Box::new(right));
        }
        left
    }
    
    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        while matches!(self.current().kind, TokenKind::AmpAmp) {
            self.advance();
            let right = self.parse_comparison();
            left = Expr::Binary(Box::new(left), BinOp::And, Box::new(right));
        }
        left
    }
    
    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_additive();
        loop {
            let op = match self.current().kind {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::Ne => BinOp::Ne,
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    
    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();
        loop {
            let op = match self.current().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                TokenKind::Pipe => BinOp::BitOr,
                TokenKind::Caret => BinOp::BitXor,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    
    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_unary();
        loop {
            let op = match self.current().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Mod,
                TokenKind::Amp => BinOp::BitAnd,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary();
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        left
    }
    
    fn parse_unary(&mut self) -> Expr {
        match self.current().kind {
            TokenKind::Minus => {
                self.advance();
                Expr::Unary(UnaryOp::Neg, Box::new(self.parse_unary()))
            }
            TokenKind::Bang => {
                self.advance();
                Expr::Unary(UnaryOp::Not, Box::new(self.parse_unary()))
            }
            TokenKind::Tilde => {
                self.advance();
                Expr::Unary(UnaryOp::BitNot, Box::new(self.parse_unary()))
            }
            _ => self.parse_primary(),
        }
    }
    
    fn parse_primary(&mut self) -> Expr {
        match &self.current().kind {
            TokenKind::IntLit(n) => {
                let n = *n;
                self.advance();
                Expr::IntLit(n)
            }
            TokenKind::StrLit(s) => {
                let s = s.clone();
                self.advance();
                Expr::StrLit(s)
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                
                // Check for function call
                if matches!(self.current().kind, TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.current().kind, TokenKind::RParen | TokenKind::Eof) {
                        args.push(self.parse_expr());
                        if !matches!(self.current().kind, TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect(TokenKind::RParen);
                    
                    // Check for builtin
                    if name.starts_with("__builtin_") {
                        Expr::BuiltinCall(name, args)
                    } else {
                        Expr::Call(name, args)
                    }
                } else {
                    Expr::Ident(name)
                }
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(TokenKind::RParen);
                expr
            }
            _ => {
                self.advance();
                Expr::IntLit(0)
            }
        }
    }
    
    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.expect(TokenKind::LBrace);
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt());
        }
        self.expect(TokenKind::RBrace);
        stmts
    }
    
    fn parse_stmt(&mut self) -> Stmt {
        match &self.current().kind {
            TokenKind::Let => {
                self.advance();
                let name = if let TokenKind::Ident(n) = &self.current().kind {
                    let n = n.clone();
                    self.advance();
                    n
                } else {
                    "?".to_string()
                };
                
                // Optional type annotation
                if matches!(self.current().kind, TokenKind::Colon) {
                    self.advance();
                    // Skip type for now
                    if let TokenKind::Ident(_) = &self.current().kind {
                        self.advance();
                    }
                }
                
                let init = if matches!(self.current().kind, TokenKind::Eq) {
                    self.advance();
                    Some(self.parse_expr())
                } else {
                    None
                };
                
                Stmt::Let { name, init }
            }
            TokenKind::Return => {
                self.advance();
                let value = if matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
                    None
                } else {
                    Some(self.parse_expr())
                };
                Stmt::Return(value)
            }
            TokenKind::If => {
                self.advance();
                let cond = self.parse_expr();
                let then_block = self.parse_block();
                let else_block = if matches!(self.current().kind, TokenKind::Else) {
                    self.advance();
                    if matches!(self.current().kind, TokenKind::If) {
                        Some(vec![self.parse_stmt()])
                    } else {
                        Some(self.parse_block())
                    }
                } else {
                    None
                };
                Stmt::If { cond, then_block, else_block }
            }
            TokenKind::While => {
                self.advance();
                let cond = self.parse_expr();
                let body = self.parse_block();
                Stmt::While { cond, body }
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                
                if matches!(self.current().kind, TokenKind::Eq) {
                    self.advance();
                    let value = self.parse_expr();
                    Stmt::Assign { target: name, value }
                } else if matches!(self.current().kind, TokenKind::LParen) {
                    // Function call as statement
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.current().kind, TokenKind::RParen | TokenKind::Eof) {
                        args.push(self.parse_expr());
                        if !matches!(self.current().kind, TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect(TokenKind::RParen);
                    
                    if name.starts_with("__builtin_") {
                        Stmt::Expr(Expr::BuiltinCall(name, args))
                    } else {
                        Stmt::Expr(Expr::Call(name, args))
                    }
                } else {
                    Stmt::Expr(Expr::Ident(name))
                }
            }
            _ => {
                let expr = self.parse_expr();
                Stmt::Expr(expr)
            }
        }
    }
    
    fn parse_func(&mut self) -> Func {
        self.expect(TokenKind::Func);
        
        let name = if let TokenKind::Ident(n) = &self.current().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            "?".to_string()
        };
        
        // Parameters
        self.expect(TokenKind::LParen);
        let mut params = Vec::new();
        while !matches!(self.current().kind, TokenKind::RParen | TokenKind::Eof) {
            if let TokenKind::Ident(pname) = &self.current().kind {
                let pname = pname.clone();
                self.advance();
                self.expect(TokenKind::Colon);
                let ptype = if let TokenKind::Ident(t) = &self.current().kind {
                    let t = t.clone();
                    self.advance();
                    t
                } else {
                    "Int".to_string()
                };
                params.push(Param { name: pname, typ: ptype });
            }
            if !matches!(self.current().kind, TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        self.expect(TokenKind::RParen);
        
        // Return type
        let ret_type = if matches!(self.current().kind, TokenKind::Arrow) {
            self.advance();
            if let TokenKind::Ident(t) = &self.current().kind {
                let t = t.clone();
                self.advance();
                Some(t)
            } else {
                None
            }
        } else {
            None
        };
        
        let body = self.parse_block();
        
        Func { name, params, ret_type, body }
    }
    
    pub fn parse_module(&mut self) -> Module {
        let mut decls = Vec::new();
        
        while !matches!(self.current().kind, TokenKind::Eof) {
            match &self.current().kind {
                TokenKind::Func => {
                    decls.push(Decl::Func(self.parse_func()));
                }
                TokenKind::Const => {
                    self.advance();
                    let name = if let TokenKind::Ident(n) = &self.current().kind {
                        let n = n.clone();
                        self.advance();
                        n
                    } else {
                        "?".to_string()
                    };
                    // Skip type
                    if matches!(self.current().kind, TokenKind::Colon) {
                        self.advance();
                        self.advance(); // skip type name
                    }
                    self.expect(TokenKind::Eq);
                    let value = self.parse_expr();
                    decls.push(Decl::Const { name, value });
                }
                TokenKind::Import => {
                    self.advance();
                    let mut path = String::new();
                    while let TokenKind::Ident(part) = &self.current().kind {
                        if !path.is_empty() {
                            path.push('.');
                        }
                        path.push_str(part);
                        self.advance();
                        if !matches!(self.current().kind, TokenKind::Dot) {
                            break;
                        }
                        self.advance();
                    }
                    decls.push(Decl::Import(path));
                }
                _ => {
                    self.advance();
                }
            }
        }
        
        Module { decls }
    }
}

pub fn parse(tokens: &[Token]) -> Module {
    let mut parser = Parser::new(tokens);
    parser.parse_module()
}
