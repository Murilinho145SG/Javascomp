#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Function,
    Let,
    Const,
    If,
    Else,
    While,
    Do,
    For,
    Break,
    Continue,
    Return,

    // TODO!
    Switch,
    Class,
    Import,
    Export,
    Default,
    Async,
}

impl Keyword {
    pub fn to_lexeme(&self) -> &'static str {
        match self {
            Keyword::Function => "function",
            Keyword::Let => "let",
            Keyword::Const => "const",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::Do => "do",
            Keyword::For => "for",
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::Return => "return",
            Keyword::Switch => "switch",
            Keyword::Class => "class",
            Keyword::Import => "import",
            Keyword::Export => "export",
            Keyword::Default => "default",
            Keyword::Async => "async",
        }
    }

    pub fn from_str(value: &str) -> Option<Keyword> {
        match value {
            "function" => Some(Keyword::Function),
            "let" => Some(Keyword::Let),
            "const" => Some(Keyword::Const),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "do" => Some(Keyword::Do),
            "for" => Some(Keyword::For),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "return" => Some(Keyword::Return),
            "switch" => Some(Keyword::Switch),
            "class" => Some(Keyword::Class),
            "import" => Some(Keyword::Import),
            "export" => Some(Keyword::Export),
            "default" => Some(Keyword::Default),
            "async" => Some(Keyword::Async),
            _ => None,
        }
    }
}
