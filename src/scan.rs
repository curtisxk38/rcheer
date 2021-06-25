use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

pub enum ScanResult {
    Tokens(Vec<Token>),
    Error(ScanError),
}

#[derive(Debug)]
pub struct ScanError {
    pub message: String,
    pub line: i32,
    pub column: i32,
}

pub struct Scanner {
    line: i32,
    column: i32,
}

impl Scanner {

    pub fn new() -> Scanner {
        Scanner {line: 1, column: 0}
    }

    pub fn scan(&mut self, program: &str) -> ScanResult {
        let mut tokens = Vec::new();
        let mut chars = program.chars().peekable();

        loop {
            let peeked = chars.peek();
            match peeked {
                Some(char) => {
                    match char {
                        '+' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::Plus,
                                lexeme: String::from("+"), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '-' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::Minus,
                                lexeme: String::from("-"), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '*' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::Star, 
                                lexeme: String::from("*"), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '(' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::LeftParen, 
                                lexeme: String::from("("), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        ')' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::RightParen,
                                lexeme: String::from(")"), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '{' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::LeftBrace, 
                                lexeme: String::from("{"), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '}' => {
                            self.advance_char(&mut chars);
                            let token = Token {token_type: TokenType::RightBrace,
                                lexeme: String::from("}"), line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                            let char = char.clone();
                            self.advance_char(&mut chars);
                            let token = self.match_number(&mut chars, char);
                            tokens.push(token)
                        }
                        '0' => {
                            self.advance_char(&mut chars);
                            let token = Token { token_type: TokenType::IntLiteral, lexeme: String::from("0"),
                                line: self.line, column: self.column };
                            tokens.push(token);

                            if let Some(char) = chars.peek() {
                                match char {
                                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                                        return ScanResult::Error(ScanError {
                                            message: String::from("Leading zeros in integer literals are not permitted"),
                                            line: self.line,
                                            column: self.column,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            
                        }
                        '>' => {
                            self.advance_char(&mut chars);
                            let mut token_type = TokenType::Greater;
                            let mut lexeme = String::from(">");
                            match chars.peek() {
                                Some(char) => {
                                    match char {
                                        '=' => {
                                            self.advance_char(&mut chars);
                                            token_type = TokenType::GreaterEqual;
                                            lexeme = String::from(">=");
                                        }
                                        _ => {

                                        }
                                    }
                                }
                                None => {}
                            }
                            let token = Token {token_type, lexeme, line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '<' => {
                            self.advance_char(&mut chars);
                            let mut token_type = TokenType::Less;
                            let mut lexeme = String::from("<");
                            match chars.peek() {
                                Some(char) => {
                                    match char {
                                        '=' => {
                                            self.advance_char(&mut chars);
                                            token_type = TokenType::LessEqual;
                                            lexeme = String::from("<=");
                                        }
                                        _ => {

                                        }
                                    }
                                }
                                None => {}
                            }
                            let token = Token {token_type, lexeme, line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '=' => {
                            self.advance_char(&mut chars);
                            let mut token_type = TokenType::Equal;
                            let mut lexeme = String::from("=");
                            match chars.peek() {
                                Some(char) => {
                                    match char {
                                        '=' => {
                                            self.advance_char(&mut chars);
                                            token_type = TokenType::EqualEqual;
                                            lexeme = String::from("==");
                                        }
                                        _ => {

                                        }
                                    }
                                }
                                None => {}
                            }
                            let token = Token {token_type, lexeme, line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '!' => {
                            self.advance_char(&mut chars);
                            let mut token_type = TokenType::Bang;
                            let mut lexeme = String::from("!");
                            match chars.peek() {
                                Some(char) => {
                                    match char {
                                        '=' => {
                                            self.advance_char(&mut chars);
                                            token_type = TokenType::BangEqual;
                                            lexeme = String::from("!=");
                                        }
                                        _ => {

                                        }
                                    }
                                }
                                None => {}
                            }
                            let token = Token {token_type, lexeme, line: self.line, column: self.column};
                            tokens.push(token);
                        }
                        '\t'|' ' => {
                            self.advance_char(&mut chars)
                        }
                        '\n' => {
                            self.advance_char(&mut chars);
                            self.line += 1;
                            self.column = 0;
                        }
                        _ => {
                            if char.is_alphabetic() {
                                let token = self.match_alphabetic(&mut chars);
                                tokens.push(token);
                            } else {
                                return ScanResult::Error(ScanError {
                                    message: format!("Unrecognized input {}", char),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
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

    fn match_number(&mut self, chars: &mut Peekable<Chars>, first_char: char) -> Token {
        let mut lexeme = format!("{}", first_char);
        loop {
            match chars.peek() {
                Some(char) => {
                    match char {
                        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                            lexeme.push(*char);
                            self.advance_char(chars)
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
        Token {token_type: TokenType::IntLiteral, lexeme, line: self.line, column: self.column}
    }

    fn match_alphabetic(&mut self, chars: &mut Peekable<Chars>) -> Token {
        let mut lexeme = String::from("");
        loop {
            if let Some(possible_alphabetic) = chars.peek() {
                if possible_alphabetic.is_alphanumeric() {
                    lexeme.push(*possible_alphabetic);
                    self.advance_char(chars);
                } else {
                    break;
                }
            } else {
                break;
            }
        };

        let token_type = match &lexeme[..] {
            "else" => TokenType::Else,
            "if" => TokenType::If,
            _ => TokenType::Identifier,
        };
        Token {token_type, lexeme, line: self.line, column: self.column}
    }

    fn advance_char(&mut self, chars: &mut Peekable<Chars>) {
        chars.next();
        self.column += 1;
    }
}