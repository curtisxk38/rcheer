use std::{iter::Peekable, slice::Iter};

use crate::{ast::{Binary, BinaryOp, Expr, Literal, LiteralType}, token::{Token, TokenType}};


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
            ParseResult::AST(expr)
        }
        Err(_) => {
            ParseResult::Error
        }
    }
}

// expression => primary (("+"|"-") primary)* ;
pub fn expression<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    let mut expr = primary(tokens)?;
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
                    },
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
        let right = primary(tokens)?;
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right)})
    }
    Ok(expr)
}

// primary => NUMBER ;
pub fn primary<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    match tokens.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::IntLiteral => {
                    let token = *token;
                    tokens.next();
                    Ok(Expr::Literal(Literal { token, literal_type: LiteralType::Int}))
                },
                _ => {
                    Err(ParseError{message: format!("Expected an int, found: {:?}", token)})
                }
            }
        }
        None => {
            Err(ParseError{message: String::from("Expected a primary, no tokens left")})
        }
    }
}