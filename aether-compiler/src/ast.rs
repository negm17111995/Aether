//! Aether AST - Abstract Syntax Tree definitions
//!
//! Complete AST for all Aether constructs

use std::fmt;

/// Source location
#[derive(Debug, Clone, Copy, Default)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

/// Type representation
#[derive(Debug, Clone)]
pub enum Type {
    /// Named type: Int, String, MyStruct
    Named(String),
    /// Pointer type: *T
    Ptr(Box<Type>),
    /// Array type: [T; N] or [T]
    Array(Box<Type>, Option<usize>),
    /// Generic type: Option<T>, Result<T, E>
    Generic(String, Vec<Type>),
    /// Function type: func(A, B) -> C
    Func(Vec<Type>, Box<Option<Type>>),
    /// Inferred type (placeholder)
    Infer,
    /// Unit type (void)
    Unit,
}

impl Type {
    pub fn is_int(&self) -> bool {
        matches!(self, Type::Named(n) if n == "Int")
    }
    
    pub fn is_bool(&self) -> bool {
        matches!(self, Type::Named(n) if n == "Bool")
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Named(n) => write!(f, "{}", n),
            Type::Ptr(t) => write!(f, "*{}", t),
            Type::Array(t, Some(n)) => write!(f, "[{}; {}]", t, n),
            Type::Array(t, None) => write!(f, "[{}]", t),
            Type::Generic(name, args) => {
                write!(f, "{}<", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ">")
            }
            Type::Func(params, ret) => {
                write!(f, "func(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")?;
                if let Some(r) = ret.as_ref() {
                    write!(f, " -> {}", r)?;
                }
                Ok(())
            }
            Type::Infer => write!(f, "_"),
            Type::Unit => write!(f, "()"),
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, BitAnd, BitOr, BitXor, Shl, Shr,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg, Not, BitNot, Ref, Deref,
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal
    Int(i64, Span),
    /// Float literal
    Float(f64, Span),
    /// String literal
    String(String, Span),
    /// Boolean literal
    Bool(bool, Span),
    /// Identifier
    Ident(String, Span),
    /// Binary operation
    Binary(BinOp, Box<Expr>, Box<Expr>, Span),
    /// Unary operation
    Unary(UnOp, Box<Expr>, Span),
    /// Function call
    Call(Box<Expr>, Vec<Expr>, Span),
    /// Method call: obj.method(args)
    MethodCall(Box<Expr>, String, Vec<Expr>, Span),
    /// Field access: obj.field
    Field(Box<Expr>, String, Span),
    /// Array index: arr[i]
    Index(Box<Expr>, Box<Expr>, Span),
    /// Array literal: [a, b, c]
    Array(Vec<Expr>, Span),
    /// Struct literal: Point { x: 1, y: 2 }
    Struct(String, Vec<(String, Expr)>, Span),
    /// If expression
    If(Box<Expr>, Box<Block>, Option<Box<Block>>, Span),
    /// Lambda: |x, y| x + y
    Lambda(Vec<Param>, Option<Type>, Box<Expr>, Span),
    /// Match expression
    Match(Box<Expr>, Vec<MatchArm>, Span),
    /// Path: module::item
    Path(Vec<String>, Span),
    /// Spawn thread: spawn(func, args)
    Spawn(Box<Expr>, Vec<Expr>, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Int(_, s) | Expr::Float(_, s) | Expr::String(_, s) |
            Expr::Bool(_, s) | Expr::Ident(_, s) | Expr::Binary(_, _, _, s) |
            Expr::Unary(_, _, s) | Expr::Call(_, _, s) | Expr::Field(_, _, s) |
            Expr::Index(_, _, s) | Expr::Array(_, s) | Expr::Struct(_, _, s) |
            Expr::If(_, _, _, s) | Expr::Lambda(_, _, _, s) | Expr::Match(_, _, s) |
            Expr::MethodCall(_, _, _, s) | Expr::Path(_, s) | Expr::Spawn(_, _, s) => *s,
        }
    }
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

/// Pattern for matching
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Ident(String),
    Literal(Expr),
    Tuple(Vec<Pattern>),
    Struct(String, Vec<(String, Pattern)>),
    Enum(String, String, Vec<Pattern>),
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub default: Option<Expr>,
    pub span: Span,
}

/// Statement
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Let binding
    Let {
        name: String,
        ty: Option<Type>,
        init: Option<Expr>,
        mutable: bool,
        span: Span,
    },
    /// Expression statement
    Expr(Expr, Span),
    /// Assignment
    Assign(Expr, Expr, Span),
    /// Return
    Return(Option<Expr>, Span),
    /// If statement
    If(Expr, Block, Option<Block>, Span),
    /// While loop
    While(Expr, Block, Span),
    /// For loop
    For(String, Expr, Block, Span),
    /// Break
    Break(Span),
    /// Continue
    Continue(Span),
    /// Block
    Block(Block, Span),
}

/// Block of statements
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// Struct field
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub public: bool,
    pub span: Span,
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Type>,
    pub span: Span,
}

/// Declaration
#[derive(Debug, Clone)]
pub enum Decl {
    /// Function declaration
    Func {
        name: String,
        generics: Vec<String>,
        params: Vec<Param>,
        ret: Option<Type>,
        body: Block,
        public: bool,
        span: Span,
    },
    /// Struct declaration
    Struct {
        name: String,
        generics: Vec<String>,
        fields: Vec<Field>,
        public: bool,
        span: Span,
    },
    /// Enum declaration
    Enum {
        name: String,
        generics: Vec<String>,
        variants: Vec<Variant>,
        public: bool,
        span: Span,
    },
    /// Trait declaration
    Trait {
        name: String,
        generics: Vec<String>,
        methods: Vec<Decl>,
        public: bool,
        span: Span,
    },
    /// Impl block
    Impl {
        trait_name: Option<String>,
        type_name: String,
        generics: Vec<String>,
        methods: Vec<Decl>,
        span: Span,
    },
    /// Constant
    Const {
        name: String,
        ty: Type,
        value: Expr,
        public: bool,
        span: Span,
    },
    /// Type alias
    TypeAlias {
        name: String,
        generics: Vec<String>,
        ty: Type,
        public: bool,
        span: Span,
    },
    /// Import
    Import {
        path: Vec<String>,
        alias: Option<String>,
        span: Span,
    },
    /// Static/global mutable variable
    Static {
        name: String,
        ty: Option<Type>,
        value: Option<Expr>,
        mutable: bool,
        public: bool,
        span: Span,
    },
}

impl Decl {
    pub fn name(&self) -> Option<&str> {
        match self {
            Decl::Func { name, .. } | Decl::Struct { name, .. } |
            Decl::Enum { name, .. } | Decl::Trait { name, .. } |
            Decl::Const { name, .. } | Decl::TypeAlias { name, .. } |
            Decl::Static { name, .. } => Some(name),
            Decl::Impl { type_name, .. } => Some(type_name),
            Decl::Import { .. } => None,
        }
    }
}

/// Module (compilation unit)
#[derive(Debug, Clone)]
pub struct Module {
    pub decls: Vec<Decl>,
    pub span: Span,
}
