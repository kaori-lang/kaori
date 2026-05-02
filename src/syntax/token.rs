use logos::Logos;
use std::fmt;
#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t\r\n\f]+")]
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
    #[token(";")]
    Semicolon,
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
            Token::DeclareAssign => "`:=`",
            Token::Assign => "`=`",
            Token::AddAssign => "`+=`",
            Token::SubtractAssign => "`-=`",
            Token::MultiplyAssign => "`*=`",
            Token::DivideAssign => "`/=`",
            Token::ModuloAssign => "`%=`",
            Token::Plus => "`+`",
            Token::Minus => "`-`",
            Token::Multiply => "`*`",
            Token::Divide => "`/`",
            Token::Modulo => "`%`",
            Token::NotEqual => "`!=`",
            Token::Equal => "`==`",
            Token::GreaterEqual => "`>=`",
            Token::LessEqual => "`<=`",
            Token::Greater => "`>`",
            Token::Less => "`<`",
            Token::Comma => "`,`",
            Token::Semicolon => "`;`",
            Token::Colon => "`:`",
            Token::Dot => "`.`",
            Token::Pipe => "`|`",
            Token::LeftParen => "`(`",
            Token::RightParen => "`)`",
            Token::LeftBrace => "`{`",
            Token::RightBrace => "`}`",
            Token::And => "`and`",
            Token::Or => "`or`",
            Token::Not => "`not`",
            Token::Function => "`fn`",
            Token::For => "`for`",
            Token::While => "`while`",
            Token::Break => "`break`",
            Token::Continue => "`continue`",
            Token::If => "`if`",
            Token::Else => "`else`",
            Token::Return => "`return`",
            Token::Print => "`print`",
            Token::True => "`true`",
            Token::False => "`false`",
            Token::To => "`to`",
            Token::DownTo => "`downto`",
            Token::By => "`by`",
            Token::NumberLiteral => "<number literal>",
            Token::StringLiteral => "<string literal>",
            Token::UnterminatedStringLiteral => "<unterminated string>",
            Token::Identifier => "<identifier>",
            Token::Eof => "<end of file>",
        };
        write!(f, "{}", s)
    }
}
