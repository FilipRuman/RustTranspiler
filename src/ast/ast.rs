use crate::tokens::{Token, TokenKind};
#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    VariableDeclaration(String, bool, Expression, TokenKind),
}
impl Statement {}
#[derive(Debug, Clone)]

pub enum Expression {
    Number(f32),
    String(String),
    Symbol(String),
    Binary(Box<Expression>, Token, Box<Expression>),
}
impl Expression {}
#[derive(Debug)]

pub enum VariableType {
    I32,
    I64,
    F32,
    F64,
    Str,
    Var,
}
