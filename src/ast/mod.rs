// AST: Abstract Syntax Tree
// Define a estrutura de dados que representa o código.
// Ex: Structs para Expressões, Statements, Declarações.

pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    BinaryOp(Box<Expr>, String, Box<Expr>), // ex: 1 + 2
}

pub enum Stmt {
    LetDecl { name: String, value: Expr },
    // IfStatement, WhileLoop...
}
