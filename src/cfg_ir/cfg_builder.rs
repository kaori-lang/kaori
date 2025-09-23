use std::collections::HashMap;

use crate::{
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
    basic_block::Terminator, block_id::BlockId, cfg_instruction::CfgInstructionKind,
    cfg_stream::CfgStream,
};

pub struct CfgBuilder<'a> {
    cfg_stream: &'a mut CfgStream,
    register: usize,
    nodes_register: HashMap<HirId, usize>,
    nodes_block: HashMap<HirId, BlockId>,
}

impl<'a> CfgBuilder<'a> {
    pub fn new(cfg_stream: &'a mut CfgStream) -> Self {
        Self {
            cfg_stream,
            register: 0,
            nodes_register: HashMap::new(),
            nodes_block: HashMap::new(),
        }
    }

    fn allocate_register(&mut self) -> usize {
        let register = self.register;

        self.register += 1;

        register
    }

    fn free_all_registers(&mut self) {
        self.register = 0;
    }

    pub fn build_ir(&mut self, declarations: &[HirDecl]) {
        for declaration in declarations {
            let cfg_root = self.cfg_stream.create_cfg();

            self.nodes_block.insert(declaration.id, cfg_root);
        }

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
                let src = self.visit_expression(right);
                let dest = self.allocate_register();

                self.nodes_register.insert(declaration.id, dest);

                let instruction = CfgInstructionKind::Move { dest, src };

                self.cfg_stream.emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                self.cfg_stream.current_bb = *self.nodes_block.get(&declaration.id).unwrap();

                for parameter in parameters {
                    let register = self.allocate_register();
                    self.nodes_register.insert(parameter.id, register);
                }

                for node in body {
                    self.visit_ast_node(node);
                }

                let last_bb = self.cfg_stream.current_bb;

                self.cfg_stream.get_bb(last_bb).terminator = Terminator::Return;

                self.free_all_registers();
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
                self.visit_expression(condition);

                let condition_bb = self.cfg_stream.current_bb;
                let then_bb = self.cfg_stream.create_bb();
                let else_bb = self.cfg_stream.create_bb();
                let terminator_block = self.cfg_stream.create_bb();

                self.cfg_stream.current_bb = then_bb;
                self.visit_statement(then_branch);

                if let Some(branch) = else_branch {
                    self.cfg_stream.current_bb = else_bb;
                    self.visit_statement(branch);
                }

                self.cfg_stream.get_bb(condition_bb).terminator = Terminator::Branch {
                    r#true: then_bb,
                    r#false: else_bb,
                };

                self.cfg_stream.get_bb(then_bb).terminator = Terminator::Goto(terminator_block);

                self.cfg_stream.get_bb(else_bb).terminator = Terminator::Goto(terminator_block);

                self.cfg_stream.current_bb = terminator_block;
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init);
                }
                let previous_bb = self.cfg_stream.current_bb;
                let condition_bb = self.cfg_stream.create_bb();
                let block_bb = self.cfg_stream.create_bb();
                let terminator_bb = self.cfg_stream.create_bb();

                self.cfg_stream.current_bb = condition_bb;
                self.visit_expression(condition);

                self.cfg_stream.current_bb = block_bb;
                self.visit_statement(block);

                self.cfg_stream.get_bb(previous_bb).terminator = Terminator::Goto(condition_bb);

                self.cfg_stream.get_bb(condition_bb).terminator = Terminator::Branch {
                    r#true: block_bb,
                    r#false: terminator_bb,
                };

                self.cfg_stream.get_bb(block_bb).terminator = Terminator::Goto(condition_bb);

                self.cfg_stream.current_bb = terminator_bb;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let src = self.visit_expression(expr);

                    let instruction = CfgInstructionKind::Return { src };

                    self.cfg_stream.emit_instruction(instruction);
                }
                let current_bb = self.cfg_stream.current_bb;

                self.cfg_stream.get_bb(current_bb).terminator = Terminator::Return;
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> usize {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src = self.visit_expression(right);

                let instruction = CfgInstructionKind::Move { dest, src };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.visit_expression(left);
                let src2 = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    BinaryOpKind::Add => CfgInstructionKind::Add { dest, src1, src2 },
                    BinaryOpKind::Subtract => CfgInstructionKind::Subtract { dest, src1, src2 },
                    BinaryOpKind::Multiply => CfgInstructionKind::Multiply { dest, src1, src2 },
                    BinaryOpKind::Divide => CfgInstructionKind::Divide { dest, src1, src2 },
                    BinaryOpKind::Modulo => CfgInstructionKind::Modulo { dest, src1, src2 },

                    BinaryOpKind::Equal => CfgInstructionKind::Equal { dest, src1, src2 },
                    BinaryOpKind::NotEqual => CfgInstructionKind::NotEqual { dest, src1, src2 },
                    BinaryOpKind::Greater => CfgInstructionKind::Greater { dest, src1, src2 },
                    BinaryOpKind::GreaterEqual => {
                        CfgInstructionKind::GreaterEqual { dest, src1, src2 }
                    }
                    BinaryOpKind::Less => CfgInstructionKind::Less { dest, src1, src2 },
                    BinaryOpKind::LessEqual => CfgInstructionKind::LessEqual { dest, src1, src2 },

                    // gonna be changed
                    BinaryOpKind::And => CfgInstructionKind::And { dest, src1, src2 },
                    BinaryOpKind::Or => CfgInstructionKind::Or { dest, src1, src2 },
                };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => CfgInstructionKind::Negate { dest, src },
                    UnaryOpKind::Not => CfgInstructionKind::Not { dest, src },
                };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register();

                let call_instruction = CfgInstructionKind::Call;

                self.cfg_stream.emit_instruction(call_instruction);

                for (dest, argument) in arguments.iter().enumerate() {
                    let src = self.visit_expression(argument);

                    let instruction = CfgInstructionKind::Move { dest, src };

                    self.cfg_stream.emit_instruction(instruction);
                }

                let callee_register = self.visit_expression(callee);

                dest
            }

            HirExprKind::VariableRef(id) => *self.nodes_register.get(id).unwrap(),
            HirExprKind::FunctionRef(id) => {
                let dest = self.allocate_register();

                let value = *self.nodes_block.get(id).unwrap();

                let instruction = CfgInstructionKind::FunctionConst { dest, value };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::StringLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = CfgInstructionKind::StringConst {
                    dest,
                    value: value.to_owned(),
                };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::BooleanLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = CfgInstructionKind::BooleanConst {
                    dest,
                    value: *value,
                };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::NumberLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = CfgInstructionKind::NumberConst {
                    dest,
                    value: *value,
                };

                self.cfg_stream.emit_instruction(instruction);

                dest
            }
        }
    }
}
