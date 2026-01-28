pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    BinaryOp(Box<Expr>, String, Box<Expr>), // ex: 1 + 2
}

pub enum Stmt {
    LetDecl { name: String, value: Expr },
}
