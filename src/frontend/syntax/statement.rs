use crate::frontend::scanner::span::Span;

use super::{ast_node::ASTNode, declaration::Decl, expression::Expr};

#[derive(Debug)]
pub struct Stmt {
    pub span: Span,
    pub kind: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
    Print(Box<Expr>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    WhileLoop {
        condition: Box<Expr>,
        block: Box<Stmt>,
    },
    Block(Vec<ASTNode>),
    Expression(Box<Expr>),
    Break,
    Continue,
}

impl Stmt {
    pub fn print(expression: impl Into<Box<Expr>>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Print(expression.into()),
        }
    }

    pub fn if_(
        condition: impl Into<Box<Expr>>,
        then_branch: impl Into<Box<Stmt>>,
        else_branch: Option<Stmt>,
        span: Span,
    ) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::If {
                condition: condition.into(),
                then_branch: then_branch.into(),
                else_branch: match else_branch {
                    Some(branch) => Some(Box::new(branch)),
                    None => None,
                },
            },
        }
    }

    pub fn while_loop(
        condition: impl Into<Box<Expr>>,
        block: impl Into<Box<Stmt>>,
        span: Span,
    ) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::WhileLoop {
                condition: condition.into(),
                block: block.into(),
            },
        }
    }

    pub fn for_loop(
        declaration: Decl,
        condition: impl Into<Box<Expr>>,
        increment: Stmt,
        block: Stmt,
        span: Span,
    ) -> Stmt {
        if let StmtKind::Block(nodes) = block.kind {
            nodes.push(ASTNode::Statement(increment));
        }

        let while_loop_ = Stmt::while_loop(condition, block, span);

        let mut nodes: Vec<ASTNode> = Vec::new();
        nodes.push(ASTNode::Declaration(declaration));
        nodes.push(ASTNode::Statement(while_loop_));

        Stmt {
            span,
            kind: StmtKind::Block(nodes),
        }
    }

    pub fn block(declarations: Vec<ASTNode>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Block(declarations),
        }
    }

    pub fn expression(expr: impl Into<Box<Expr>>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Expression(expr.into()),
        }
    }

    pub fn break_(span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Break,
        }
    }

    pub fn continue_(span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Continue,
        }
    }
}
