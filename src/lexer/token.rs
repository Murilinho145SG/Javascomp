use crate::lexer::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
	Identifier(String),
	Keyword(Keyword),
	Number(f64),
	String(String)
}

pub struct Token {
	pub token: TokenKind,
}