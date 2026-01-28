use crate::lexer::Operator;

use crate::types::Type;

pub enum Expression {
    Number(f64),
    String(String),

    Identifier(String),

    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    
    Let(Box<Expression>, Type, Option<Box<Expression>>),
    Const(Box<Expression>, Type, Option<Box<Expression>>),
    Assign(Box<Expression>, Box<Expression>),

    Get(Box<Expression>, Box<Expression>),

    Call(Box<Expression>, Vec<Expression>),

    Function {
        name: Option<String>,
        return_type: Type,
        params: Vec<(String, Type)>,
        body: Vec<Expression>
    }
}

pub type Ast = Vec<Expression>;
