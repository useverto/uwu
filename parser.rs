use crate::ast::*;
use crate::tokenizer::Lexer;
use crate::tokens::{Node, Number, Token};
use std::fmt;

/// Represents a kind of parser error
#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

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
    pub current_token: Node,
}

impl ParseError {
    fn new(kind: ParseErrorKind, msg: String, current_token: Node) -> Self {
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
    pub current_token: Node,
    pub next_token: Node,
    pub errors: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Node {
                token: Token::Eof,
                ch: "".to_string(),
                loc: 0,
            },
            next_token: Node {
                token: Token::Eof,
                ch: "".to_string(),
                loc: 0,
            },
            errors: vec![],
        };

        parser.bump();
        parser.bump();

        parser
    }

    fn token_to_precedence(tok: &Node) -> Precedence {
        match tok.token {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LessThan | Token::LessThanEqual => Precedence::LessGreater,
            Token::GreaterThan | Token::GreaterThanEqual => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::Caret | Token::Percent => Precedence::Sum,
            Token::Lbracket => Precedence::Index,
            Token::Lparen => Precedence::Call,
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

    fn current_token_is(&mut self, tok: Token) -> bool {
        self.current_token.token == tok
    }

    fn next_token_is(&mut self, tok: &Token) -> bool {
        self.next_token.token == *tok
    }

    fn expect_next_token(&mut self, tok: Token) -> bool {
        if self.next_token_is(&tok) || self.next_token_is(&Token::Blank) {
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

    fn error_next_token(&mut self, tok: Token) {
        self.errors.push(ParseError::new(
            ParseErrorKind::UnexpectedToken,
            format!(
                "Unexpected `{:?}`, expected `{:?}` instead",
                self.next_token.token, tok
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

        while !self.current_token_is(Token::Eof) {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.bump();
        }

        program
    }

    fn parse_block_stmt(&mut self) -> BlockStmt {
        self.bump();

        let mut block = vec![];

        while !self.current_token_is(Token::End) && !self.current_token_is(Token::Eof) {
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
            Token::Blank => Some(Stmt::Blank),
            Token::Return => self.parse_return_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.bump();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Stmt::Return(expr))
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                if self.next_token_is(&Token::Semicolon) {
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
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Let => self.parse_let_expr(),
            Token::Number(_) => self.parse_int_expr(),
            Token::String(_) => self.parse_string_expr(),
            Token::Bool(_) => self.parse_bool_expr(),
            Token::Lbracket => self.parse_array_expr(),
            Token::Lbrace => self.parse_hash_expr(),
            Token::Percent | Token::Caret | Token::Bang | Token::Minus | Token::Plus => {
                self.parse_prefix_expr()
            }
            Token::Lparen => self.parse_grouped_expr(),
            Token::If => self.parse_if_expr(),
            Token::While => self.parse_while_expr(),
            Token::Func => self.parse_func_expr(),
            Token::Blank => {
                self.bump();
                return self.parse_expr(Precedence::Lowest);
            }
            _ => {
                self.error_no_prefix_parser();
                return None;
            }
        };

        while !self.next_token_is(&Token::Semicolon) && precedence < self.next_token_precedence() {
            match self.next_token.token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Caret
                | Token::Percent
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEqual
                | Token::GreaterThan
                | Token::GreaterThanEqual => {
                    self.bump();
                    left = self.parse_infix_expr(left.unwrap());
                }
                Token::Lbracket => {
                    self.bump();
                    left = self.parse_index_expr(left.unwrap());
                }
                Token::Lparen => {
                    let l = left.clone().unwrap();
                    self.bump();
                    if let Expr::Ident(_) = l {
                        left = self.parse_call_expr(l);
                    } else {
                        self.error_next_token(Token::Lparen);
                    }
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        match self.current_token.token {
            Token::Ident(ref mut ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        if !self.next_token_is(&Token::Assign) {
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
            Token::Assign => self.bump(),
            _ => return self.parse_expr(Precedence::Lowest),
        }

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Expr::Assign(name, Box::new(expr)))
    }

    fn parse_let_expr(&mut self) -> Option<Expr> {
        self.bump();

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        self.bump();

        match self.current_token.token {
            Token::Assign => self.bump(),
            _ => return self.parse_expr(Precedence::Lowest),
        }

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Expr::Let(name, Box::new(expr)))
    }

    fn parse_func_expr(&mut self) -> Option<Expr> {
        self.bump();

        let name = self.parse_ident();

        if !self.expect_next_token(Token::Lparen) {
            return None;
        }

        let params = match self.parse_func_params() {
            Some(params) => params,
            None => return None,
        };

        if !self.expect_next_token(Token::Colon) {
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

        if self.next_token_is(&Token::Rparen) {
            self.bump();
            return Some(params);
        }

        self.bump();

        match self.parse_ident() {
            Some(ident) => params.push(ident),
            None => return None,
        };

        while self.next_token_is(&Token::Comma) {
            self.bump();
            self.bump();

            match self.parse_ident() {
                Some(ident) => params.push(ident),
                None => return None,
            };
        }

        if !self.expect_next_token(Token::Rparen) {
            return None;
        }

        Some(params)
    }

    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.current_token.token {
            Token::Number(ref int) => match int {
                Number::Int(i) => Some(Expr::Literal(Literal::Number(Number::Int(i.clone())))),
                Number::Float(i) => Some(Expr::Literal(Literal::Number(Number::Float(i.clone())))),
            },
            _ => None,
        }
    }

    fn parse_string_expr(&mut self) -> Option<Expr> {
        match self.current_token.token {
            Token::String(ref mut s) => Some(Expr::Literal(Literal::String(s.clone()))),
            _ => None,
        }
    }

    fn parse_bool_expr(&mut self) -> Option<Expr> {
        match self.current_token.token {
            Token::Bool(value) => Some(Expr::Literal(Literal::Bool(value == true))),
            _ => None,
        }
    }

    fn parse_array_expr(&mut self) -> Option<Expr> {
        match self.parse_expr_list(Token::Rbracket) {
            Some(list) => Some(Expr::Literal(Literal::Array(list))),
            None => None,
        }
    }

    fn parse_hash_expr(&mut self) -> Option<Expr> {
        let mut pairs = Vec::new();

        while !self.next_token_is(&Token::Rbrace) {
            self.bump();

            let key = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            if !self.expect_next_token(Token::Colon) {
                return None;
            }

            self.bump();

            let value = match self.parse_expr(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            pairs.push((key, value));

            if !self.next_token_is(&Token::Rbrace) && !self.expect_next_token(Token::Comma) {
                return None;
            }
        }

        if !self.expect_next_token(Token::Rbrace) {
            return None;
        }

        Some(Expr::Literal(Literal::Hash(pairs)))
    }

    fn parse_expr_list(&mut self, end: Token) -> Option<Vec<Expr>> {
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

        while self.next_token_is(&Token::Comma) {
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
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
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
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Divide,
            Token::Asterisk => Infix::Multiply,
            Token::Equal => Infix::Equal,
            Token::NotEqual => Infix::NotEqual,
            Token::LessThan => Infix::LessThan,
            Token::LessThanEqual => Infix::LessThanEqual,
            Token::GreaterThan => Infix::GreaterThan,
            Token::GreaterThanEqual => Infix::GreaterThanEqual,
            Token::Caret => Infix::Power,
            Token::Percent => Infix::Modulo,
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

        if !self.expect_next_token(Token::Rbracket) {
            return None;
        }

        Some(Expr::Index(Box::new(left), Box::new(index)))
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.bump();

        let expr = self.parse_expr(Precedence::Lowest);

        if !self.expect_next_token(Token::Rparen) {
            None
        } else {
            expr
        }
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.expect_next_token(Token::Lparen) {
            return None;
        }

        self.bump();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::Rparen) || !self.expect_next_token(Token::Colon) {
            return None;
        }
        self.bump();

        let mut consequence = vec![];

        while !self.current_token_is(Token::End)
            && !self.current_token_is(Token::Else)
            && !self.current_token_is(Token::Eof)
        {
            match self.parse_stmt() {
                Some(stmt) => consequence.push(stmt),
                None => {}
            }
            self.bump();
        }

        let mut alternative = None;
        if self.current_token_is(Token::Else) {
            // self.bump();

            if !self.expect_next_token(Token::Colon) {
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
        if !self.expect_next_token(Token::Lparen) {
            return None;
        }

        self.bump();

        let cond = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_next_token(Token::Rparen) || !self.expect_next_token(Token::Colon) {
            return None;
        }

        let consequence = self.parse_block_stmt();

        Some(Expr::While {
            cond: Box::new(cond),
            consequence,
        })
    }

    fn parse_call_expr(&mut self, func: Expr) -> Option<Expr> {
        let args = match self.parse_expr_list(Token::Rparen) {
            Some(args) => args,
            None => return None,
        };

        Some(Expr::Call {
            func: Box::new(func),
            args,
        })
    }
}

#[cfg(test)]
mod parser_tests {
    use super::ParseError;
    use super::Parser;
    use crate::{ast::*, tokenizer::Lexer};

    fn parse(i: &str) -> Result<Program, Vec<ParseError>> {
        let mut p = Parser::new(Lexer::new(i));
        let ast = p.parse();
        let errors = p.get_errors();
        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(ast)
    }

    macro_rules! stmt {
        ($e: expr) => {
            Stmt::Expr($e)
        };
    }

    macro_rules! literal {
        ($e: expr) => {
            Expr::Literal($e)
        };
    }

    macro_rules! int {
        ($e: expr) => {
            Literal::Number(Number::Int($e))
        };
    }

    macro_rules! double {
        ($e: expr) => {
            Literal::Number(Number::Float($e))
        };
    }

    macro_rules! minus {
        ($e: expr) => {
            Stmt::Expr(Expr::Prefix(Prefix::Minus, Box::new($e)))
        };
    }

    macro_rules! string {
        ($e: expr) => {
            Literal::String($e.to_string())
        };
    }
    #[test]
    fn test_numbers() {
        assert_eq!(parse("1").unwrap(), vec![stmt!(literal!(int!(1)))]);
        assert_eq!(parse("-20").unwrap(), vec![minus!(literal!(int!(20)))]);
        assert_eq!(parse("1.2").unwrap(), vec![stmt!(literal!(double!(1.2)))]);
        assert_eq!(
            parse("-9.969").unwrap(),
            vec![minus!(literal!(double!(9.969)))]
        );
        assert_eq!(
            parse("0.00000000000000001").unwrap(),
            vec![stmt!(literal!(double!(0.00000000000000001)))]
        );
    }

    #[test]
    fn test_strings() {
        assert_eq!(parse("\"\"").unwrap(), vec![stmt!(literal!(string!("")))]);
        assert_eq!(
            parse("\"Hello, World!\"").unwrap(),
            vec![stmt!(literal!(string!("Hello, World!")))]
        );
        assert_eq!(
            parse("\"नमस्ते\"").unwrap(),
            vec![stmt!(literal!(string!("नमस्ते")))]
        );
        assert_eq!(
            parse("\"こんにちは\"").unwrap(),
            vec![stmt!(literal!(string!("こんにちは")))]
        );
        assert_eq!(
            parse("\"こんにちは\"").unwrap(),
            vec![stmt!(literal!(string!("こんにちは")))]
        );
        assert_eq!(
            parse("\"Z̤͔ͧ̑̓ä͖̭̈̇lͮ̒ͫǧ̗͚̚o̙̔ͮ̇͐̇\"").unwrap(),
            vec![stmt!(literal!(string!("Z̤͔ͧ̑̓ä͖̭̈̇lͮ̒ͫǧ̗͚̚o̙̔ͮ̇͐̇")))]
        );
        assert_eq!(
            parse("\"👱👱🏻👱🏼👱🏽👱🏾👱🏿\"").unwrap(),
            vec![stmt!(literal!(string!("👱👱🏻👱🏼👱🏽👱🏾👱🏿")))]
        );
    }
}
