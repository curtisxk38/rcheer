
#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: i32,
    pub column: i32,
}

#[derive(Debug)]
pub enum TokenType {
    IntLiteral,
    Plus,
    Minus,
    Star,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    If,
    Else,
    Identifier,
}