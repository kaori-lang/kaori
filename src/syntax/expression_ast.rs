#[derive(Debug)]
pub enum ExpressionAST {
    Binary {
        operator: BinaryOp,
        left: Box<ExpressionAST>,
        right: Box<ExpressionAST>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<ExpressionAST>,
    },
    Assign {
        identifier: String,
        right: Box<ExpressionAST>,
    },
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Negate,
    Not,
}
