#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    Increment,
    Decrement,

    And,
    Or,
    Not,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Assign,
    Comma,
    Semicolon,
    Colon,
    ThinArrow,

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

    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    Invalid,
    EndOfFile,
}
