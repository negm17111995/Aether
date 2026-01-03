// AETHER LEXER - Tokenizes Aether source code

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    IntLit(i64),
    StrLit(String),
    Ident(String),
    
    // Keywords
    Func, Let, If, Else, While, Return, Struct, Enum, Const, Import,
    
    // Operators
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, Ne, Lt, Le, Gt, Ge,
    Amp, Pipe, Caret, Bang, Tilde,
    AmpAmp, PipePipe,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Colon, Semicolon, Arrow, Dot,
    
    // Special
    Newline, Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0;
    let mut line = 1;
    let mut col = 1;
    
    while pos < chars.len() {
        let c = chars[pos];
        
        // Skip whitespace (except newlines)
        if c == ' ' || c == '\t' || c == '\r' {
            pos += 1;
            col += 1;
            continue;
        }
        
        // Newline
        if c == '\n' {
            line += 1;
            col = 1;
            pos += 1;
            continue;
        }
        
        // Comments
        if c == '/' && pos + 1 < chars.len() && chars[pos + 1] == '/' {
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }
        
        // Identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            let start = pos;
            while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                pos += 1;
            }
            let ident: String = chars[start..pos].iter().collect();
            let kind = match ident.as_str() {
                "func" => TokenKind::Func,
                "let" => TokenKind::Let,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "while" => TokenKind::While,
                "return" => TokenKind::Return,
                "struct" => TokenKind::Struct,
                "enum" => TokenKind::Enum,
                "const" => TokenKind::Const,
                "import" => TokenKind::Import,
                _ => TokenKind::Ident(ident),
            };
            tokens.push(Token { kind, line, col });
            col += pos - start;
            continue;
        }
        
        // Numbers
        if c.is_digit(10) {
            let start = pos;
            while pos < chars.len() && chars[pos].is_digit(10) {
                pos += 1;
            }
            let num: String = chars[start..pos].iter().collect();
            let val: i64 = num.parse().unwrap_or(0);
            tokens.push(Token { kind: TokenKind::IntLit(val), line, col });
            col += pos - start;
            continue;
        }
        
        // String literals
        if c == '"' {
            pos += 1;
            let start = pos;
            while pos < chars.len() && chars[pos] != '"' {
                pos += 1;
            }
            let s: String = chars[start..pos].iter().collect();
            pos += 1; // skip closing quote
            tokens.push(Token { kind: TokenKind::StrLit(s), line, col });
            continue;
        }
        
        // Two-character operators
        if pos + 1 < chars.len() {
            let two: String = chars[pos..pos+2].iter().collect();
            let kind = match two.as_str() {
                "==" => Some(TokenKind::EqEq),
                "!=" => Some(TokenKind::Ne),
                "<=" => Some(TokenKind::Le),
                ">=" => Some(TokenKind::Ge),
                "&&" => Some(TokenKind::AmpAmp),
                "||" => Some(TokenKind::PipePipe),
                "->" => Some(TokenKind::Arrow),
                _ => None,
            };
            if let Some(k) = kind {
                tokens.push(Token { kind: k, line, col });
                pos += 2;
                col += 2;
                continue;
            }
        }
        
        // Single-character tokens
        let kind = match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '=' => TokenKind::Eq,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            '&' => TokenKind::Amp,
            '|' => TokenKind::Pipe,
            '^' => TokenKind::Caret,
            '!' => TokenKind::Bang,
            '~' => TokenKind::Tilde,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Dot,
            _ => {
                pos += 1;
                col += 1;
                continue;
            }
        };
        
        tokens.push(Token { kind, line, col });
        pos += 1;
        col += 1;
    }
    
    tokens.push(Token { kind: TokenKind::Eof, line, col });
    tokens
}
