use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,

    And,
    Or,
    Not,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Dollar,
    Comma,
    Semicolon,
    Colon,
    ThinArrow,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Struct,
    For,
    While,
    Break,
    Continue,
    If,
    Else,
    Return,
    Print,
    True,
    False,
    Bool,
    Number,

    Identifier,
    StringLiteral,
    NumberLiteral,

    Invalid,
    EndOfFile,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Multiply => "*",
            TokenKind::Divide => "/",
            TokenKind::Modulo => "%",

            TokenKind::Assign => "=",
            TokenKind::AddAssign => "+=",
            TokenKind::SubtractAssign => "-=",
            TokenKind::MultiplyAssign => "*=",
            TokenKind::DivideAssign => "/=",
            TokenKind::ModuloAssign => "%=",

            TokenKind::And => "and",
            TokenKind::Or => "or",
            TokenKind::Not => "not",
            TokenKind::NotEqual => "!=",
            TokenKind::Equal => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",

            TokenKind::Dollar => "$",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::Colon => ":",
            TokenKind::ThinArrow => "->",

            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",

            TokenKind::Function => "def",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Return => "return",
            TokenKind::Struct => "struct",
            TokenKind::Print => "print",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Bool => "bool",
            TokenKind::Number => "number",

            TokenKind::Identifier => "identifier",
            TokenKind::StringLiteral => "string",
            TokenKind::NumberLiteral => "number",

            TokenKind::Invalid => "invalid",
            TokenKind::EndOfFile => "EOF",
        };

        write!(f, "{name}")
    }
}
