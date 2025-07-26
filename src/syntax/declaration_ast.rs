use super::expression_ast::ExpressionAST;

pub enum DeclarationAST {
    Variable {
        left: ExpressionAST,
        right: Box<ExpressionAST>,
        line: u32,
    },
}
