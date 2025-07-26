use super::expression_ast::ExpressionAST;

#[derive(Debug)]
pub enum StatementAST {
    Print {
        expression: Box<ExpressionAST>,
        line: u32,
    },

    Expression {
        expression: Box<ExpressionAST>,
        line: u32,
    },
    If {
        condition: Box<ExpressionAST>,
        then_branch: Box<StatementAST>,
        else_branch: Option<Box<StatementAST>>,
        line: u32,
    },
    WhileLoop {
        condition: Box<ExpressionAST>,
        block: Box<StatementAST>,
        line: u32,
    },
    Block {
        statements: Vec<Box<StatementAST>>,
        line: u32,
    },
    Break {
        line: u32,
    },
    Continue {
        line: u32,
    },
}
