use super::{span::Span, token_kind::TokenKind};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
