use crate::lexer::{Keyword, Operator, Token, TokenKind};

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            if self.ch == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.read_char();
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        self.input[self.read_position]
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        let token_kind = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::Operator(Operator::Equal)
                } else if self.peek_char() == '>' {
                    self.read_char();
                    TokenKind::Operator(Operator::ArrowFunction)
                } else {
                    TokenKind::Operator(Operator::Assignment)
                }
            }
            ';' => TokenKind::Operator(Operator::Semicolon),
            '(' => TokenKind::Operator(Operator::LParen),
            ')' => TokenKind::Operator(Operator::RParen),
            ',' => TokenKind::Operator(Operator::Comma),
            '+' => TokenKind::Unknown(self.ch), // TODO: Add Plus
            '-' => TokenKind::Unknown(self.ch), // TODO: Add Minus
            '*' => TokenKind::Unknown(self.ch), // TODO: Add Asterisk
            '/' => TokenKind::Unknown(self.ch), // TODO: Add Slash
            '{' => TokenKind::Operator(Operator::LBrace),
            '}' => TokenKind::Operator(Operator::RBrace),
            '[' => TokenKind::Operator(Operator::LBracket),
            ']' => TokenKind::Operator(Operator::RBracket),
            ':' => TokenKind::Operator(Operator::Colon),
            '.' => TokenKind::Operator(Operator::Dot),
            '?' => TokenKind::Operator(Operator::QuestionMark),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::Operator(Operator::NotEqual)
                } else {
                    TokenKind::Unknown(self.ch)
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::Operator(Operator::GreaterEqual)
                } else {
                    TokenKind::Operator(Operator::GreaterThan)
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::Operator(Operator::LessEqual)
                } else {
                    TokenKind::Operator(Operator::LessThan)
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    TokenKind::Operator(Operator::And)
                } else {
                    TokenKind::Operator(Operator::BitAnd)
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    TokenKind::Operator(Operator::Or)
                } else {
                    TokenKind::Unknown(self.ch) // TODO: BitOr
                }
            }
            '"' | '\'' => {
                let quote = self.ch;
                let s = self.read_string(quote);
                return Token {
                    token: TokenKind::String(s),
                    lexeme: "string".to_string(),
                    line: start_line,
                    column: start_column,
                };
            }
            '\0' => TokenKind::EndOfFile,
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = match Keyword::from_str(&literal) {
                        Some(kw) => TokenKind::Keyword(kw),
                        None => TokenKind::Identifier(literal.clone()),
                    };
                    return Token {
                        token: kind,
                        lexeme: literal,
                        line: start_line,
                        column: start_column,
                    };
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number();
                    let value = literal.parse::<f64>().unwrap_or(0.0);
                    return Token {
                        token: TokenKind::Number(value),
                        lexeme: literal,
                        line: start_line,
                        column: start_column,
                    };
                } else {
                    TokenKind::Unknown(self.ch)
                }
            }
        };

        self.read_char();

        let lexeme = match &token_kind {
             TokenKind::Operator(op) => op.to_lexeme().to_string(),
             TokenKind::Unknown(c) => c.to_string(),
             TokenKind::EndOfFile => "\0".to_string(),
             _ => "".to_string(),
        };

        Token {
            token: token_kind,
            lexeme,
            line: start_line,
            column: start_column,
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) || self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        if self.ch == '.' {
            self.read_char();
            while self.ch.is_digit(10) {
                self.read_char();
            }
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_string(&mut self, closing: char) -> String {
        let mut value = "".to_string();
        self.read_char();
        loop {
            if self.ch == closing || self.ch == '\0' {
                break;
            }
            value.push(self.ch);
            self.read_char();
        }
        self.read_char();
        value
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
