use crate::tokens::{Node, Number, Token};

/// Represents a Lexer instance.
pub struct Lexer<'a> {
    /// Source is requried to be stored for later diagnosis of errors and reporting.
    pub input: &'a str,
    /// Position of the current state of the lexer.
    ///
    /// Note: this does not represent the line or column. It ignored line ends and considers it a part of source.
    ///       useful for codespan diagnostics.
    pub pos: usize,
    /// Position of the character in the future state of lexer.
    /// used for reviewing expectations and conditional analysis before actually trying to parse.
    ///
    /// Note: this does not represent the line or column. It ignored line ends and considers it a part of source.
    ///       useful for codespan diagnostics.
    pub next_pos: usize,
    /// The utf-8 character currently being parsed in the present state of Lexer.
    /// corresponds to the utf8 value for the current token being analysed.
    pub ch: u8,
}

enum IsNumber {
    Float,
    Int,
}

macro_rules! ctok {
    ($self: expr, $l:expr) => {
        Node {
            token: $l,
            ch: String::from_utf8_lossy(&[$self.ch]).to_string(),
            loc: $self.pos,
        }
    };
}

/// The lexer implementation
impl<'a> Lexer<'a> {
    /// Create a new instance of lexer for a given source input.
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    fn nextch(&mut self) -> u8 {
        if self.next_pos >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.next_pos];
        }
    }

    fn nextch_is(&mut self, ch: u8) -> bool {
        self.nextch() == ch
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn skip_comments(&mut self) {
        loop {
            match self.ch {
                b'\n' | b'\r' | 0 => {
                    break;
                }
                _ => self.read_char(),
            }
        }
    }

    pub fn next_token(&mut self) -> Node {
        self.skip_whitespace();
        let toki = match self.ch {
            b'=' => {
                let mut tok = ctok!(self, Token::Assign);
                if self.nextch_is(b'=') {
                    tok.token = Token::Equal;
                    self.read_char();
                }
                tok
            }
            b'^' => ctok!(self, Token::Caret),
            b'%' => ctok!(self, Token::Percent),
            b'+' => ctok!(self, Token::Plus),
            b'-' => ctok!(self, Token::Minus),
            b'!' => {
                let mut tok = ctok!(self, Token::Bang);
                if self.nextch_is(b'=') {
                    tok.token = Token::NotEqual;
                    self.read_char();
                }
                tok
            }
            b'/' => ctok!(self, Token::Slash),
            b'*' => ctok!(self, Token::Asterisk),
            b'<' => {
                let mut tok = ctok!(self, Token::LessThan);
                if self.nextch_is(b'=') {
                    tok.token = Token::LessThanEqual;
                    self.read_char();
                }
                tok
            }
            b'>' => {
                let mut tok = ctok!(self, Token::GreaterThan);
                if self.nextch_is(b'=') {
                    tok.token = Token::GreaterThanEqual;
                    self.read_char();
                }
                tok
            }
            b'(' => ctok!(self, Token::Lparen),
            b')' => ctok!(self, Token::Rparen),
            b'{' => ctok!(self, Token::Lbrace),
            b'}' => ctok!(self, Token::Rbrace),
            b'[' => ctok!(self, Token::Lbracket),
            b']' => ctok!(self, Token::Rbracket),
            b',' => ctok!(self, Token::Comma),
            b';' => ctok!(self, Token::Semicolon),
            b':' => ctok!(self, Token::Colon),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.consume_identifier();
            }
            b'0'..=b'9' => {
                return self.consume_number();
            }
            b'"' => {
                return self.consume_string();
            }
            b'\n' => {
                self.read_char();
                return self.next_token();
            }
            b'#' => {
                self.skip_comments();
                return self.next_token();
            }
            0 => ctok!(self, Token::Eof),
            _ => ctok!(self, Token::Illegal),
        };

        self.read_char();

        return toki;
    }

    fn consume_identifier(&mut self) -> Node {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
        let literal = &self.input[start_pos..self.pos];
        match literal {
            "if" => Node {
                token: Token::If,
                ch: literal.to_string(),
                loc: start_pos,
            },
            "let" => Node {
                token: Token::Let,
                ch: literal.to_string(),
                loc: start_pos,
            },
            "else" => Node {
                token: Token::Else,
                ch: literal.to_string(),
                loc: start_pos,
            },
            "while" => Node {
                token: Token::While,
                ch: literal.to_string(),
                loc: start_pos,
            },
            "fn" => Node {
                token: Token::Func,
                ch: literal.to_string(),
                loc: start_pos,
            },
            "true" => Node {
                token: Token::Bool(true),
                ch: literal.to_string(),
                loc: start_pos,
            },
            "false" => Node {
                token: Token::Bool(false),
                ch: literal.to_string(),
                loc: start_pos,
            },
            "end" => Node {
                token: Token::End,
                ch: literal.to_string(),
                loc: start_pos,
            },
            "return" => Node {
                token: Token::Return,
                ch: literal.to_string(),
                loc: start_pos,
            },
            _ => Node {
                token: Token::Ident(String::from(literal)),
                ch: literal.to_string(),
                loc: start_pos,
            },
        }
    }

    fn consume_number(&mut self) -> Node {
        let start_pos = self.pos;
        let mut num_type: IsNumber = IsNumber::Int;
        loop {
            match self.ch {
                b'0'..=b'9' => {
                    self.read_char();
                }
                b'.' => {
                    self.read_char();
                    num_type = IsNumber::Float;
                }
                _ => {
                    break;
                }
            }
        }

        let literal = &self.input[start_pos..self.pos];

        let token: Token = match num_type {
            IsNumber::Int => Token::Number(Number::Int(literal.parse::<i32>().unwrap())),
            IsNumber::Float => Token::Number(Number::Float(literal.parse::<f64>().unwrap())),
        };

        Node {
            token,
            ch: literal.to_string(),
            loc: start_pos,
        }
    }

    fn consume_string(&mut self) -> Node {
        self.read_char();

        let start_pos = self.pos;

        loop {
            match self.ch {
                b'"' | 0 => {
                    let literal = &self.input[start_pos..self.pos];

                    let tok = Node {
                        token: Token::String(literal.to_string()),
                        ch: literal.to_string(),
                        loc: start_pos,
                    };
                    {
                        self.read_char();
                    }
                    return tok;
                }
                _ => {
                    self.read_char();
                }
            }
        }
    }
}
