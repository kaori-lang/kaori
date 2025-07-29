use crate::compiler::lexer::span::Span;

use super::{expression::Expression, type_ast::TypeAST};

#[derive(Debug)]
pub enum DeclarationAST {
    Variable {
        identifier: Box<Expression>,
        right: Box<Expression>,
        ty: TypeAST,
        span: Span,
    },
}
