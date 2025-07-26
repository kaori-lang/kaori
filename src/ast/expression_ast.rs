#[derive(Debug)]
pub enum ExpressionAST {
    Binary {
        operator: BinaryOperator,
        left: Box<ExpressionAST>,
        right: Box<ExpressionAST>,
    },
    Unary {
        operator: UnaryOperator,
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
pub enum BinaryOperator {
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
pub enum UnaryOperator {
    Negate,
    Not,
}
