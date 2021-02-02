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
            Token::Lbracket | Token::Dot => Precedence::Index,
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
            Token::Slash => self.parse_regexp_expr(),
            Token::Percent | Token::Caret | Token::Minus | Token::Plus | Token::Bang => {
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
                Token::Dot => {
                    self.bump();
                    left = self.parse_accessor_expr(left.unwrap());
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

        Some(Expr::Assign(Box::new(Expr::Ident(name)), Box::new(expr)))
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

    fn parse_regexp_pattern(&mut self) -> Expr {
        let mut block = String::new();
        while !self.current_token_is(Token::Slash) && !self.current_token_is(Token::Eof) {
            block.push_str(&self.current_token.ch);
            self.bump();
        }

        Expr::Ident(Ident(block))
    }

    fn parse_regexp_expr(&mut self) -> Option<Expr> {
        self.bump();

        let pattern = self.parse_regexp_pattern();
        self.bump();

        let flags = self.parse_ident();

        if self.next_token_is(&Token::Semicolon) {
            self.bump();
        }

        Some(Expr::Regexp {
            pattern: Box::new(pattern),
            flags,
        })
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

    fn parse_accessor_expr(&mut self, left: Expr) -> Option<Expr> {
        self.bump();

        let accessor = self.parse_ident()?;

        Some(Expr::Accessor(Box::new(left), vec![accessor]))
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

        let index = Expr::Index(Box::new(left), Box::new(index));
        if self.next_token_is(&Token::Assign) {
            self.bump();
            self.bump();
            let val = self.parse_expr(Precedence::Lowest)?;
            return Some(Expr::Assign(Box::new(index), Box::new(val)));
        }

        Some(index)
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

    macro_rules! ident {
        ($e: expr) => {
            Box::new(Expr::Ident(Ident($e.to_string())))
        };
    }

    macro_rules! call {
        ($fn: expr, $args: expr) => {
            Expr::Call {
                func: $fn,
                args: $args,
            }
        };
    }

    macro_rules! if_expr {
        ($if: expr, $stmt: expr, $else: expr) => {
            Expr::If {
                cond: Box::new($if),
                consequence: $stmt,
                alternative: Some($else),
            }
        };

        ($if: expr, $stmt: expr) => {
            Expr::If {
                cond: Box::new($if),
                consequence: $stmt,
                alternative: None,
            }
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

    macro_rules! lessthan {
        ($e: expr, $f: expr) => {
            Expr::Infix(Infix::LessThan, Box::new($e), Box::new($f))
        };
    }

    macro_rules! add {
        ($e: expr, $f: expr) => {
            Expr::Infix(Infix::Plus, Box::new($e), Box::new($f))
        };
    }

    macro_rules! subtract {
        ($e: expr, $f: expr) => {
            Expr::Infix(Infix::Minus, Box::new($e), Box::new($f))
        };
    }

    macro_rules! equal {
        ($e: expr, $f: expr) => {
            Expr::Infix(Infix::Equal, Box::new($e), Box::new($f))
        };
    }

    macro_rules! string {
        ($e: expr) => {
            Literal::String(format!("\"{}\"", $e.to_string()))
        };
    }

    macro_rules! boolean {
        ($e: expr) => {
            Literal::Bool($e)
        };
    }

    macro_rules! index {
        ($e: expr, $f: expr) => {
            Expr::Index($e, $f)
        };
    }

    macro_rules! array {
        ($e: expr) => {
            Expr::Literal(Literal::Array($e))
        };
    }

    macro_rules! regexp {
        ($p: expr, $f: expr) => {
            Expr::Regexp {
                pattern: $p,
                flags: $f,
            }
        };
    }

    macro_rules! object {
        ($e: expr) => {
            Expr::Literal(Literal::Hash($e))
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

    #[test]
    fn test_bool() {
        assert_eq!(
            parse("true").unwrap(),
            vec![stmt!(literal!(boolean!(true)))]
        );
        assert_eq!(
            parse("false").unwrap(),
            vec![stmt!(literal!(boolean!(false)))]
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            parse("[451, 2.9, false, \"hi!\"]").unwrap(),
            vec![stmt!(array!(vec![
                literal!(int!(451)),
                literal!(double!(2.9)),
                literal!(boolean!(false)),
                literal!(string!("hi!"))
            ]))]
        );
    }

    #[test]
    fn test_hash() {
        assert_eq!(
            parse(r#"{"k": "v"}"#).unwrap(),
            vec![stmt!(object!(vec![(
                literal!(string!("k")),
                literal!(string!("v"))
            )]))]
        );
        assert_eq!(
            parse(r#"{"key": "value", "arr": [1, 2, 3]}"#).unwrap(),
            vec![stmt!(object!(vec![
                (literal!(string!("key")), literal!(string!("value"))),
                (
                    literal!(string!("arr")),
                    array!(vec![
                        literal!(int!(1)),
                        literal!(int!(2)),
                        literal!(int!(3))
                    ])
                )
            ]))]
        );
        assert_eq!(
            parse(r#"{0: [1, 2, 3]}"#).unwrap(),
            vec![stmt!(object!(vec![(
                literal!(int!(0)),
                array!(vec![
                    literal!(int!(1)),
                    literal!(int!(2)),
                    literal!(int!(3))
                ])
            )]))]
        );
        assert_eq!(
            parse(r#"{"vec": [1, 2, [3]]}"#).unwrap(),
            vec![stmt!(object!(vec![(
                literal!(string!("vec")),
                array!(vec![
                    literal!(int!(1)),
                    literal!(int!(2)),
                    array!(vec![literal!(int!(3))])
                ])
            )]))]
        );
    }

    #[test]
    fn test_if_stmt_cond() {
        assert_eq!(
            parse("if(true): end").unwrap(),
            vec![stmt!(if_expr!(literal!(boolean!(true)), vec![]))]
        );

        assert_eq!(
            parse("if(1 < 2): end").unwrap(),
            vec![stmt!(if_expr!(
                lessthan!(literal!(int!(1)), literal!(int!(2))),
                vec![]
            ))]
        );
    }

    #[test]
    fn test_if_stmt_consq() {
        assert_eq!(
            parse("if(1 < 1.9999): print(\"Fair precision\") end").unwrap(),
            vec![stmt!(if_expr!(
                lessthan!(literal!(int!(1)), literal!(double!(1.9999))),
                vec![stmt!(call!(
                    ident!("print"),
                    vec![literal!(string!("Fair precision"))]
                ))]
            ))]
        );

        assert_eq!(
            parse("if(type(input) == \"string\"): print(\"Type of input is string\") else: print(\"Type of input is not a string\") end").unwrap(),
            vec![
                stmt!(
                    if_expr!(
                        equal!(
                            call!(
                                ident!("type"),
                                vec![*ident!("input")]
                            ),
                            literal!(string!("string"))
                        ),
                        vec![
                            stmt!(call!(
                                ident!("print"),
                                vec![literal!(string!("Type of input is string"))]
                                )
                            )
                        ],
                        vec![
                            stmt!(
                                call!(
                                    ident!("print"),
                                    vec![
                                        literal!(string!("Type of input is not a string"))
                                    ]
                                )
                            )
                        ]
                    )
                )
            ]
        );
    }

    #[test]
    fn test_infix() {
        assert_eq!(
            parse("1 + 2").unwrap(),
            vec![stmt!(add!(literal!(int!(1)), literal!(int!(2))))]
        );

        assert_eq!(
            parse("1 + 190.7 - 1").unwrap(),
            vec![stmt!(subtract!(
                add!(literal!(int!(1)), literal!(double!(190.7))),
                literal!(int!(1))
            ))]
        );
    }

    #[test]
    fn test_index() {
        assert_eq!(
            parse("o[1]").unwrap(),
            vec![stmt!(index!(ident!("o"), Box::new(literal!(int!(1)))))]
        );

        assert_eq!(
            parse("[1, 2][0]").unwrap(),
            vec![stmt!(index!(
                Box::new(array!(vec![literal!(int!(1)), literal!(int!(2))])),
                Box::new(literal!(int!(0)))
            ))]
        );
    }

    #[test]
    fn test_call() {
        assert_eq!(
            parse("print(\"Hello, World!\")").unwrap(),
            vec![stmt!(call!(
                ident!("print"),
                vec![literal!(string!("Hello, World!"))]
            ))]
        );

        assert_eq!(
            parse("int(0.9910)").unwrap(),
            vec![stmt!(call!(ident!("int"), vec![literal!(double!(0.9910))]))]
        );
    }

    #[test]
    fn test_regexp() {
        assert_eq!(
            parse("/x/").unwrap(),
            vec![stmt!(regexp!(ident!("x"), None))]
        );
        assert_eq!(
            parse(r#"/^(([^<>()[\]\.,;:\s@\"]+(\.[^<>()[\]\.,;:\s@\"]+)*)|(\".+\"))@(([^<>()[\]\.,;:\s@\"]+\.)+[^<>()[\]\.,;:\s@\"]{2,})$/g"#).unwrap(),
            vec![stmt!(regexp!(ident!(r#"^(([^<>()[\]\.,;:\s@\"]+(\.[^<>()[\]\.,;:\s@\"]+)*)|(\".+\"))@(([^<>()[\]\.,;:\s@\"]+\.)+[^<>()[\]\.,;:\s@\"]{2,})$"#), Some(Ident("g".to_string()))))]
        );
    }
}
