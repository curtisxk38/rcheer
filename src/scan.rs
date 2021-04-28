use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

pub enum ScanResult {
    Tokens(Vec<Token>),
    Error(String),
}

pub fn scan(program: &str) -> ScanResult {
    let mut tokens = Vec::new();
    let mut chars = program.chars().peekable();

    loop {
        let peeked = chars.peek();
        match peeked {
            Some(char) => {
                match char {
                    '+' => {
                        chars.next();
                        let token = Token {token_type: TokenType::Plus, lexeme: format!("+")};
                        tokens.push(token);
                    }
                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                        let char = char.clone();
                        chars.next();
                        let token = match_number(&mut chars, char);
                        tokens.push(token)
                    }
                    '\t'|' ' => {
                        chars.next();
                    }
                    '\n' => {
                        chars.next();
                    }
                    _ => {
                        return ScanResult::Error(format!("Unrecognized input {}", char));
                    }
                };
            }
            None => {
                break;
            }
        }
    }

    ScanResult::Tokens(tokens)
}

fn match_number(chars: &mut Peekable<Chars>, first_char: char) -> Token {
    let mut lexeme = format!("{}", first_char);
    loop {
        match chars.peek() {
            Some(char) => {
                match char {
                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                        lexeme.push(*char);
                        chars.next();
                    }
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    };
    Token {token_type: TokenType::IntLiteral, lexeme}
}