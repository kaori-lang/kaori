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
    Number,       // Numeric value (e.g., 123, -45)
    String,
    Bool,
    EndOfFile, // End of input
    Invalid,   // Unrecognized token
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub line: u32,
    pub value: Option<String>,
}

impl Token {
    pub fn new(ty: TokenType, line: u32, value: Option<String>) -> Self {
        Self { ty, line, value }
    }
}
