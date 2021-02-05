// ******* Token ******* //
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub token: Token,
    pub ch: String,
    pub loc: usize,
}

impl Node {
    pub fn new(token: Token, ch: String, loc: usize) -> Self {
        Self { token, ch, loc }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Number {
    Int(i32),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Blank,
    Eof,

    // Identifiers + literals
    Ident(String),
    Number(Number),
    Bool(bool),
    String(String),
    Regexp(String, String),

    // Statements
    Assign,
    If,
    Else,
    While,
    Let,
    Return,

    // Arithmetic Operators
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators#arithmetic_operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Caret,
    Percent,

    // Assignment operators
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators#assignment_operators
    PlusAssign,
    MulAssign,
    DivAssign,
    SubAssign,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Delimiters
    Comma,
    Dot,
    Colon,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    Func,
    End,
}
