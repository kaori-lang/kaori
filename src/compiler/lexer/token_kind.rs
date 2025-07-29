#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Arithmetic operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,

    // Unary arithmetic operators
    Increment,
    Decrement,

    // Logical operators
    And,
    Or,
    Not,

    // Comparison
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Assignment and separators
    Assign,
    Comma,
    Semicolon,
    Colon,
    ThinArrow,

    // Grouping
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    For,
    While,
    Break,
    Continue,
    If,
    Else,
    Return,
    Print,

    // Literals and identifiers
    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // Special
    Invalid,
    EndOfFile,
}
