use std::mem;

use crate::{ast::{Binary, BinaryOp, Expr, Grouping, If, Literal, LiteralType, Unary, UnaryOp}};

pub struct TypeError {
    pub message: String,
}

pub enum TypeResult {
    Success,
    Error
}

pub struct TypeChecker {
    pub errors: Vec<TypeError>
}

#[derive(Debug, Copy, Clone)]
pub enum TypeKind {
    Int,
    Bool,
    Error
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker { errors: Vec::new() }
    }

    pub fn typecheck(&mut self, expr: &mut Expr) -> TypeResult {
        self.type_expr(expr);
        if self.errors.len() > 0 {
            TypeResult::Error
        } else {
            TypeResult::Success
        }
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
            Expr::If(if_expr) => self.type_if(if_expr),
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
            (BinaryOp::Greater, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::GreaterEqual, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::Less, TypeKind::Int, TypeKind::Int) 
            | (BinaryOp::LessEqual, TypeKind::Int, TypeKind::Int) => {
                TypeKind::Bool
            }
            (BinaryOp::BangEqual, _, _) 
            | (BinaryOp::EqualEqual, _, _) => {
                match (left_kind, right_kind) {
                    (TypeKind::Error, _) => {
                        TypeKind::Error
                    }
                    (_, TypeKind::Error) => {
                        TypeKind::Error
                    }
                    _ => {
                        if mem::discriminant(&left_kind) == mem::discriminant(&right_kind) {
                            TypeKind::Bool
                        } else {
                            self.errors.push(TypeError {message: 
                                format!("Type error for {:?}: Expected LHS ({:?}) to match RHS ({:?}) for {:?}",
                                binary.token, left_kind, right_kind, binary.operation
                            )});
                            TypeKind::Error
                        }
                    }
                }    
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

    fn type_if<'t>(&mut self, if_expr: &'t mut If) -> TypeKind {
        let then_type = self.type_expr(if_expr.then_branch.as_mut());

        let type_kind = if let Some(else_branch) = &mut if_expr.else_branch {
            let else_type = self.type_expr(else_branch.as_mut());
            let type_kind = if std::mem::discriminant(&then_type) == std::mem::discriminant(&else_type) {
                then_type
            } else {
                match (then_type, else_type) {
                    (TypeKind::Error, _) | (_, TypeKind::Error) => {}
                    _ => {
                        // report error if this is new error and not propogated from child type
                        self.errors.push(TypeError {message: 
                            format!("Type error for {:?}: then branch returns {:?} and else branch returns {:?}",
                            if_expr.token, then_type, else_type
                        )});
                    }
                }
                TypeKind::Error
                
            };
            type_kind
        } else {
            then_type
        };

        if_expr.type_kind = Some(type_kind);
        type_kind
    }
}