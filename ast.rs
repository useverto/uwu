pub use crate::tokens::Number;
use std::fmt;

// ******* AST ******* //
#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Not,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Prefix::Plus => write!(f, "+"),
            Prefix::Minus => write!(f, "-"),
            Prefix::Not => write!(f, "!"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    Power,
    Modulo,
    PlusAssign,
    MulAssign,
    DivAssign,
    SubAssign,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Divide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
            Infix::Equal => write!(f, "=="),
            Infix::NotEqual => write!(f, "!="),
            Infix::GreaterThanEqual => write!(f, ">="),
            Infix::GreaterThan => write!(f, ">"),
            Infix::LessThanEqual => write!(f, "<="),
            Infix::LessThan => write!(f, "<"),
            Infix::Power => write!(f, "**"),
            Infix::Modulo => write!(f, "%"),
            Infix::PlusAssign => write!(f, "+="),
            Infix::MulAssign => write!(f, "*="),
            Infix::DivAssign => write!(f, "/="),
            Infix::SubAssign => write!(f, "-="),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Ident(Ident),
    Let(Ident, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Accessor(Box<Expr>, Ident),
    If {
        cond: Box<Expr>,
        consequence: BlockStmt,
        alternative: Option<BlockStmt>,
    },
    While {
        cond: Box<Expr>,
        consequence: BlockStmt,
    },
    Func {
        params: Vec<Ident>,
        body: BlockStmt,
        name: Option<Ident>,
    },
    Macro {
        name: Box<Expr>,
        args: Vec<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Regexp {
        pattern: Box<Expr>,
        flags: Option<Ident>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Number(Number),
    String(String),
    Bool(bool),
    Array(Vec<Expr>),
    Hash(Vec<(Expr, Expr)>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Blank,
    Expr(Expr),
    Return(Expr),
}

pub type BlockStmt = Vec<(Stmt, usize)>;

pub type Program = BlockStmt;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(x)
    Index,       // array[index]
    Assign,      // x += 1
}
