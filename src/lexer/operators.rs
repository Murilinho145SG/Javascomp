#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
    Or,
    And,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    Assignment,
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    QuestionMark,
    Arrow,
}

impl Operator {
    pub fn to_lexeme(&self) -> &'static str {
        match self {
            Operator::Equal        => "==",
            Operator::NotEqual     => "!=",
            Operator::Or           => "||",
            Operator::And          => "&&",
            Operator::GreaterThan  => ">",
            Operator::GreaterEqual => ">=",
            Operator::LessThan     => "<",
            Operator::LessEqual    => "<=",
            Operator::Assignment   => "=",
            Operator::LBrace       => "{",
            Operator::RBrace       => "}",
            Operator::LParen       => "(",
            Operator::RParen       => ")",
            Operator::LBracket     => "[",
            Operator::RBracket     => "]",
            Operator::Comma        => ",",
            Operator::Dot          => ".",
            Operator::Semicolon    => ";",
            Operator::Colon        => ":",
            Operator::QuestionMark => "?",
            Operator::Arrow        => "=>", 
        }
    }

    pub fn from_str(value: &str) -> Option<Operator> {
        match value {
            "==" => Some(Operator::Equal),
            "!=" => Some(Operator::NotEqual),
            "||" => Some(Operator::Or),
            "&&" => Some(Operator::And),
            ">"  => Some(Operator::GreaterThan),
            ">=" => Some(Operator::GreaterEqual),
            "<"  => Some(Operator::LessThan),
            "<=" => Some(Operator::LessEqual),
            "="  => Some(Operator::Assignment),
            "{"  => Some(Operator::LBrace),
            "}"  => Some(Operator::RBrace),
            "("  => Some(Operator::LParen),
            ")"  => Some(Operator::RParen),
            "["  => Some(Operator::LBracket),
            "]"  => Some(Operator::RBracket),
            ","  => Some(Operator::Comma),
            "."  => Some(Operator::Dot),
            ";"  => Some(Operator::Semicolon),
            ":"  => Some(Operator::Colon),
            "?"  => Some(Operator::QuestionMark),
            "=>" => Some(Operator::Arrow),
            _    => None,
        }
    }
}