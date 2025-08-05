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
    pub fn print(expression: Expr, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Print(Box::new(expression)),
        }
    }

    pub fn if_(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: match else_branch {
                    Some(branch) => Some(Box::new(branch)),
                    None => None,
                },
            },
        }
    }

    pub fn while_loop(condition: Expr, block: Stmt, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::WhileLoop {
                condition: Box::new(condition),
                block: Box::new(block),
            },
        }
    }

    pub fn for_loop(
        declaration: Decl,
        condition: Expr,
        increment: Stmt,
        mut block: Stmt,
        span: Span,
    ) -> Stmt {
        if let StmtKind::Block(nodes) = &mut block.kind {
            nodes.push(ASTNode::Statement(increment));
        }

        let while_loop_ = Stmt::while_loop(condition, block, span);

        let nodes: Vec<ASTNode> = vec![
            ASTNode::Declaration(declaration),
            ASTNode::Statement(while_loop_),
        ];

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

    pub fn expression(expr: Expr, span: Span) -> Stmt {
        Stmt {
            span,
            kind: StmtKind::Expression(Box::new(expr)),
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
