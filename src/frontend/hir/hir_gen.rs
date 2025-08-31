use crate::frontend::syntax::{
    ast_node::AstNode,
    decl::{Decl, DeclKind},
    expr::{Expr, ExprKind},
    operator::{BinaryOp, UnaryOp},
    stmt::{Stmt, StmtKind},
};

use super::{hir_ast_node::HirAstNode, hir_decl::HirDecl, hir_expr::HirExpr, hir_stmt::HirStmt};

pub struct HirGen {}

impl HirGen {
    pub fn generate(declarations: &[Decl]) -> Vec<HirDecl> {
        declarations
            .iter()
            .map(|declaration| HirGen::generate_declaration(declaration))
            .collect()
    }

    fn generate_nodes(nodes: &[AstNode]) -> Vec<HirAstNode> {
        nodes
            .iter()
            .map(|node| HirGen::generate_ast_node(node))
            .collect()
    }

    fn generate_ast_node(node: &AstNode) -> HirAstNode {
        match node {
            AstNode::Declaration(declaration) => {
                let declaration = HirGen::generate_declaration(declaration);

                HirAstNode::Declaration(declaration)
            }
            AstNode::Statement(statement) => {
                let statement = HirGen::generate_statement(statement);

                HirAstNode::Statement(statement)
            }
        }
    }

    fn generate_declaration(declaration: &Decl) -> HirDecl {
        match &declaration.kind {
            DeclKind::Parameter { name, ty } => {
                HirDecl::parameter(name.to_owned(), ty.to_owned(), declaration.span)
            }
            DeclKind::Field { name, ty } => {
                HirDecl::field(name.to_owned(), ty.to_owned(), declaration.span)
            }
            DeclKind::Variable { name, right, ty } => {
                let right = HirGen::generate_expression(right);

                HirDecl::variable(name.to_owned(), right, ty.to_owned(), declaration.span)
            }
            DeclKind::Function {
                parameters,
                body,
                name,
                ty,
            } => {
                let body = HirGen::generate_nodes(body);
                let parameters = parameters
                    .iter()
                    .map(|param| HirGen::generate_declaration(param))
                    .collect();

                HirDecl::function(
                    name.to_owned(),
                    parameters,
                    body,
                    ty.to_owned(),
                    declaration.span,
                )
            }
            DeclKind::Struct { name, fields, ty } => {
                let fields = fields
                    .iter()
                    .map(|field| HirGen::generate_declaration(field))
                    .collect();

                HirDecl::struct_(name.to_owned(), fields, ty.to_owned(), declaration.span)
            }
        }
    }

    fn generate_statement(statement: &Stmt) -> HirStmt {
        match &statement.kind {
            StmtKind::Expression(expression) => {
                let expr = HirGen::generate_expression(expression);

                HirStmt::expression(expr, statement.span)
            }
            StmtKind::Print(expression) => {
                let expr = HirGen::generate_expression(expression);

                HirStmt::print(expr, statement.span)
            }
            StmtKind::Block(nodes) => {
                let nodes = HirGen::generate_nodes(nodes);

                HirStmt::block(nodes, statement.span)
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = HirGen::generate_expression(condition);
                let then_branch = HirGen::generate_statement(then_branch);
                let else_branch = if let Some(branch) = else_branch {
                    Some(HirGen::generate_statement(branch))
                } else {
                    None
                };

                HirStmt::branch_(condition, then_branch, else_branch, statement.span)
            }
            StmtKind::WhileLoop { condition, block } => {
                let condition = HirGen::generate_expression(condition);
                let block = HirGen::generate_statement(block);

                HirStmt::while_loop(condition, block, statement.span)
            }
            StmtKind::Break => HirStmt::break_(statement.span),

            StmtKind::Continue => HirStmt::continue_(statement.span),

            StmtKind::Return(expr) => {
                let expr = match expr {
                    Some(expr) => Some(HirGen::generate_expression(expr)),
                    None => None,
                };

                HirStmt::return_(expr, statement.span)
            }
        }
    }

    fn generate_expression(expression: &Expr) -> HirExpr {
        match &expression.kind {
            ExprKind::Assign { left, right } => {
                let right = HirGen::generate_expression(right);
                let left = HirGen::generate_expression(left);
                let span = expression.span;

                HirExpr::assign(left, right, span)
            }
            ExprKind::Binary {
                left,
                right,
                operator,
            } => {
                let left = HirGen::generate_expression(left);
                let right = HirGen::generate_expression(right);
                let span = expression.span;

                match operator {
                    BinaryOp::Add => HirExpr::add(left, right, span),
                    BinaryOp::Subtract => HirExpr::sub(left, right, span),
                    BinaryOp::Multiply => HirExpr::mul(left, right, span),
                    BinaryOp::Divide => HirExpr::div(left, right, span),
                    BinaryOp::Modulo => HirExpr::mod_(left, right, span),
                    BinaryOp::Equal => HirExpr::equal(left, right, span),
                    BinaryOp::NotEqual => HirExpr::not_equal(left, right, span),
                    BinaryOp::Less => HirExpr::less(left, right, span),
                    BinaryOp::LessEqual => HirExpr::less_equal(left, right, span),
                    BinaryOp::Greater => HirExpr::greater(left, right, span),
                    BinaryOp::GreaterEqual => HirExpr::greater_equal(left, right, span),
                    BinaryOp::And => HirExpr::and(left, right, span),
                    BinaryOp::Or => HirExpr::or(left, right, span),
                }
            }
            ExprKind::Unary { right, operator } => {
                let right = HirGen::generate_expression(right);
                let span = expression.span;

                match operator {
                    UnaryOp::Not => HirExpr::not(right, span),
                    UnaryOp::Negate => HirExpr::negate(right, span),
                }
            }
            ExprKind::FunctionCall { callee, arguments } => {
                let span = expression.span;
                let callee = HirGen::generate_expression(callee);
                let arguments = arguments
                    .iter()
                    .map(|arg| HirGen::generate_expression(arg))
                    .collect();

                HirExpr::function_call(callee, arguments, span)
            }
            ExprKind::NumberLiteral(value) => HirExpr::number_literal(*value, expression.span),
            ExprKind::BooleanLiteral(value) => HirExpr::boolean_literal(*value, expression.span),
            ExprKind::StringLiteral(value) => {
                HirExpr::string_literal(value.to_owned(), expression.span)
            }
            ExprKind::Identifier { name } => HirExpr::identifier(name.to_owned(), expression.span),
        }
    }
}
