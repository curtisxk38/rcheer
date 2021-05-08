use crate::{ast::{Binary, BinaryOp, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp}};

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

#[derive(Debug, Copy, Clone)]
pub enum TypeKind {
    Int,
    Bool,
    Error
}

impl TypeChecker {
    pub fn typecheck(&mut self, expr: &mut Expr) -> TypeResult {
        self.type_expr(expr);
        TypeResult::Success
    }
    fn type_expr<'t>(&mut self, expr: &'t mut Expr) -> TypeKind {
        match expr {
            Expr::Binary(binary) => {
                self.type_binary(binary)
            }
            Expr::Unary(unary) => {
                self.type_unary(unary)
            }
            Expr::Literal(literal) => {
                self.type_literal(literal)
            }
            Expr::Grouping(grouping) => {
                self.type_grouping(grouping)
            }
        }
    }

    fn type_binary<'t>(&mut self, binary: &'t mut Binary) -> TypeKind {
        let left_kind = self.type_expr(binary.left.as_mut());
        let right_kind = self.type_expr(binary.right.as_mut());

        let type_kind = match (&binary.operation, left_kind, right_kind) {
            (BinaryOp::Add, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::Minus, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::Times, TypeKind::Int, TypeKind::Int) => {
                TypeKind::Int
            }
            _ => {
                match (left_kind, right_kind) {
                    (TypeKind::Error, _) => {},
                    (_, TypeKind::Error) => {},
                    _ => {
                        // report error if this is new error and not propogated from child type
                        self.errors.push(TypeError {message: 
                            format!("Type error for {:?}: illegal operation {:?} on {:?} and {:?}",
                            binary.token, binary.operation, left_kind, right_kind
                        )});
                    }
                }
                TypeKind::Error
            }
        };
        binary.type_kind = Some(type_kind);
        type_kind

    }

    fn type_unary<'t>(&mut self, unary: &'t mut Unary) -> TypeKind {
        let right_kind = self.type_expr(unary.right.as_mut());
        let type_kind = match (&unary.operation, right_kind) {
            (UnaryOp::Minus, TypeKind::Int) => {
                TypeKind::Int
            }
            _ => {
                match right_kind {
                    TypeKind::Error => {},
                    _ => {
                        // report error if this is new error and not propogated from child type
                        self.errors.push(TypeError {message: 
                            format!("Type error for {:?}: illegal operation {:?} on {:?}",
                            unary.token, unary.operation, right_kind 
                        )});
                    }
                }
                TypeKind::Error
            }
        };
        unary.type_kind = Some(type_kind);
        type_kind
    }

    fn type_literal<'t>(&mut self, literal: &'t mut Literal) -> TypeKind {
        let type_kind = match literal.literal_type {
            LiteralType::Int => TypeKind::Int,
        };
        literal.type_kind = Some(type_kind);
        type_kind
    }

    fn type_grouping<'t>(&mut self, grouping: &'t mut Grouping) -> TypeKind {
        let type_kind = self.type_expr(grouping.expr.as_mut());
        grouping.type_kind = Some(type_kind);
        type_kind
    }
}