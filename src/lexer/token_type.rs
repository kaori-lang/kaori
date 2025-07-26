#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,

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

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,

    For,
    While,
    Break,
    Continue,

    If,
    Else,
    Return,
    Print,

    String,
    Float,
    Boolean,
    Identifier,
    Literal,

    Eof,
}
