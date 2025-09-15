use std::collections::HashMap;

use crate::frontend::{
    semantic::{
        hir_decl::{HirDecl, HirDeclKind},
        hir_expr::{HirExpr, HirExprKind},
        hir_id::HirId,
        hir_node::HirNode,
        hir_stmt::{HirStmt, HirStmtKind},
    },
    syntax::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
};

use super::{
    basic_block::{BasicBlock, Terminator},
    cfg_instruction::CfgInstruction,
    register::Register,
};

pub struct CfgIr {
    basic_blocks: Vec<BasicBlock>,
    current_basic_block: usize,
    register_stack: Vec<u8>,
    nodes_register: HashMap<HirId, Register>,
}

impl CfgIr {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            basic_blocks: Vec::new(),
            current_basic_block: 0,
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

    pub fn create_register(&mut self) -> Register {
        let register = *self.register_stack.last().unwrap();

        Register::new(register)
    }

    pub fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let block = &mut self.basic_blocks[self.current_basic_block];

        block.add_instruction(instruction);
    }

    pub fn create_basic_block(&mut self) -> usize {
        let block = BasicBlock::default();
        let index = self.basic_blocks.len();
        self.basic_blocks.push(block);

        index
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

                for (index, parameter) in parameters.iter().enumerate() {
                    let register = Register::new(index as u8);
                    self.nodes_register.insert(parameter.id, register);
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
            } => {
                let condition_block = self.basic_blocks.len() - 1;

                self.visit_expression(condition);

                let then_block = self.create_basic_block();

                let else_block = self.create_basic_block();

                self.current_basic_block = then_block;

                self.visit_statement(then_branch);

                if let Some(branch) = else_branch {
                    self.current_basic_block = else_block;
                    self.visit_statement(branch);
                }

                let terminator_block = self.create_basic_block();

                self.basic_blocks.get_mut(then_block).unwrap().terminator =
                    Terminator::Jump(terminator_block);

                self.basic_blocks.get_mut(else_block).unwrap().terminator =
                    Terminator::Jump(terminator_block);

                self.basic_blocks
                    .get_mut(condition_block)
                    .unwrap()
                    .terminator = Terminator::Conditional {
                    then_block,
                    else_block,
                };
            }
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

    fn visit_expression(&mut self, expression: &HirExpr) -> Register {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
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
                let dst = self.create_register();

                let instruction = match operator.kind {
                    BinaryOpKind::Add => CfgInstruction::Add { dst, r1, r2 },
                    BinaryOpKind::Subtract => CfgInstruction::Subtract { dst, r1, r2 },
                    BinaryOpKind::Multiply => CfgInstruction::Multiply { dst, r1, r2 },
                    BinaryOpKind::Divide => CfgInstruction::Divide { dst, r1, r2 },
                    BinaryOpKind::Modulo => CfgInstruction::Modulo { dst, r1, r2 },

                    BinaryOpKind::Equal => CfgInstruction::Equal { dst, r1, r2 },
                    BinaryOpKind::NotEqual => CfgInstruction::NotEqual { dst, r1, r2 },
                    BinaryOpKind::Greater => CfgInstruction::Greater { dst, r1, r2 },
                    BinaryOpKind::GreaterEqual => CfgInstruction::GreaterEqual { dst, r1, r2 },
                    BinaryOpKind::Less => CfgInstruction::Less { dst, r1, r2 },
                    BinaryOpKind::LessEqual => CfgInstruction::LessEqual { dst, r1, r2 },

                    // be changed
                    BinaryOpKind::And => CfgInstruction::And { dst, r1, r2 },
                    BinaryOpKind::Or => CfgInstruction::Or { dst, r1, r2 },
                };

                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::Unary { right, operator } => {
                let r1 = self.visit_expression(right);
                let dst = self.create_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => CfgInstruction::Negate { dst, r1 },
                    UnaryOpKind::Not => CfgInstruction::Not { dst, r1 },
                };

                self.emit_instruction(instruction);

                dst
            }
            HirExprKind::FunctionCall { callee, arguments } => Register::new(0),
            HirExprKind::FunctionRef(id) => Register::new(0),
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
        }
    }
}
