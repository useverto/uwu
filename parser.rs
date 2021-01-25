use crate::ast::*;
use crate::tokenizer::Lexer;
use crate::tokens::{Node as Token, Number, Token as TokenLit};
use std::fmt;

/// Represents a kind of parser error
#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

// TODO: Lots of optimizations pending...
impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected Token"),
        }
    }
}

/// Represents a parser error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub msg: String,
    pub current_token: Token,
}

impl ParseError {
    fn new(kind: ParseErrorKind, msg: String, current_token: Token) -> Self {
        ParseError {
            kind,
            msg,
            current_token,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

/// Multiple parser errors
pub type ParseErrors = Vec<ParseError>;

/// Represents an AST Parser
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Token,
    pub next_token: Token,
    pub errors: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token {
                token: TokenLit::Eof,
                ch: "".to_string(),
                loc: 0,
            },
            next_token: Token {
                token: TokenLit::Eof,
                ch: "".to_string(),
                loc: 0,
            },
            errors: vec![],
        };

        parser.bump();
        parser.bump();

        parser
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok.token {
            TokenLit::Equal | TokenLit::NotEqual => Precedence::Equals,
            TokenLit::LessThan | TokenLit::LessThanEqual => Precedence::LessGreater,
            TokenLit::GreaterThan | TokenLit::GreaterThanEqual => Precedence::LessGreater,
            TokenLit::Plus | TokenLit::Minus => Precedence::Sum,
            TokenLit::Slash | TokenLit::Asterisk => Precedence::Product,
            // FIXME: implement precendence for modulo & pow
            TokenLit::Caret | TokenLit::Percent => Precedence::Sum,
            TokenLit::Lbracket => Precedence::Index,
            TokenLit::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    pub fn get_errors(&mut self) -> ParseErrors {
        self.errors.clone()
    }

    fn bump(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn current_token_is(&mut self, tok: TokenLit) -> bool {
        self.current_token.token == tok
    }

    fn next_token_is(&mut self, tok: &TokenLit) -> bool {
        self.next_token.token == *tok
    }

    fn expect_next_token(&mut self, tok: TokenLit) -> bool {
        if self.next_token_is(&tok) || self.next_token_is(&TokenLit::Blank) {
            self.bump();
            return true;
        } else {
            self.error_next_token(tok);
            return false;
        }
    }

    fn current_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.current_token)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.next_token)
    }

    fn error_next_token(&mut self, tok: TokenLit) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!(
                "expected {:?}, got {:?} instead",
                tok, self.next_token.token
            ),
            self.current_token.clone(),
        ));
    }

    fn error_no_prefix_parser(&mut self) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!("Unexpected token `{}` found", self.current_token.ch),
            self.current_token.clone(),
        ));
    }

    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.current_token_is(TokenLit::Eof) {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.bump();
        }

        program
    }

    fn parse_inline_stmt(&mut self) -> BlockStmt {
        self.bump();

        let mut block = vec![];

        while !self.current_token_is(TokenLit::Blank) && !self.current_token_is(TokenLit::Eof) {
            match self.parse_stmt() {
                Some(stmt) => block.push(stmt),
                None => {}
            }
            self.bump();
        }

        block
    }

    fn parse_block_stmt(&mut self) -> BlockStmt {
        self.bump();

        let mut block = vec![];

        while !self.current_token_is(TokenLit::Rbrace) && !self.current_token_is(TokenLit::Eof) {
            match self.parse_stmt() {
                Some(stmt) => block.push(stmt),
                None => {}
            }
            self.bump();
        }

        block
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.current_token.token {
            TokenLit::Blank => Some(Stmt::Blank),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&TokenLit::Semicolon) {
                    self.bump();
                }
                Some(Stmt::Expr(expr))
            }
            None => None,
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        // prefix
        let mut left = match self.current_token.token {
            TokenLit::Ident(_) => self.parse_ident_expr(),
            TokenLit::Number(_) => self.parse_int_expr(),
            TokenLit::String(_) => self.parse_string_expr(),
            TokenLit::Bool(_) => self.parse_bool_expr(),
            TokenLit::Lbracket => self.parse_array_expr(),
            TokenLit::Lbrace => self.parse_hash_expr(),
            TokenLit::Percent
            | TokenLit::Caret
            | TokenLit::Bang
            | TokenLit::Minus
            | TokenLit::Plus => self.parse_prefix_expr(),
            TokenLit::Lparen => self.parse_grouped_expr(),
            TokenLit::If => self.parse_if_expr(),
            TokenLit::While => self.parse_while_expr(),
            TokenLit::Func => self.parse_func_expr(),
            TokenLit::Blank => {
                self.bump();
                return self.parse_expr(Precedence::Lowest);
            }
            _ => {
                self.error_no_prefix_parser();
                return None;
            }
        };

        while !self.next_token_is(&TokenLit::Semicolon) && precedence < self.next_token_precedence()
        {
            match self.next_token.token {
                TokenLit::Plus
                | TokenLit::Minus
                | TokenLit::Slash
                | TokenLit::Asterisk
                | TokenLit::Caret
                | TokenLit::Percent
                | TokenLit::Equal
                | TokenLit::NotEqual
                | TokenLit::LessThan
                | TokenLit::LessThanEqual
                | TokenLit::GreaterThan
                | TokenLit::GreaterThanEqual => {
                    self.bump();
                    left = self.parse_infix_expr(left.unwrap());
                }
                TokenLit::Lbracket => {
                    self.bump();
                    left = self.parse_index_expr(left.unwrap());
                }
                TokenLit::Lparen => {
                    self.bump();
                    left = self.parse_call_expr(left.unwrap());
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        match self.current_token.token {
            TokenLit::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        if !self.next_token_is(&TokenLit::Assign) {
            match self.parse_ident() {
                Some(ident) => return Some(Expr::Ident(ident)),
                None => return None,
            };
        }

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };
        self.bump();

        match self.current_token.token {
            TokenLit::Assign => self.bump(),
            _ => return self.parse_expr(Precedence::Lowest),
        }

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&TokenLit::Semicolon) {
            self.bump();
        }

        Some(Expr::Let(name, Box::new(expr)))
    }

    fn parse_func_expr(&mut self) -> Option<Expr> {
        self.bump();

        let name = self.parse_ident();

        if !self.expect_next_token(TokenLit::Lparen) {
            return None;
        }

        let params = match self.parse_func_params() {
            Some(params) => params,
            None => return None,
        };
        // Inline function declration
        //
        // `fn sqrt(x): x ^ 2`
        if self.next_token_is(&TokenLit::Colon) {
            self.bump();
            return Some(Expr::Func {
                params,
                body: self.parse_inline_stmt(),
                name,
            });
        }

        if !self.expect_next_token(TokenLit::Lbrace) {
            return None;
        }

        Some(Expr::Func {
            params,
            body: self.parse_block_stmt(),
            name,
        })
    }

    fn parse_func_params(&mut self) -> Option<Vec<Ident>> {
        let mut params = vec![];

        if self.next_token_is(&TokenLit::Rparen) {
            self.bump();
            return Some(params);
        }

        self.bump();

        match self.parse_ident() {
            Some(ident) => params.push(ident),
            None => return None,
        };

        while self.next_token_is(&TokenLit::Comma) {
            self.bump();
            self.bump();

            match self.parse_ident() {
                Some(ident) => params.push(ident),
                None => return None,
            };
        }

        if !self.expect_next_token(TokenLit::Rparen) {
            return None;
        }

        Some(params)
    }

    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.current_token.token {
            TokenLit::Number(ref int) => match int {
                Number::Int(i) => Some(Expr::Literal(Literal::Number(Number::Int(i.clone())))),
                Number::Float(i) => Some(Expr::Literal(Literal::Number(Number::Float(i.clone())))),
            },
            _ => None,
        }
    }

    fn parse_string_expr(&mut self) -> Option<Expr> {
        match self.current_token.token {
            TokenLit::String(ref mut s) => Some(Expr::Literal(Literal::String(s.clone()))),
            _ => None,
        }
    }

    fn parse_bool_expr(&mut self) -> Option<Expr> {
        match self.current_token.token {
            TokenLit::Bool(value) => Some(Expr::Literal(Literal::Bool(value == true))),
            _ => None,
        }
    }

    fn parse_array_expr(&mut self) -> Option<Expr> {
        match self.parse_expr_list(TokenLit::Rbracket) {
            Some(list) => Some(Expr::Literal(Literal::Array(list))),
            None => None,
        }
    }

    fn parse_hash_expr(&mut self) -> Option<Expr> {
        let mut pairs = Vec::new();

        while !self.next_token_is(&TokenLit::Rbrace) {
            self.bump();

            let key = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            if !self.expect_next_token(TokenLit::Colon) {
                return None;
            }

            self.bump();

            let value = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            pairs.push((key, value));

            if !self.next_token_is(&TokenLit::Rbrace) && !self.expect_next_token(TokenLit::Comma) {
                return None;
            }
        }

        if !self.expect_next_token(TokenLit::Rbrace) {
            return None;
        }

        Some(Expr::Literal(Literal::Hash(pairs)))
    }

    fn parse_expr_list(&mut self, end: TokenLit) -> Option<Vec<Expr>> {
        let mut list = vec![];

        if self.next_token_is(&end) {
            self.bump();
            return Some(list);
        }

        self.bump();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => return None,
        }

        while self.next_token_is(&TokenLit::Comma) {
            self.bump();
            self.bump();

            match self.parse_expr(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => return None,
            }
        }

        if !self.expect_next_token(end) {
            return None;
        }

        Some(list)
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.current_token.token {
            TokenLit::Bang => Prefix::Not,
            TokenLit::Minus => Prefix::Minus,
            TokenLit::Plus => Prefix::Plus,
            _ => return None,
        };

        self.bump();

        match self.parse_expr(Precedence::Prefix) {
            Some(expr) => Some(Expr::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.current_token.token {
            TokenLit::Plus => Infix::Plus,
            TokenLit::Minus => Infix::Minus,
            TokenLit::Slash => Infix::Divide,
            TokenLit::Asterisk => Infix::Multiply,
            TokenLit::Equal => Infix::Equal,
            TokenLit::NotEqual => Infix::NotEqual,
            TokenLit::LessThan => Infix::LessThan,
            TokenLit::LessThanEqual => Infix::LessThanEqual,
            TokenLit::GreaterThan => Infix::GreaterThan,
            TokenLit::GreaterThanEqual => Infix::GreaterThanEqual,
            TokenLit::Caret => Infix::Power,
            TokenLit::Percent => Infix::Modulo,
            _ => return None,
        };

        let precedence = self.current_token_precedence();

        self.bump();

        match self.parse_expr(precedence) {
            Some(expr) => Some(Expr::Infix(infix, Box::new(left), Box::new(expr))),
            None => None,
        }
    }

    fn parse_index_expr(&mut self, left: Expr) -> Option<Expr> {
        self.bump();

        let index = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(TokenLit::Rbracket) {
            return None;
        }

        Some(Expr::Index(Box::new(left), Box::new(index)))
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.bump();

        let expr = self.parse_expr(Precedence::Lowest);

        if !self.expect_next_token(TokenLit::Rparen) {
            None
        } else {
            expr
        }
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.expect_next_token(TokenLit::Lparen) {
            return None;
        }

        self.bump();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(TokenLit::Rparen) || !self.expect_next_token(TokenLit::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_stmt();
        let mut alternative = None;

        if self.next_token_is(&TokenLit::Else) {
            self.bump();

            if !self.expect_next_token(TokenLit::Lbrace) {
                return None;
            }

            alternative = Some(self.parse_block_stmt());
        }

        Some(Expr::If {
            cond: Box::new(cond),
            consequence,
            alternative,
        })
    }

    fn parse_while_expr(&mut self) -> Option<Expr> {
        if !self.expect_next_token(TokenLit::Lparen) {
            return None;
        }

        self.bump();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(TokenLit::Rparen) || !self.expect_next_token(TokenLit::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_stmt();

        Some(Expr::While {
            cond: Box::new(cond),
            consequence,
        })
    }

    fn parse_call_expr(&mut self, func: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(TokenLit::Rparen) {
            Some(args) => args,
            None => return None,
        };

        Some(Expr::Call {
            func: Box::new(func),
            args,
        })
    }
}
