#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Plus,         // '+'
    Minus,        // '-'
    Multiply,     // '*'
    Divide,       // '/'
    LeftParen,    // '('
    RightParen,   // ')'
    LeftBrace,    // '{'
    RightBrace,   // '}'
    And,          // '&&'
    Or,           // '||'
    Not,          // '!'
    NotEqual,     // '!='
    Assign,       // '='
    Equal,        // '=='
    Greater,      // '>'
    GreaterEqual, // '>='
    Less,         // '<'
    LessEqual,    // '<='
    Comma,        // ','
    Semicolon,    // ';'

    Def,
    While,
    If,
    Else,
    Return,

    DataType,
    Identifier,
    Number, // Numeric value (e.g., 123, -45)
    String,
    Boolean,
    EndOfFile, // End of input
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub value: String,
}

impl Token {
    pub fn new(ty: TokenType, line: u32, value: String) -> Self {
        Self { ty, line, value }
    }
}
