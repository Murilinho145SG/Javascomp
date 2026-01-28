pub mod lexer;
pub mod parser;
pub mod ast;
pub mod codegen;
pub mod diagnostics;

/// Ponto de entrada para a lógica do compilador (API).
/// Aqui podemos conectar o lexer, parser e gerador de código.
pub fn compile(_source: &str) {
    println!("Compilando... (Not yet implemented)");
}
