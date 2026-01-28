use crate::lexer::{Keyword, Operator};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Number(f64),
    String(String),
    EndOfFile,
    Unknown(char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}
