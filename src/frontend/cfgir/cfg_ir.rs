use std::collections::HashMap;

use crate::frontend::semantic::{
    hir_decl::{HirDecl, HirDeclKind},
    hir_expr::{HirExpr, HirExprKind},
    hir_id::HirId,
    hir_node::HirNode,
    hir_stmt::{HirStmt, HirStmtKind},
};

use super::basic_block::BasicBlock;

pub struct CfgIr {
    blocks: Vec<BasicBlock>,
    register_stack: Vec<u8>,
    nodes_register: HashMap<HirId, u8>,
}

impl CfgIr {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            register_stack: Vec::new(),
        }
    }

    pub fn check(&mut self, declarations: &[HirDecl]) {
        for declaration in declarations {
            self.visit_declaration(declaration);
        }
    }

    fn visit_nodes(&mut self, nodes: &[HirNode]) {
        for node in nodes {
            self.visit_ast_node(node);
        }
    }

    fn visit_ast_node(&mut self, node: &HirNode) {
        match node {
            HirNode::Declaration(declaration) => self.visit_declaration(declaration),
            HirNode::Statement(statement) => self.visit_statement(statement),
        };
    }

    fn visit_declaration(&mut self, declaration: &HirDecl) {
        match &declaration.kind {
            HirDeclKind::Variable { right, .. } => {}

            HirDeclKind::Function { body, .. } => {}
            HirDeclKind::Struct { fields } => {}
            HirDeclKind::Parameter { .. } => {}
            HirDeclKind::Field { .. } => {}
        }
    }

    fn visit_statement(&mut self, statement: &HirStmt) {
        match &statement.kind {
            HirStmtKind::Expression(expression) => {
                self.visit_expression(expression);
            }
            HirStmtKind::Print(expression) => {
                self.visit_expression(expression);
            }
            HirStmtKind::Block(nodes) => {
                self.visit_nodes(nodes);
            }
            HirStmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {}
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {}
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {}
        };
    }

    fn visit_expression(&self, expression: &HirExpr) {
        match &expression.kind {
            HirExprKind::Assign(left, right) => {}
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {}
            HirExprKind::Unary { right, operator } => {}
            HirExprKind::FunctionCall { callee, arguments } => {}
            HirExprKind::FunctionRef(id) => {}
            HirExprKind::VariableRef(id) => {}
            HirExprKind::StringLiteral(..) => {}
            HirExprKind::BooleanLiteral(..) => {}
            HirExprKind::NumberLiteral(..) => {}
        };
    }
}
