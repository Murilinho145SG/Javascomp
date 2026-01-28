// Diagnostics: Sistema de Erros e Avisos
// Foco: Mensagens amigáveis e explicativas.

pub struct Diagnostic {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

impl Diagnostic {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            line: 0,
            column: 0,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }
}
