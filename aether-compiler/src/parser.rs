//! Aether Parser - Recursive descent parser
//!
//! Parses tokens into a complete AST

use crate::lexer::{Token, TokenKind};
use crate::ast::*;
use anyhow::{anyhow, Result};

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }
    
    fn peek_kind(&self) -> TokenKind {
        self.peek().kind
    }
    
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }
    
    fn check(&self, kind: TokenKind) -> bool {
        self.peek_kind() == kind
    }
    
    fn match_tok(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn expect(&mut self, kind: TokenKind) -> Result<Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(anyhow!("Expected {:?}, got {:?} at line {}", kind, self.peek_kind(), self.peek().line))
        }
    }
    
    fn span(&self) -> Span {
        let tok = self.peek();
        Span { line: tok.line, col: tok.col }
    }
    
    // ========== TYPE PARSING ==========
    
    fn parse_type(&mut self) -> Result<Type> {
        // Pointer: *Type
        if self.match_tok(TokenKind::Star) {
            let inner = self.parse_type()?;
            return Ok(Type::Ptr(Box::new(inner)));
        }
        
        // Array: [Type] or [Type; N]
        if self.match_tok(TokenKind::LBrack) {
            let elem = self.parse_type()?;
            let size = if self.match_tok(TokenKind::Semi) {
                let tok = self.expect(TokenKind::Int)?;
                Some(tok.int_value.unwrap_or(0) as usize)
            } else {
                None
            };
            self.expect(TokenKind::RBrack)?;
            return Ok(Type::Array(Box::new(elem), size));
        }
        
        // Function type: func(A, B) -> C
        if self.match_tok(TokenKind::Func) {
            self.expect(TokenKind::LParen)?;
            let mut params = Vec::new();
            if !self.check(TokenKind::RParen) {
                params.push(self.parse_type()?);
                while self.match_tok(TokenKind::Comma) {
                    params.push(self.parse_type()?);
                }
            }
            self.expect(TokenKind::RParen)?;
            let ret = if self.match_tok(TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };
            return Ok(Type::Func(params, Box::new(ret)));
        }
        
        // Named type
        if self.check(TokenKind::Ident) {
            let name = self.advance().lexeme.clone();
            
            // Generics: Name<A, B>
            if self.match_tok(TokenKind::Lt) {
                let mut args = Vec::new();
                args.push(self.parse_type()?);
                while self.match_tok(TokenKind::Comma) {
                    args.push(self.parse_type()?);
                }
                self.expect(TokenKind::Gt)?;
                return Ok(Type::Generic(name, args));
            }
            
            return Ok(Type::Named(name));
        }
        
        Err(anyhow!("Expected type at line {}", self.peek().line))
    }
    
    // ========== EXPRESSION PARSING ==========
    
    fn parse_primary(&mut self) -> Result<Expr> {
        let span = self.span();
        
        // Literals
        if self.check(TokenKind::Int) {
            let tok = self.advance();
            return Ok(Expr::Int(tok.int_value.unwrap_or(0), span));
        }
        if self.check(TokenKind::Float) {
            let tok = self.advance();
            return Ok(Expr::Float(tok.float_value.unwrap_or(0.0), span));
        }
        if self.check(TokenKind::String) {
            let tok = self.advance();
            return Ok(Expr::String(tok.string_value.clone().unwrap_or_default(), span));
        }
        if self.match_tok(TokenKind::True) {
            return Ok(Expr::Bool(true, span));
        }
        if self.match_tok(TokenKind::False) {
            return Ok(Expr::Bool(false, span));
        }
        
        // Identifier
        if self.check(TokenKind::Ident) {
            let name = self.advance().lexeme.clone();
            return Ok(Expr::Ident(name, span));
        }
        
        // Parenthesized expression
        if self.match_tok(TokenKind::LParen) {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            return Ok(expr);
        }
        
        // Array literal
        if self.match_tok(TokenKind::LBrack) {
            let mut elements = Vec::new();
            if !self.check(TokenKind::RBrack) {
                elements.push(self.parse_expr()?);
                while self.match_tok(TokenKind::Comma) {
                    if self.check(TokenKind::RBrack) { break; }
                    elements.push(self.parse_expr()?);
                }
            }
            self.expect(TokenKind::RBrack)?;
            return Ok(Expr::Array(elements, span));
        }
        
        // If expression
        if self.check(TokenKind::If) {
            return self.parse_if_expr();
        }
        
        Err(anyhow!("Expected expression at line {}", self.peek().line))
    }
    
    fn parse_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;
        
        loop {
            let span = self.span();
            
            // Function call
            if self.match_tok(TokenKind::LParen) {
                let mut args = Vec::new();
                if !self.check(TokenKind::RParen) {
                    args.push(self.parse_expr()?);
                    while self.match_tok(TokenKind::Comma) {
                        args.push(self.parse_expr()?);
                    }
                }
                self.expect(TokenKind::RParen)?;
                expr = Expr::Call(Box::new(expr), args, span);
                continue;
            }
            
            // Index
            if self.match_tok(TokenKind::LBrack) {
                let idx = self.parse_expr()?;
                self.expect(TokenKind::RBrack)?;
                expr = Expr::Index(Box::new(expr), Box::new(idx), span);
                continue;
            }
            
            // Field access
            if self.match_tok(TokenKind::Dot) {
                let field = self.expect(TokenKind::Ident)?.lexeme.clone();
                
                // Method call
                if self.check(TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.check(TokenKind::RParen) {
                        args.push(self.parse_expr()?);
                        while self.match_tok(TokenKind::Comma) {
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    expr = Expr::MethodCall(Box::new(expr), field, args, span);
                } else {
                    expr = Expr::Field(Box::new(expr), field, span);
                }
                continue;
            }
            
            break;
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expr> {
        let span = self.span();
        
        if self.match_tok(TokenKind::Minus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnOp::Neg, Box::new(operand), span));
        }
        if self.match_tok(TokenKind::Bang) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnOp::Not, Box::new(operand), span));
        }
        if self.match_tok(TokenKind::Tilde) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnOp::BitNot, Box::new(operand), span));
        }
        if self.match_tok(TokenKind::Amp) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnOp::Ref, Box::new(operand), span));
        }
        if self.match_tok(TokenKind::Star) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary(UnOp::Deref, Box::new(operand), span));
        }
        
        self.parse_postfix()
    }
    
    fn get_precedence(&self, kind: TokenKind) -> u8 {
        match kind {
            TokenKind::PipePipe => 1,
            TokenKind::AmpAmp => 2,
            TokenKind::Pipe => 3,
            TokenKind::Caret => 4,
            TokenKind::Amp => 5,
            TokenKind::EqEq | TokenKind::Ne => 6,
            TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge => 7,
            TokenKind::Plus | TokenKind::Minus => 8,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 9,
            _ => 0,
        }
    }
    
    fn token_to_binop(&self, kind: TokenKind) -> Option<BinOp> {
        match kind {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            TokenKind::Percent => Some(BinOp::Mod),
            TokenKind::EqEq => Some(BinOp::Eq),
            TokenKind::Ne => Some(BinOp::Ne),
            TokenKind::Lt => Some(BinOp::Lt),
            TokenKind::Le => Some(BinOp::Le),
            TokenKind::Gt => Some(BinOp::Gt),
            TokenKind::Ge => Some(BinOp::Ge),
            TokenKind::AmpAmp => Some(BinOp::And),
            TokenKind::PipePipe => Some(BinOp::Or),
            TokenKind::Amp => Some(BinOp::BitAnd),
            TokenKind::Pipe => Some(BinOp::BitOr),
            TokenKind::Caret => Some(BinOp::BitXor),
            _ => None,
        }
    }
    
    fn parse_binary(&mut self, min_prec: u8) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        
        loop {
            let prec = self.get_precedence(self.peek_kind());
            if prec < min_prec {
                break;
            }
            
            let op_kind = self.peek_kind();
            let span = self.span();
            self.advance();
            
            let op = self.token_to_binop(op_kind).unwrap();
            let right = self.parse_binary(prec + 1)?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), span);
        }
        
        Ok(left)
    }
    
    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_binary(1)
    }
    
    fn parse_if_expr(&mut self) -> Result<Expr> {
        let span = self.span();
        self.expect(TokenKind::If)?;
        let cond = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let else_block = if self.match_tok(TokenKind::Else) {
            if self.check(TokenKind::If) {
                let if_expr = self.parse_if_expr()?;
                Some(Box::new(Block {
                    stmts: vec![Stmt::Expr(if_expr, span)],
                    span,
                }))
            } else {
                Some(Box::new(self.parse_block()?))
            }
        } else {
            None
        };
        Ok(Expr::If(Box::new(cond), Box::new(then_block), else_block, span))
    }
    
    // ========== STATEMENT PARSING ==========
    
    fn parse_block(&mut self) -> Result<Block> {
        let span = self.span();
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        
        while !self.check(TokenKind::RBrace) && !self.check(TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }
        
        self.expect(TokenKind::RBrace)?;
        Ok(Block { stmts, span })
    }
    
    fn parse_stmt(&mut self) -> Result<Stmt> {
        let span = self.span();
        
        // Let
        if self.match_tok(TokenKind::Let) {
            let mutable = self.match_tok(TokenKind::Mut);
            let name = self.expect(TokenKind::Ident)?.lexeme.clone();
            let ty = if self.match_tok(TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            let init = if self.match_tok(TokenKind::Eq) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            return Ok(Stmt::Let { name, ty, init, mutable, span });
        }
        
        // If
        if self.check(TokenKind::If) {
            self.advance();
            let cond = self.parse_expr()?;
            let then_block = self.parse_block()?;
            let else_block = if self.match_tok(TokenKind::Else) {
                Some(self.parse_block()?)
            } else {
                None
            };
            return Ok(Stmt::If(cond, then_block, else_block, span));
        }
        
        // While
        if self.match_tok(TokenKind::While) {
            let cond = self.parse_expr()?;
            let body = self.parse_block()?;
            return Ok(Stmt::While(cond, body, span));
        }
        
        // For
        if self.match_tok(TokenKind::For) {
            let var = self.expect(TokenKind::Ident)?.lexeme.clone();
            self.expect(TokenKind::In)?;
            let iter = self.parse_expr()?;
            let body = self.parse_block()?;
            return Ok(Stmt::For(var, iter, body, span));
        }
        
        // Return
        if self.match_tok(TokenKind::Return) {
            let val = if !self.check(TokenKind::RBrace) && !self.check(TokenKind::Semi) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            return Ok(Stmt::Return(val, span));
        }
        
        // Break
        if self.match_tok(TokenKind::Break) {
            return Ok(Stmt::Break(span));
        }
        
        // Continue
        if self.match_tok(TokenKind::Continue) {
            return Ok(Stmt::Continue(span));
        }
        
        // Block
        if self.check(TokenKind::LBrace) {
            let block = self.parse_block()?;
            return Ok(Stmt::Block(block, span));
        }
        
        // Expression or assignment
        let expr = self.parse_expr()?;
        
        if self.match_tok(TokenKind::Eq) {
            let val = self.parse_expr()?;
            return Ok(Stmt::Assign(expr, val, span));
        }
        if self.match_tok(TokenKind::PlusEq) {
            let val = self.parse_expr()?;
            let add = Expr::Binary(BinOp::Add, Box::new(expr.clone()), Box::new(val), span);
            return Ok(Stmt::Assign(expr, add, span));
        }
        if self.match_tok(TokenKind::MinusEq) {
            let val = self.parse_expr()?;
            let sub = Expr::Binary(BinOp::Sub, Box::new(expr.clone()), Box::new(val), span);
            return Ok(Stmt::Assign(expr, sub, span));
        }
        
        Ok(Stmt::Expr(expr, span))
    }
    
    // ========== DECLARATION PARSING ==========
    
    fn parse_param(&mut self) -> Result<Param> {
        let span = self.span();
        let name = self.expect(TokenKind::Ident)?.lexeme.clone();
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        let default = if self.match_tok(TokenKind::Eq) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Param { name, ty, default, span })
    }
    
    fn parse_func(&mut self, public: bool) -> Result<Decl> {
        let span = self.span();
        self.expect(TokenKind::Func)?;
        let name = self.expect(TokenKind::Ident)?.lexeme.clone();
        
        // Generics
        let generics = if self.match_tok(TokenKind::Lt) {
            let mut gens = Vec::new();
            while !self.check(TokenKind::Gt) {
                gens.push(self.expect(TokenKind::Ident)?.lexeme.clone());
                if !self.check(TokenKind::Gt) {
                    self.expect(TokenKind::Comma)?;
                }
            }
            self.expect(TokenKind::Gt)?;
            gens
        } else {
            Vec::new()
        };
        
        // Parameters
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            params.push(self.parse_param()?);
            while self.match_tok(TokenKind::Comma) {
                params.push(self.parse_param()?);
            }
        }
        self.expect(TokenKind::RParen)?;
        
        // Return type
        let ret = if self.match_tok(TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Body
        let body = self.parse_block()?;
        
        Ok(Decl::Func { name, generics, params, ret, body, public, span })
    }
    
    fn parse_struct(&mut self, public: bool) -> Result<Decl> {
        let span = self.span();
        self.expect(TokenKind::Struct)?;
        let name = self.expect(TokenKind::Ident)?.lexeme.clone();
        
        let generics = if self.match_tok(TokenKind::Lt) {
            let mut gens = Vec::new();
            while !self.check(TokenKind::Gt) {
                gens.push(self.expect(TokenKind::Ident)?.lexeme.clone());
                if !self.check(TokenKind::Gt) { self.expect(TokenKind::Comma)?; }
            }
            self.expect(TokenKind::Gt)?;
            gens
        } else {
            Vec::new()
        };
        
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while !self.check(TokenKind::RBrace) {
            let field_public = self.match_tok(TokenKind::Pub);
            let field_span = self.span();
            let field_name = self.expect(TokenKind::Ident)?.lexeme.clone();
            self.expect(TokenKind::Colon)?;
            let field_ty = self.parse_type()?;
            fields.push(Field { name: field_name, ty: field_ty, public: field_public, span: field_span });
            self.match_tok(TokenKind::Comma);
        }
        self.expect(TokenKind::RBrace)?;
        
        Ok(Decl::Struct { name, generics, fields, public, span })
    }
    
    fn parse_enum(&mut self, public: bool) -> Result<Decl> {
        let span = self.span();
        self.expect(TokenKind::Enum)?;
        let name = self.expect(TokenKind::Ident)?.lexeme.clone();
        
        let generics = if self.match_tok(TokenKind::Lt) {
            let mut gens = Vec::new();
            while !self.check(TokenKind::Gt) {
                gens.push(self.expect(TokenKind::Ident)?.lexeme.clone());
                if !self.check(TokenKind::Gt) { self.expect(TokenKind::Comma)?; }
            }
            self.expect(TokenKind::Gt)?;
            gens
        } else {
            Vec::new()
        };
        
        self.expect(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        while !self.check(TokenKind::RBrace) {
            let var_span = self.span();
            let var_name = self.expect(TokenKind::Ident)?.lexeme.clone();
            let fields = if self.match_tok(TokenKind::LParen) {
                let mut fs = Vec::new();
                if !self.check(TokenKind::RParen) {
                    fs.push(self.parse_type()?);
                    while self.match_tok(TokenKind::Comma) {
                        fs.push(self.parse_type()?);
                    }
                }
                self.expect(TokenKind::RParen)?;
                fs
            } else {
                Vec::new()
            };
            variants.push(Variant { name: var_name, fields, span: var_span });
            self.match_tok(TokenKind::Comma);
        }
        self.expect(TokenKind::RBrace)?;
        
        Ok(Decl::Enum { name, generics, variants, public, span })
    }
    
    fn parse_import(&mut self) -> Result<Decl> {
        let span = self.span();
        self.expect(TokenKind::Import)?;
        let mut path = Vec::new();
        path.push(self.expect(TokenKind::Ident)?.lexeme.clone());
        while self.match_tok(TokenKind::Dot) {
            path.push(self.expect(TokenKind::Ident)?.lexeme.clone());
        }
        Ok(Decl::Import { path, alias: None, span })
    }
    
    fn parse_const(&mut self, public: bool) -> Result<Decl> {
        let span = self.span();
        self.expect(TokenKind::Const)?;
        let name = self.expect(TokenKind::Ident)?.lexeme.clone();
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        Ok(Decl::Const { name, ty, value, public, span })
    }
    
    fn parse_decl(&mut self) -> Result<Decl> {
        let public = self.match_tok(TokenKind::Pub);
        
        match self.peek_kind() {
            TokenKind::Func => self.parse_func(public),
            TokenKind::Struct => self.parse_struct(public),
            TokenKind::Enum => self.parse_enum(public),
            TokenKind::Import => self.parse_import(),
            TokenKind::Const => self.parse_const(public),
            _ => Err(anyhow!("Expected declaration at line {}", self.peek().line)),
        }
    }
    
    pub fn parse_module(&mut self) -> Result<Module> {
        let span = self.span();
        let mut decls = Vec::new();
        
        while !self.check(TokenKind::Eof) {
            decls.push(self.parse_decl()?);
        }
        
        Ok(Module { decls, span })
    }
}

pub fn parse(tokens: &[Token]) -> Result<Module> {
    let mut parser = Parser::new(tokens);
    parser.parse_module()
}
