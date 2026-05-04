use logos::Logos;
use std::fmt;
#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t\f\r\n]+")]
pub enum Token {
    #[token(":=")]
    DeclareAssign,
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
    #[token("native")]
    Native,
    #[token("fn")]
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
    #[token("print")]
    Print,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("to")]
    To,
    #[token("downto")]
    DownTo,
    #[token("by")]
    By,
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    NumberLiteral,
    #[token(";")]
    Semicolon,
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[regex(r#""([^"\\]|\\.)*"#)]
    UnterminatedStringLiteral,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DeclareAssign => "`:=`",
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
            Self::LeftBrace => "`{`",
            Self::RightBrace => "`}`",
            Self::And => "`and`",
            Self::Or => "`or`",
            Self::Not => "`not`",
            Self::Native => "`native`",
            Self::Function => "`fn`",
            Self::For => "`for`",
            Self::While => "`while`",
            Self::Break => "`break`",
            Self::Continue => "`continue`",
            Self::If => "`if`",
            Self::Else => "`else`",
            Self::Return => "`return`",
            Self::Print => "`print`",
            Self::True => "`true`",
            Self::False => "`false`",
            Self::To => "`to`",
            Self::DownTo => "`downto`",
            Self::By => "`by`",
            Self::NumberLiteral => "<number literal>",
            Self::StringLiteral => "<string literal>",
            Self::UnterminatedStringLiteral => "<unterminated string>",
            Self::Identifier => "<identifier>",
            Self::Eof => "<end of file>",
        };
        write!(f, "{}", s)
    }
}
