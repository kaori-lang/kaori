use logos::Logos;

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

    // Keywords
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
    #[token("unchecked")]
    Unchecked,
    #[token("to")]
    To,
    #[token("downto")]
    DownTo,
    #[token("by")]
    By,

    // Literals & Identifier
    // Number: integer or float (e.g. 42, 3.14)
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    NumberLiteral,

    // String: "hello world"
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[regex(r#""([^"\\]|\\.)*"#)]
    UnterminatedStringLiteral,

    // Identifier: must come AFTER all keywords
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    Eof,
}
