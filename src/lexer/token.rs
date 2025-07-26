use super::{data::Data, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub lexeme: String,
    pub literal: Data,
}
