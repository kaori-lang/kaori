use std::collections::HashMap;

use crate::frontend::semantic::{
    hir_decl::{HirDecl, HirDeclKind},
    hir_expr::{HirExpr, HirExprKind},
    hir_id::HirId,
    hir_node::HirNode,
    hir_stmt::{HirStmt, HirStmtKind},
};

use super::{basic_block::BasicBlock, cfg_instruction::CfgInstruction};

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

    pub fn exit_function(&mut self) {
        self.register_stack.pop();
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
            HirDeclKind::Variable { right } => {
                let r1 = self.visit_expression(right);
                let dst = self.create_register();

                let instruction = CfgInstruction::LoadLocal { dst, r1 };

                self.emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                self.enter_function();

                for (register, parameter) in parameters.iter().enumerate() {
                    self.nodes_register.insert(parameter.id, register as u8);
                }

                for node in body {
                    self.visit_ast_node(node);
                }

                self.exit_function();
            }
            HirDeclKind::Struct { fields } => {}
            HirDeclKind::Parameter => {}
            HirDeclKind::Field => {}
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
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    self.visit_expression(expr);
                }
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> u8 {
        let register = match &expression.kind {
            HirExprKind::Assign(left, right) => {
                let dst = self.visit_expression(left);
                let r1 = self.visit_expression(right);

                let instruction = CfgInstruction::StoreLocal { dst, r1 };
                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let r1 = self.visit_expression(left);
                let r2 = self.visit_expression(right);

                r1
            }
            HirExprKind::Unary { right, operator } => {
                let r1 = self.visit_expression(right);

                r1
            }
            HirExprKind::FunctionCall { callee, arguments } => 1,
            HirExprKind::FunctionRef(id) => 1,
            HirExprKind::VariableRef(id) => {
                let r1 = *self.nodes_register.get(id).unwrap();
                let dst = self.create_register();
                let instruction = CfgInstruction::LoadLocal { dst, r1 };

                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::StringLiteral(value) => {
                let dst = self.create_register();

                let instruction = CfgInstruction::StringConst {
                    dst,
                    value: value.to_owned(),
                };

                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::BooleanLiteral(value) => {
                let dst = self.create_register();

                let instruction = CfgInstruction::BooleanConst { dst, value: *value };

                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::NumberLiteral(value) => {
                let dst = self.create_register();

                let instruction = CfgInstruction::NumberConst { dst, value: *value };

                self.emit_instruction(instruction);

                dst
            }
        };

        register
    }
}
