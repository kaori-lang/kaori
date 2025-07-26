use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub position: usize,
    pub size: usize,
}
