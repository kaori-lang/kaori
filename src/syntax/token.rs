use logos::Logos;
use std::{fmt, ops::Range};

#[derive(Default, Debug, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn merge(self, other: Span) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start as u32,
            end: value.end as u32,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.start as usize..value.end as usize
    }
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t\f\r\n]+")]
pub enum Token {
    #[token("=")]
    Assign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubtractAssign,
    #[token("*=")]
    MultiplyAssign,
    #[token("/=")]
    DivideAssign,
    #[token("%=")]
    ModuloAssign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("^")]
    Caret,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("!=")]
    NotEqual,
    #[token("==")]
    Equal,
    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("|")]
    Pipe,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("#")]
    Hash,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("ref")]
    Ref,
    #[token("fun")]
    Function,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,
    #[token("use")]
    Use,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Nil,
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    NumberLiteral,
    #[token(";")]
    Semicolon,
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Assign => "`=`",
            Self::AddAssign => "`+=`",
            Self::SubtractAssign => "`-=`",
            Self::MultiplyAssign => "`*=`",
            Self::DivideAssign => "`/=`",
            Self::ModuloAssign => "`%=`",
            Self::Plus => "`+`",
            Self::Minus => "`-`",
            Self::Multiply => "`*`",
            Self::Divide => "`/`",
            Self::Modulo => "`%`",
            Self::Caret => "`^`",
            Self::NotEqual => "`!=`",
            Self::Equal => "`==`",
            Self::GreaterEqual => "`>=`",
            Self::LessEqual => "`<=`",
            Self::Greater => "`>`",
            Self::Less => "`<`",
            Self::Comma => "`,`",
            Self::Colon => "`:`",
            Self::Semicolon => "`;`",
            Self::Dot => "`.`",
            Self::Pipe => "`|`",
            Self::LeftParen => "`(`",
            Self::RightParen => "`)`",
            Self::Hash => "`#`",
            Self::LeftBrace => "`{`",
            Self::RightBrace => "`}`",
            Self::And => "`and`",
            Self::Or => "`or`",
            Self::Not => "`not`",
            Self::Let => "`let`",
            Self::Const => "`const`",
            Self::Ref => "`ref`",
            Self::Function => "`fun`",
            Self::For => "`for`",
            Self::While => "`while`",
            Self::Break => "`break`",
            Self::Continue => "`continue`",
            Self::If => "`if`",
            Self::Else => "`else`",
            Self::Return => "`return`",
            Self::Use => "`use`",
            Self::True => "`true`",
            Self::False => "`false`",
            Self::Nil => "`nil`",
            Self::NumberLiteral => "<number literal>",
            Self::StringLiteral => "<string literal>",
            Self::Identifier => "<identifier>",
            Self::Eof => "<end of file>",
        };
        write!(f, "{}", s)
    }
}
