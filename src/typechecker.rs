use crate::{ast::{Binary, BinaryOp, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp}, typed_ast::{TBinary, TExpr, TGrouping, TLiteral, TUnary, TypedExpr}};

pub struct TypeError {
    pub message: String,
}

pub enum TypeResult {
    Success,
    Error(Vec<TypeChecker>)
}

struct TypeChecker {
    errors: Vec<TypeError>
}

#[derive(Copy, Clone)]
pub enum TypeKind {
    Int,
    Bool,
    Error
}

impl TypeChecker {
    pub fn typecheck(&mut self, expr: &Expr) -> TypeResult {
        self.type_expr(expr);
        TypeResult::Success
    }
    fn type_expr<'t>(&mut self, expr: &'t Expr) -> TypedExpr<'t> {
        match expr {
            Expr::Binary(binary) => {
                let tbinary = self.type_binary(binary);
                TypedExpr {type_kind: tbinary.type_kind, expr: TExpr::Binary(tbinary)}
            }
            Expr::Unary(unary) => {
                let tunary = self.type_unary(unary);
                TypedExpr {type_kind: tunary.type_kind, expr: TExpr::Unary(tunary)}
            }
            Expr::Literal(literal) => {
                let tliteral = self.type_literal(literal);
                TypedExpr {type_kind: tliteral.type_kind, expr: TExpr::Literal(tliteral)}
            }
            Expr::Grouping(grouping) => {
                let tgrouping = self.type_grouping(grouping);
                TypedExpr {type_kind: tgrouping.type_kind, expr: TExpr::Grouping(tgrouping)}
            }
        }
    }

    fn type_binary<'t>(&mut self, binary: &'t Binary) -> TBinary<'t> {
        let left = self.type_expr(binary.left.as_ref());
        let right = self.type_expr(binary.right.as_ref());

        let type_kind = match (&binary.operation, left.type_kind, right.type_kind) {
            (BinaryOp::Add, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::Minus, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::Times, TypeKind::Int, TypeKind::Int) => {
                TypeKind::Int
            }
            _ => {
                TypeKind::Error
            }
        };

        TBinary {type_kind, binary}
    }

    fn type_unary<'t>(&mut self, unary: &'t Unary) -> TUnary<'t> {
        let right = self.type_expr(unary.right.as_ref());

        let type_kind = match (&unary.operation, right.type_kind) {
            (UnaryOp::Minus, TypeKind::Int) => {
                TypeKind::Int
            }
            _ => {
                TypeKind::Error
            }
        };

        TUnary {type_kind, unary}
    }

    fn type_literal<'t>(&mut self, literal: &'t Literal) -> TLiteral<'t> {
        let type_kind = match literal.literal_type {
            LiteralType::Int => TypeKind::Int,
        };
        TLiteral {type_kind, literal}
    }

    fn type_grouping<'t>(&mut self, grouping: &'t Grouping) -> TGrouping<'t> {
        let expr = self.type_expr(grouping.expr.as_ref());
        TGrouping {grouping, type_kind: expr.type_kind}
    }
}