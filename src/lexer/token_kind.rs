use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    DeclareAssign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Dollar,
    Comma,
    Semicolon,
    Newline,
    Colon,
    ThinArrow,
    Dot,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    And,
    Or,
    Not,
    Function,
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
    Unchecked,

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
            TokenKind::DeclareAssign => ":=",
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
            TokenKind::Newline => "\n",
            TokenKind::Colon => ":",
            TokenKind::ThinArrow => "->",
            TokenKind::Dot => ".",

            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",

            TokenKind::Function => "fun",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Return => "return",
            TokenKind::Print => "print",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Unchecked => "unchecked",

            TokenKind::Identifier => "identifier",
            TokenKind::StringLiteral => "string",
            TokenKind::NumberLiteral => "number",

            TokenKind::Invalid => "invalid",
            TokenKind::EndOfFile => "EOF",
        };

        write!(f, "{name}")
    }
}
