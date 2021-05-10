use crate::{token::Token, typechecker::TypeKind};


#[derive(Debug)]
pub enum Expr<'t> {
    Binary(Binary<'t>),
    Unary(Unary<'t>),
    Literal(Literal<'t>),
    Grouping(Grouping<'t>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Minus,
    Times,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus
}

#[derive(Debug)]
pub enum LiteralType {
    Int
}

#[derive(Debug)]
pub struct Binary<'t> {
    pub token: &'t Token,
    pub operation: BinaryOp,
    pub left: Box<Expr<'t>>,
    pub right: Box<Expr<'t>>,
    pub type_kind: Option<TypeKind>,
}

#[derive(Debug)]
pub struct Unary<'t> {
    pub token: &'t Token,
    pub operation: UnaryOp,
    pub right: Box<Expr<'t>>,
    pub type_kind: Option<TypeKind>,
}

#[derive(Debug)]
pub struct Literal<'t> {
    pub token: &'t Token,
    pub literal_type: LiteralType,
    pub type_kind: Option<TypeKind>,
}

#[derive(Debug)]
pub struct Grouping<'t> {
    pub expr: Box<Expr<'t>>,
    pub type_kind: Option<TypeKind>,
}

// print string representation of AST for debugging
#[allow(dead_code)]
pub fn tree_repr(root: &Expr, indent: usize) -> String {
    let repr;
    match root {
        Expr::Binary(n) => {
            let op = match n.operation {
                BinaryOp::Add => "+",
                BinaryOp::Minus => "-",
                BinaryOp::Times => "*",
            };
            repr = format!("{:>width$}\n{left}\n{right}",
                op=op,
                left=tree_repr(n.left.as_ref(), indent + 1),
                right=tree_repr(n.right.as_ref(), indent + 1),
                width = indent
            );
        }
        Expr::Unary(n) => {
            let op = match n.operation {
                UnaryOp::Minus => "-"
            };
            repr = format!("{:>width$}\n{right}",
                op=op,
                right=tree_repr(n.right.as_ref(), indent + 1),
                width = indent
            );
        }
        Expr::Literal(n) => {
            repr = format!("{:>width$}", lit=n.token.lexeme.clone(), width = indent);
        }
        Expr::Grouping(n) => {
            repr = tree_repr(n.expr.as_ref(), indent);
        }
    }
    repr
}
