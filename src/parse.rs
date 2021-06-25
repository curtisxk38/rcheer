use std::{fmt::format, iter::Peekable, slice::Iter};

use crate::{ast::{Binary, BinaryOp, Expr, Grouping, If, Literal, LiteralType, Unary, UnaryOp}, token::{Token, TokenType}};


pub enum ParseResult<'t> {
    AST(Expr<'t>),
    Error(ParseError)
}

pub struct ParseError {
    pub message: String,
}

// program => expression ;
pub fn parse(tokens: &Vec<Token>) -> ParseResult {
    let mut tokens = tokens.iter().peekable();
    match expression(&mut tokens) {
        Ok(expr) => {
            match tokens.peek() {
                Some(token) => {
                    // finished parsing, but there's still some tokens left
                    ParseResult::Error(ParseError {message: format!("Finished parsing, but some tokens remain: {:?}", token)})
                }
                None => {
                    ParseResult::AST(expr)
                }
            }
        }
        Err(err) => {
            ParseResult::Error(err)
        }
    }
}

// expression -> equality | if_expr
fn expression<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    match tokens.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::If => {
                    if_expr(tokens)
                }
                _ => {
                    equality(tokens)
                }
            }
        }
        None => {Err(ParseError{message: String::from("Reached EOF while parsing")})}
    }
}

// if_expr -> "if" expression "{" expression "}" ("else {" expression "}")?
fn if_expr<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    let token = tokens.next().unwrap(); // consume "if"
    let condition = Box::new(expression(tokens)?);
    
    match tokens.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::LeftBrace => {
                    tokens.next();
                }
                _ => {
                    return Err(ParseError{message: format!("Expect {{ got: {:?}", token)});
                }
            }
        }
        None => {
            return Err(ParseError{message: format!("Expect {{ reached EOF")});
        }
    };

    let then_branch = Box::new(expression(tokens)?);

    match tokens.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::RightBrace => {
                    tokens.next();
                }
                _ => {
                    return Err(ParseError{message: format!("Expect }} got: {:?}", token)});
                }
            }
        }
        None => {
            return Err(ParseError{message: format!("Expect }} reached EOF")});
        }
    };

    let else_branch = match tokens.peek() {
        Some(token) => {
            match token.token_type {
                TokenType::Else => {
                    tokens.next(); // consume "else"
                    // match {
                    match tokens.peek() {
                        Some(token) => {
                            match token.token_type {
                                TokenType::LeftBrace => {
                                    tokens.next();
                                }
                                _ => {
                                    return Err(ParseError{message: format!("Expect {{ got: {:?}", token)});
                                }
                            }
                        }
                        None => {
                            return Err(ParseError{message: format!("Expect {{ reached EOF")});
                        }
                    };
                
                    let else_branch = expression(tokens)?;
                
                    // match }
                    match tokens.peek() {
                        Some(token) => {
                            match token.token_type {
                                TokenType::RightBrace => {
                                    tokens.next();
                                }
                                _ => {
                                    return Err(ParseError{message: format!("Expect }} got: {:?}", token)});
                                }
                            }
                        }
                        None => {
                            return Err(ParseError{message: format!("Expect }} reached EOF")});
                        }
                    };
                    Some(Box::new(else_branch))
                }
                _ => None
            }
        }
        None => None
    };    

    Ok(Expr::If(If{token, condition, then_branch, else_branch, type_kind: None}))
}

// equality -> comparison ( ( "!=" | "==" ) comparison )*
fn equality<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    let mut expr = comparison(tokens)?;
    loop {
        let op_token;
        let operation;
        match tokens.peek() {
            Some(token) => {
                match token.token_type {
                    TokenType::BangEqual => {
                        op_token = *token;
                        operation = BinaryOp::BangEqual;
                        tokens.next();
                    }
                    TokenType::EqualEqual => {
                        op_token = *token;
                        operation = BinaryOp::EqualEqual;
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
        let right = comparison(tokens)?;
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right), type_kind: None})
    }
    Ok(expr)
}
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn comparison<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
    let mut expr = term(tokens)?;
    loop {
        let op_token;
        let operation;
        match tokens.peek() {
            Some(token) => {
                match token.token_type {
                    TokenType::Greater => {
                        op_token = *token;
                        operation = BinaryOp::Greater;
                        tokens.next();
                    }
                    TokenType::GreaterEqual => {
                        op_token = *token;
                        operation = BinaryOp::GreaterEqual;
                        tokens.next();
                    }
                    TokenType::Less => {
                        op_token = *token;
                        operation = BinaryOp::Less;
                        tokens.next();
                    }
                    TokenType::LessEqual => {
                        op_token = *token;
                        operation = BinaryOp::LessEqual;
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
        let right = term(tokens)?;
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right), type_kind: None})
    }
    Ok(expr)
}

// term => factor (("+"|"-") factor)* ;
fn term<'t>(tokens: &mut Peekable<Iter<'t, Token>>) -> Result<Expr<'t>, ParseError> {
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
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right), type_kind: None})
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
        expr = Expr::Binary(Binary {token: op_token, operation, left: Box::new(expr), right: Box::new(right), type_kind: None})
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
                    Ok(Expr::Unary(Unary {token: op_token, operation, right: Box::new(right), type_kind: None}))
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
                    Ok(Expr::Literal(Literal { token, literal_type: LiteralType::Int, type_kind: None}))
                }
                TokenType::LeftParen => {
                    tokens.next();
                    let expr = expression(tokens)?;
                    let node = Expr::Grouping(Grouping {expr: Box::new(expr), type_kind: None});
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