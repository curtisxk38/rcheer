
#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    IntLiteral,
    Plus,
    Minus,
    Star,
    LeftParen,
    RightParen,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
}