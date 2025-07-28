use crate::compiler::lexer::span::Span;

use super::{ast_node::ASTNode, expression_ast::ExpressionAST};

#[derive(Debug)]
pub enum StatementAST {
    Print {
        expression: Box<ExpressionAST>,
        span: Span,
    },

    Expression {
        expression: Box<ExpressionAST>,
        span: Span,
    },
    If {
        condition: Box<ExpressionAST>,
        then_branch: Box<StatementAST>,
        else_branch: Option<Box<StatementAST>>,
        span: Span,
    },
    WhileLoop {
        condition: Box<ExpressionAST>,
        block: Box<StatementAST>,
        span: Span,
    },
    Block {
        declarations: Vec<ASTNode>,
        span: Span,
    },
    Break(Span),
    Continue(Span),
}
