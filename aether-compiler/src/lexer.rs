//! Aether Lexer - Full tokenizer for Aether syntax
//! 
//! Handles all tokens: keywords, operators, literals, identifiers

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Literals
    Int,
    Float,
    String,
    Char,
    True,
    False,
    
    // Identifiers
    Ident,
    
    // Keywords
    Func,
    Let,
    Mut,
    Const,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    Struct,
    Enum,
    Trait,
    Impl,
    Type,
    Import,
    Pub,
    Self_,
    Match,
    Parallel,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Eq,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    AmpAmp,
    PipePipe,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    Arrow,
    FatArrow,
    ColonColon,
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Comma,
    Colon,
    Semi,
    Dot,
    At,
    Hash,
    Question,
    
    // Special
    Eof,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
    pub int_value: Option<i64>,
    pub float_value: Option<f64>,
    pub string_value: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, line: usize, col: usize) -> Self {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            line,
            col,
            int_value: None,
            float_value: None,
            string_value: None,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    col: usize,
    start: usize,
    current: usize,
    keywords: HashMap<&'static str, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("func", TokenKind::Func);
        keywords.insert("let", TokenKind::Let);
        keywords.insert("mut", TokenKind::Mut);
        keywords.insert("const", TokenKind::Const);
        keywords.insert("if", TokenKind::If);
        keywords.insert("else", TokenKind::Else);
        keywords.insert("while", TokenKind::While);
        keywords.insert("for", TokenKind::For);
        keywords.insert("in", TokenKind::In);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("break", TokenKind::Break);
        keywords.insert("continue", TokenKind::Continue);
        keywords.insert("struct", TokenKind::Struct);
        keywords.insert("enum", TokenKind::Enum);
        keywords.insert("trait", TokenKind::Trait);
        keywords.insert("impl", TokenKind::Impl);
        keywords.insert("type", TokenKind::Type);
        keywords.insert("import", TokenKind::Import);
        keywords.insert("pub", TokenKind::Pub);
        keywords.insert("self", TokenKind::Self_);
        keywords.insert("true", TokenKind::True);
        keywords.insert("false", TokenKind::False);
        keywords.insert("match", TokenKind::Match);
        keywords.insert("parallel", TokenKind::Parallel);
        
        Lexer {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
            col: 1,
            start: 0,
            current: 0,
            keywords,
        }
    }
    
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }
    
    fn advance(&mut self) -> Option<char> {
        if let Some((i, c)) = self.chars.next() {
            self.current = i + c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '/' => {
                    let next = self.source[self.current..].chars().nth(1);
                    if next == Some('/') {
                        // Line comment
                        while self.peek() != Some('\n') && self.peek().is_some() {
                            self.advance();
                        }
                    } else if next == Some('*') {
                        // Block comment
                        self.advance(); // /
                        self.advance(); // *
                        while let Some(c) = self.advance() {
                            if c == '*' && self.peek() == Some('/') {
                                self.advance();
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    
    fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }
    
    fn make_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.lexeme(), self.line, self.col)
    }
    
    fn number(&mut self) -> Token {
        // Check for hex literal (0x...)
        if self.lexeme() == "0" && self.peek() == Some('x') {
            self.advance(); // consume 'x'
            while self.peek().map_or(false, |c| c.is_ascii_hexdigit()) {
                self.advance();
            }
            let mut tok = self.make_token(TokenKind::Int);
            let hex_str = &self.lexeme()[2..]; // skip "0x"
            tok.int_value = i64::from_str_radix(hex_str, 16).ok();
            return tok;
        }
        
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }
        
        // Check for float
        if self.peek() == Some('.') {
            let next = self.source[self.current..].chars().nth(1);
            if next.map_or(false, |c| c.is_ascii_digit()) {
                self.advance(); // consume '.'
                while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    self.advance();
                }
                let mut tok = self.make_token(TokenKind::Float);
                tok.float_value = self.lexeme().parse().ok();
                return tok;
            }
        }
        
        let mut tok = self.make_token(TokenKind::Int);
        tok.int_value = self.lexeme().parse().ok();
        tok
    }
    
    fn identifier(&mut self) -> Token {
        while self.peek().map_or(false, |c| c.is_alphanumeric() || c == '_') {
            self.advance();
        }
        
        let text = self.lexeme();
        let kind = self.keywords.get(text).copied().unwrap_or(TokenKind::Ident);
        self.make_token(kind)
    }
    
    fn string(&mut self) -> Token {
        let mut value = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        _ => value.push(escaped),
                    }
                }
            } else {
                self.advance();
                value.push(c);
            }
        }
        self.advance(); // closing quote
        
        let mut tok = self.make_token(TokenKind::String);
        tok.string_value = Some(value);
        tok
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        
        let c = match self.advance() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, "", self.line, self.col),
        };
        
        match c {
            // Single char tokens
            '(' => self.make_token(TokenKind::LParen),
            ')' => self.make_token(TokenKind::RParen),
            '{' => self.make_token(TokenKind::LBrace),
            '}' => self.make_token(TokenKind::RBrace),
            '[' => self.make_token(TokenKind::LBrack),
            ']' => self.make_token(TokenKind::RBrack),
            ',' => self.make_token(TokenKind::Comma),
            ';' => self.make_token(TokenKind::Semi),
            '.' => self.make_token(TokenKind::Dot),
            '@' => self.make_token(TokenKind::At),
            '#' => self.make_token(TokenKind::Hash),
            '?' => self.make_token(TokenKind::Question),
            '~' => self.make_token(TokenKind::Tilde),
            
            // One or two char tokens
            '+' => {
                if self.match_char('=') { self.make_token(TokenKind::PlusEq) }
                else { self.make_token(TokenKind::Plus) }
            }
            '-' => {
                if self.match_char('>') { self.make_token(TokenKind::Arrow) }
                else if self.match_char('=') { self.make_token(TokenKind::MinusEq) }
                else { self.make_token(TokenKind::Minus) }
            }
            '*' => {
                if self.match_char('=') { self.make_token(TokenKind::StarEq) }
                else { self.make_token(TokenKind::Star) }
            }
            '/' => {
                if self.match_char('=') { self.make_token(TokenKind::SlashEq) }
                else { self.make_token(TokenKind::Slash) }
            }
            '%' => self.make_token(TokenKind::Percent),
            '=' => {
                if self.match_char('=') { self.make_token(TokenKind::EqEq) }
                else if self.match_char('>') { self.make_token(TokenKind::FatArrow) }
                else { self.make_token(TokenKind::Eq) }
            }
            '!' => {
                if self.match_char('=') { self.make_token(TokenKind::Ne) }
                else { self.make_token(TokenKind::Bang) }
            }
            '<' => {
                if self.match_char('=') { self.make_token(TokenKind::Le) }
                else { self.make_token(TokenKind::Lt) }
            }
            '>' => {
                if self.match_char('=') { self.make_token(TokenKind::Ge) }
                else { self.make_token(TokenKind::Gt) }
            }
            '&' => {
                if self.match_char('&') { self.make_token(TokenKind::AmpAmp) }
                else { self.make_token(TokenKind::Amp) }
            }
            '|' => {
                if self.match_char('|') { self.make_token(TokenKind::PipePipe) }
                else { self.make_token(TokenKind::Pipe) }
            }
            '^' => self.make_token(TokenKind::Caret),
            ':' => {
                if self.match_char(':') { self.make_token(TokenKind::ColonColon) }
                else { self.make_token(TokenKind::Colon) }
            }
            
            // Literals
            '"' => self.string(),
            '\'' => {
                let ch = self.advance();
                if self.peek() == Some('\'') {
                    self.advance();
                }
                let mut tok = self.make_token(TokenKind::Char);
                tok.int_value = ch.map(|c| c as i64);
                tok
            }
            
            // Numbers and identifiers
            _ if c.is_ascii_digit() => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),
            
            _ => self.make_token(TokenKind::Error),
        }
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    
    tokens
}
