use crate::lexer::span::Span;

use super::{expression_ast::ExpressionAST, type_ast::TypeAST};

#[derive(Debug)]
pub enum DeclarationAST {
    Variable {
        identifier: String,
        right: Box<ExpressionAST>,
        ty: TypeAST,
        span: Span,
    },
}
