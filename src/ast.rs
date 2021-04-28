use crate::token::Token;


pub enum Expr<'t> {
    Binary(Binary<'t>),
    Literal(Literal<'t>),
}

pub enum BinaryOp {
    Add
}

pub enum LiteralType {
    Int
}

pub struct Binary<'t> {
    pub token: &'t Token,
    pub operation: BinaryOp,
    pub left: Box<Expr<'t>>,
    pub right: Box<Expr<'t>>,
}

pub struct Literal<'t> {
    pub token: &'t Token,
    pub literal_type: LiteralType,
}