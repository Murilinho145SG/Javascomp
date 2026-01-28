pub mod lexer;
pub mod parser;
pub mod ast;
pub mod codegen;
pub mod diagnostics;

/// Entry point for the compiler logic (API).
/// Here we connect the lexer, parser, and code generator.
pub fn compile(_source: &str) {
    println!("Compiling... (Not yet implemented)");
}
