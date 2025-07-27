use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}
