
#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    IntLiteral,
    Plus,
}