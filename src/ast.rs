use crate::token::Token;


pub enum Expr<'t> {
    Binary(Binary<'t>),
    Unary(Unary<'t>),
    Literal(Literal<'t>),
    Grouping(Grouping<'t>),
}

pub enum BinaryOp {
    Add,
    Minus,
    Times
}

pub enum UnaryOp {
    Minus
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

pub struct Unary<'t> {
    pub token: &'t Token,
    pub operation: UnaryOp,
    pub right: Box<Expr<'t>>
}

pub struct Literal<'t> {
    pub token: &'t Token,
    pub literal_type: LiteralType,
}

pub struct Grouping<'t> {
    pub expr: Box<Expr<'t>>
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
