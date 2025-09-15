use std::collections::HashMap;

use crate::frontend::semantic::{
    hir_decl::{HirDecl, HirDeclKind},
    hir_expr::{HirExpr, HirExprKind},
    hir_id::HirId,
    hir_node::HirNode,
    hir_stmt::{HirStmt, HirStmtKind},
};

use super::basic_block::{BasicBlock, CfgInstruction};

pub struct CfgIr {
    blocks: Vec<BasicBlock>,
    register_stack: Vec<u8>,
    nodes_register: HashMap<HirId, u8>,
}

impl CfgIr {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            register_stack: vec![0],
            nodes_register: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        let register = self.register_stack.last().unwrap();
        self.register_stack.push(*register);
    }

    pub fn exit_scope(&mut self) {
        self.register_stack.pop();
    }

    pub fn enter_function(&mut self) {
        let register = 0;

        self.register_stack.push(register);
    }

    pub fn create_register(&mut self) -> u8 {
        let register = self.register_stack.last().unwrap();

        *register
    }

    pub fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let block = self.blocks.last_mut().unwrap();

        block.add_instruction(instruction);
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
            HirDeclKind::Variable { right, .. } => {
                let r1 = self.visit_expression(right);
                let 

            }

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

    fn visit_expression(&self, expression: &HirExpr) -> u8 {
        let register = match &expression.kind {
            HirExprKind::Assign(left, right) => {
                let dst = self.visit_expression(left);
                let r1 = self.visit_expression(right);

                dst
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {}
            HirExprKind::Unary { right, operator } => {}
            HirExprKind::FunctionCall { callee, arguments } => {}
            HirExprKind::FunctionRef(id) => {}
            HirExprKind::VariableRef(id) => {
                let r1 = *self.nodes_register.get(id).unwrap();
                let dst = self.create_register();
                let instruction = CfgInstruction::LoadLocal { dst, r1 };

                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::StringLiteral(..) => {}
            HirExprKind::BooleanLiteral(..) => {}
            HirExprKind::NumberLiteral(..) => {
                let register = self.create_register();
            }
        };

        register
    }
}
