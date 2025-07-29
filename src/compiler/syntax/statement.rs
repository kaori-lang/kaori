use crate::compiler::lexer::span::Span;

use super::{ast_node::ASTNode, expression::Expr};

#[derive(Debug)]
pub enum Stmt {
    Print {
        expression: Box<Expr>,
        span: Span,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        span: Span,
    },
    WhileLoop {
        condition: Box<Expr>,
        block: Box<Stmt>,
        span: Span,
    },
    Block {
        declarations: Vec<ASTNode>,
        span: Span,
    },
    Expression(Box<Expr>),
    Break(Span),
    Continue(Span),
}
