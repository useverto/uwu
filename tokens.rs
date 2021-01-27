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

    // Statements
    Assign,
    If,
    Else,
    While,
    Let,

    // Operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Caret,
    Percent,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Delimiters
    Comma,
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
