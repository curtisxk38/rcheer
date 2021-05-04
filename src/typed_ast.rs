
use crate::{ast::{Binary, Grouping, Literal, Unary}, typechecker::TypeKind};

pub struct TypedExpr<'t> {
    pub type_kind: TypeKind,
    pub expr: TExpr<'t>
}

pub enum TExpr<'t> {
    Binary(TBinary<'t>),
    Unary(TUnary<'t>),
    Literal(TLiteral<'t>),
    Grouping(TGrouping<'t>),
}

pub struct TBinary<'t> {
    pub type_kind: TypeKind,
    pub binary: &'t Binary<'t>
}

pub struct TUnary<'t> {
    pub type_kind: TypeKind,
    pub unary: &'t Unary<'t>,
}

pub struct TLiteral<'t> {
    pub type_kind: TypeKind,
    pub literal: &'t Literal<'t>,
}

pub struct TGrouping<'t> {
    pub type_kind: TypeKind,
    pub grouping: &'t Grouping<'t>,
}
