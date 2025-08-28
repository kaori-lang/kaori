use std::fmt;

#[derive(PartialEq, Copy, Clone)]
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
    Struct,
    Print,

    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

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

            TokenKind::Increment => "++",
            TokenKind::Decrement => "--",

            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::Not => "!",
            TokenKind::NotEqual => "!=",
            TokenKind::Equal => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",

            TokenKind::Assign => "=",
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

            TokenKind::Identifier => "identifier",
            TokenKind::StringLiteral => "string",
            TokenKind::NumberLiteral => "number",
            TokenKind::BooleanLiteral => "boolean",

            TokenKind::Invalid => "invalid",
            TokenKind::EndOfFile => "EOF",
        };

        write!(f, "{name}")
    }
}
