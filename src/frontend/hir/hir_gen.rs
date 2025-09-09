use crate::frontend::syntax::{
    ast_node::AstNode,
    decl::{Decl, DeclKind},
    expr::{Expr, ExprKind},
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

            HirAstNode::from(declaration)
        }
        AstNode::Statement(statement) => {
            let statement = generate_statement(statement);

            HirAstNode::from(statement)
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
            let else_branch = else_branch
                .as_ref()
                .map(|branch| generate_statement(branch));

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
            let mut inner_block: Vec<HirAstNode> = match &block.kind {
                StmtKind::Block(nodes) => nodes.iter().map(generate_ast_node).collect(),
                _ => unreachable!(),
            };

            let increment = HirAstNode::from(generate_statement(increment));

            inner_block.push(increment);

            let inner_block = HirStmt::block(inner_block, block.span);

            let condition = generate_expression(condition);
            let while_loop = HirStmt::while_loop(condition, inner_block, block.span);

            let mut outer_block = Vec::new();
            let init = HirAstNode::from(generate_declaration(init));
            let while_loop = HirAstNode::from(while_loop);

            outer_block.push(init);
            outer_block.push(while_loop);

            HirStmt::block(outer_block, statement.span)
        }
        StmtKind::Break => HirStmt::break_(statement.span),

        StmtKind::Continue => HirStmt::continue_(statement.span),

        StmtKind::Return(expr) => {
            let expr = expr.as_ref().map(|e| generate_expression(e));

            HirStmt::return_(expr, statement.span)
        }
    }
}

fn generate_expression(expression: &Expr) -> HirExpr {
    match &expression.kind {
        ExprKind::Assign { left, right } => {
            let right = generate_expression(right);
            let left = generate_expression(left);

            HirExpr::assign(left, right, expression.span)
        }
        ExprKind::Binary {
            left,
            right,
            operator,
        } => {
            let left = generate_expression(left);
            let right = generate_expression(right);

            HirExpr::binary(*operator, left, right, expression.span)
        }
        ExprKind::Unary { right, operator } => {
            let right = generate_expression(right);

            HirExpr::unary(*operator, right, expression.span)
        }
        ExprKind::FunctionCall { callee, arguments } => {
            let callee = generate_expression(callee);
            let arguments = arguments.iter().map(generate_expression).collect();

            HirExpr::function_call(callee, arguments, expression.span)
        }
        ExprKind::NumberLiteral(value) => HirExpr::number_literal(*value, expression.span),
        ExprKind::BooleanLiteral(value) => HirExpr::boolean_literal(*value, expression.span),
        ExprKind::StringLiteral(value) => {
            HirExpr::string_literal(value.to_owned(), expression.span)
        }
        ExprKind::Identifier(name) => HirExpr::identifier(name.to_owned(), expression.span),
    }
}
