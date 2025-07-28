use crate::compiler::syntax::{
    ast_node::ASTNode, expression_ast::ExpressionAST, statement_ast::StatementAST,
    type_ast::TypeAST,
};

use super::visitor::Visitor;

pub struct Resolver {
    declarations: Vec<ASTNode>,
}

impl Resolver {
    fn new(declarations: Vec<ASTNode>) -> Self {
        Self { declarations }
    }
}
impl Visitor<TypeAST> for Resolver {
    fn visit_statement(&self, statement: StatementAST) {
        match statement {
            StatementAST::Expression { expression, span } => {
                self.visit_expression(*expression);
                ()
            }
            _ => (),
        }
    }

    fn visit_expression(&self, expression: ExpressionAST) -> TypeAST {
        match expression {
            ExpressionAST::NumberLiteral(number) => TypeAST::Number,
            ExpressionAST::BooleanLiteral(bool) => TypeAST::Boolean,
            ExpressionAST::StringLiteral(str) => TypeAST::String,
            ExpressionAST::Identifier(identifier) => TypeAST::String,
            ExpressionAST::Assign { identifier, right } => TypeAST::String,
            ExpressionAST::Binary {
                operator,
                left,
                right,
            } => {
                let left = self.visit_expression(*left);
                let right = self.visit_expression(*right);

                TypeAST::Boolean
            }
            _ => TypeAST::Boolean,
        }
    }
}
