use super::data::Data;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    And,
    Or,
    Not,
    NotEqual,
    Assign,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Comma,
    Semicolon,

    Def,
    While,
    If,
    Else,
    Return,
    Print,

    String,
    Number,
    Boolean,
    Identifier,
    Literal,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub lexeme: String,
    pub literal: Data,
}
