use std::{collections::binary_heap::PeekMut, iter::Peekable, slice::Iter};

use crate::{ast::{Binary, BinaryOp, Expr, Grouping, Literal, LiteralType, Unary, UnaryOp}, token::{Token, TokenType}};


pub enum ParseResult<'t> {
    AST(Expr<'t>),
    Error
}

pub struct ParseError {
    message: String,
}

// program => expression ;
pub fn parse(tokens: &Vec<Token>) -> ParseResult {
    let mut tokens = tokens.iter().peekable();
    match expression(&mut tokens) {
        Ok(expr) => {
            match tokens.peek() {
                Some(_) => {
                    // finished parsing, but there's still some tokens left
                    ParseResult::Error
                }
                None => {
                    ParseResult::AST(expr)
                }
            }
        }
        Err(_) => {
            ParseResult::Error
        }
    }
}

// expression => factor (("+"|"-") factor)* ;
fn expression<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    let mut expr = factor(tokens)?;
    loop {
        let op_token;
        let operation;
        match tokens.peek() {
            Some(token) => {
                match token.token_type {
                    TokenType::Plus => {
                        op_token = *token;
                        operation = BinaryOp::Add;
                        tokens.next();
                    }
                    TokenType::Minus => {
                        op_token = *token;
                        operation = BinaryOp::Minus;
                        tokens.next();
                    }
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        };
        let right = factor(tokens)?;
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right)})
    }
    Ok(expr)
}

// factor => unary (("*") unary)*
fn factor<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    let mut expr = unary(tokens)?;
    loop {
        let op_token;
        let operation;
        match tokens.peek() {
            Some(token) => {
                match token.token_type {
                    TokenType::Star => {
                        op_token = *token;
                        operation = BinaryOp::Times;
                        tokens.next();
                    }
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        };
        let right = unary(tokens)?;
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right)})
    }
    Ok(expr)
}

// unary -> ( "-" ) unary | primary
fn unary<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    match tokens.peek() {
        Some(token) => {
            let op_token;
            let operation;
            match token.token_type {
                TokenType::Minus => {
                    op_token = *token;
                    operation = UnaryOp::Minus;
                    tokens.next();
                    let right = unary(tokens)?;
                    Ok(Expr::Unary(Unary {token: op_token, operation, right: Box::new(right)}))
                }
                _ => {
                    primary(tokens)
                }
            }
        }
        None => {
            Err(ParseError{message: String::from("Reached EOF while parsing")})
        }
    }
}

// primary => NUMBER | "(" expression ")";
fn primary<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    match tokens.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::IntLiteral => {
                    let token = *token;
                    tokens.next();
                    Ok(Expr::Literal(Literal { token, literal_type: LiteralType::Int}))
                }
                TokenType::LeftParen => {
                    tokens.next();
                    let expr = expression(tokens)?;
                    let node = Expr::Grouping(Grouping {expr: Box::new(expr)});
                    match tokens.peek() {
                        Some(token) => {
                            match token.token_type {
                                TokenType::RightParen => {
                                    tokens.next();
                                    Ok(node)
                                }
                                _ => {
                                    Err(ParseError{message: format!("Expected ), found: {:?}", token)})
                                }
                            }
                        }
                        None => {
                            Err(ParseError{message: format!("Reached EOF while parsing, expected )")})
                        }
                    }
                }
                _ => {
                    Err(ParseError{message: format!("Expected primary expression, found: {:?}", token)})
                }
            }
        }
        None => {
            Err(ParseError{message: String::from("Reached EOF while parsing")})
        }
    }
}