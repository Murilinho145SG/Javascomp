use crate::lexer::Operator;

pub enum Expression {
    Number(f64),
    String(String),

    Identifier(String),

    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    
    Let(Box<Expression>, Option<Box<Expression>>),
    Const(Box<Expression>, Option<Box<Expression>>),
    Assign(Box<Expression>, Box<Expression>),

    Get(Box<Expression>, Box<Expression>),

    Call(Box<Expression>, Vec<Expression>),

    Function {
        name: Option<String>,
        params: Vec<Expression>,
        body: Vec<Expression>
    }
}

pub type Ast = Vec<Expression>;
