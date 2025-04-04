#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,           // '+'
    Minus,          // '-'
    Multiply,       // '*'
    Divide,         // '/'
    LeftParen,      // '('
    RightParen,     // ')'
    LeftBrace,      // '{'
    RightBrace,     // '}'
    And,            // '&&'
    Or,             // '||'
    Not,            // '!'
    NotEqual,       // '!='
    Assign,         // '='
    Equal,          // '=='
    Greater,        // '>'
    GreaterEqual,   // '>='
    Less,           // '<'
    LessEqual,      // '<='
    Number(String), // Numeric value (e.g., 123, -45)
    String(String),
    Bool(bool),
    EndOfFile, // End of input
    Unknown,   // Unrecognized token
}
