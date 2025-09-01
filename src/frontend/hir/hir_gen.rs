use crate::frontend::syntax::{
    ast_node::AstNode,
    decl::{Decl, DeclKind},
    expr::{Expr, ExprKind},
    operator::{BinaryOp, UnaryOp},
    stmt::{Stmt, StmtKind},
};

use super::{hir_ast_node::HirAstNode, hir_decl::HirDecl, hir_expr::HirExpr, hir_stmt::HirStmt};

pub fn generate_hir(declarations: &[Decl]) -> Vec<HirDecl> {
    declarations.iter().map(generate_declaration).collect()
}

fn generate_nodes(nodes: &[AstNode]) -> Vec<HirAstNode> {
    nodes.iter().map(generate_ast_node).collect()
}

fn generate_ast_node(node: &AstNode) -> HirAstNode {
    match node {
        AstNode::Declaration(declaration) => {
            let declaration = generate_declaration(declaration);

            HirAstNode::Declaration(declaration)
        }
        AstNode::Statement(statement) => {
            let statement = generate_statement(statement);

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
            let right = generate_expression(right);

            HirDecl::variable(name.to_owned(), right, ty.to_owned(), declaration.span)
        }
        DeclKind::Function {
            parameters,
            body,
            name,
            return_ty,
        } => {
            let body = generate_nodes(body);
            let parameters = parameters.iter().map(generate_declaration).collect();

            HirDecl::function(
                name.to_owned(),
                parameters,
                body,
                return_ty.to_owned(),
                declaration.span,
            )
        }
        DeclKind::Struct { name, fields } => {
            let fields = fields.iter().map(generate_declaration).collect();

            HirDecl::struct_(name.to_owned(), fields, declaration.span)
        }
    }
}

fn generate_statement(statement: &Stmt) -> HirStmt {
    match &statement.kind {
        StmtKind::Expression(expression) => {
            let expr = generate_expression(expression);

            HirStmt::expression(expr, statement.span)
        }
        StmtKind::Print(expression) => {
            let expr = generate_expression(expression);

            HirStmt::print(expr, statement.span)
        }
        StmtKind::Block(nodes) => {
            let nodes = generate_nodes(nodes);

            HirStmt::block(nodes, statement.span)
        }
        StmtKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition = generate_expression(condition);
            let then_branch = generate_statement(then_branch);
            let else_branch = if let Some(branch) = else_branch {
                Some(generate_statement(branch))
            } else {
                None
            };

            HirStmt::branch_(condition, then_branch, else_branch, statement.span)
        }
        StmtKind::WhileLoop { condition, block } => {
            let condition = generate_expression(condition);
            let block = generate_statement(block);

            HirStmt::while_loop(condition, block, statement.span)
        }
        StmtKind::ForLoop {
            init,
            condition,
            increment,
            block,
        } => {
            /*    let mut nodes = Vec::new();
            let init = HirAstNode::Declaration(generate_declaration(init));

            nodes.push(init);

            let condition = generate_expression(condition);
            let while_loop_block = generate_statement(block);

            if let HirStmtKind::Block(mut nodes) = &while_loop_block.kind {
                let increment = generate_statement(increment);

                nodes.push(HirAstNode::Statement(increment));
            };

            let while_loop = HirStmt::while_loop(condition, while_loop_block, statement.span);

            nodes.push(HirAstNode::Statement(while_loop));

            HirStmt::block(nodes, statement.span) */
            todo!()
        }
        StmtKind::Break => HirStmt::break_(statement.span),

        StmtKind::Continue => HirStmt::continue_(statement.span),

        StmtKind::Return(expr) => {
            let expr = match expr {
                Some(expr) => Some(generate_expression(expr)),
                None => None,
            };

            HirStmt::return_(expr, statement.span)
        }
    }
}

fn generate_expression(expression: &Expr) -> HirExpr {
    match &expression.kind {
        ExprKind::Assign { left, right } => {
            let right = generate_expression(right);
            let left = generate_expression(left);
            let span = expression.span;

            HirExpr::assign(left, right, span)
        }
        ExprKind::Binary {
            left,
            right,
            operator,
        } => {
            let left = generate_expression(left);
            let right = generate_expression(right);
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
            let right = generate_expression(right);
            let span = expression.span;

            match operator {
                UnaryOp::Not => HirExpr::not(right, span),
                UnaryOp::Negate => HirExpr::negate(right, span),
                UnaryOp::Increment => {
                    let left = HirExpr::number_literal(1.0, span);

                    HirExpr::assign(right.to_owned(), HirExpr::add(left, right, span), span)
                }
                UnaryOp::Decrement => {
                    let left = HirExpr::number_literal(1.0, span);

                    HirExpr::assign(right.to_owned(), HirExpr::sub(left, right, span), span)
                }
            }
        }
        ExprKind::FunctionCall { callee, arguments } => {
            let span = expression.span;
            let callee = generate_expression(callee);
            let arguments = arguments
                .iter()
                .map(|arg| generate_expression(arg))
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
