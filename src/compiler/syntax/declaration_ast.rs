use crate::compiler::lexer::span::Span;

use super::{expression::Expr, type_ast::TypeAST};

#[derive(Debug)]
pub enum DeclarationAST {
    Variable {
        identifier: Box<Expr>,
        right: Box<Expr>,
        ty: TypeAST,
        span: Span,
    },
}
