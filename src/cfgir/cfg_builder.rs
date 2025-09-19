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
    basic_block::Terminator, basic_block_stream::BasicBlockStream, block_id::BlockId,
    register::Register, virtual_reg_inst::VirtualRegInst,
};

pub struct CfgBuilder<'a> {
    basic_block_stream: &'a mut BasicBlockStream,
    nodes_register: HashMap<HirId, usize>,
    nodes_bb: HashMap<HirId, BlockId>,
    register: usize,
}

impl<'a> CfgBuilder<'a> {
    pub fn new(basic_block_stream: &'a mut BasicBlockStream) -> Self {
        Self {
            basic_block_stream,
            register: 0,
            nodes_register: HashMap::new(),
            nodes_bb: HashMap::new(),
        }
    }

    pub fn build_ir(&mut self, declarations: &[HirDecl]) {
        for declaration in declarations {
            match &declaration.kind {
                HirDeclKind::Function { .. } => {
                    let bb_id = self.basic_block_stream.create_basic_block();

                    self.nodes_bb.insert(declaration.id, bb_id);
                }
                HirDeclKind::Struct { fields } => {}
                _ => {}
            }
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

    fn allocate_register(&mut self) -> usize {
        let register = self.register;

        self.register += 1;

        register
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
                let dst = self.allocate_register();

                self.nodes_register.insert(declaration.id, dst);

                let instruction = VirtualRegInst::LoadLocal { dst, r1 };

                self.basic_block_stream.emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                for parameter in parameters {
                    let register = self.allocate_register();
                    self.nodes_register.insert(parameter.id, register);
                }

                let function_bb = *self.nodes_bb.get(&declaration.id).unwrap();

                self.basic_block_stream.current_basic_block = function_bb;

                for node in body {
                    self.visit_ast_node(node);
                }
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

                let condition_bb = self.basic_block_stream.current_basic_block;
                let then_bb = self.basic_block_stream.create_basic_block();
                let else_bb = self.basic_block_stream.create_basic_block();

                self.basic_block_stream.set_current(then_bb);
                self.visit_statement(then_branch);

                if let Some(branch) = else_branch {
                    self.basic_block_stream.set_current(else_bb);
                    self.visit_statement(branch);
                }

                let terminator_block = self.basic_block_stream.create_basic_block();

                self.basic_block_stream
                    .get_basic_block(condition_bb)
                    .terminator = Terminator::Branch {
                    r#true: then_bb,
                    r#false: else_bb,
                };

                self.basic_block_stream.get_basic_block(then_bb).terminator =
                    Terminator::Goto(terminator_block);

                self.basic_block_stream.get_basic_block(else_bb).terminator =
                    Terminator::Goto(terminator_block);

                self.basic_block_stream.set_current(terminator_block);
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init);
                }

                self.visit_expression(condition);

                let condition_bb = self.basic_block_stream.current_basic_block;
                let block_bb = self.basic_block_stream.create_basic_block();

                self.basic_block_stream.current_basic_block = block_bb;
                self.visit_statement(block);

                let terminator_bb = self.basic_block_stream.create_basic_block();

                self.basic_block_stream
                    .get_basic_block(condition_bb)
                    .terminator = Terminator::Branch {
                    r#true: block_bb,
                    r#false: terminator_bb,
                };

                self.basic_block_stream.get_basic_block(block_bb).terminator =
                    Terminator::Goto(condition_bb);

                self.basic_block_stream.current_basic_block = terminator_bb;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let r1 = self.visit_expression(expr);

                    let instruction = VirtualRegInst::Return { r1 };

                    self.basic_block_stream.emit_instruction(instruction);
                }
                let current_bb = self.basic_block_stream.current_basic_block;

                self.basic_block_stream
                    .get_basic_block(current_bb)
                    .terminator = Terminator::Return;
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> Register {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let dst = self.visit_expression(left);
                let r1 = self.visit_expression(right);

                let instruction = VirtualRegInst::StoreLocal { dst, r1 };
                self.basic_block_stream.emit_instruction(instruction);

                self.register_allocator.free_register(r1);

                dst
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let r1 = self.visit_expression(left);
                let r2 = self.visit_expression(right);
                let dst = self.allocate_register();

                let instruction = match operator.kind {
                    BinaryOpKind::Add => VirtualRegInst::Add { dst, r1, r2 },
                    BinaryOpKind::Subtract => VirtualRegInst::Subtract { dst, r1, r2 },
                    BinaryOpKind::Multiply => VirtualRegInst::Multiply { dst, r1, r2 },
                    BinaryOpKind::Divide => VirtualRegInst::Divide { dst, r1, r2 },
                    BinaryOpKind::Modulo => VirtualRegInst::Modulo { dst, r1, r2 },

                    BinaryOpKind::Equal => VirtualRegInst::Equal { dst, r1, r2 },
                    BinaryOpKind::NotEqual => VirtualRegInst::NotEqual { dst, r1, r2 },
                    BinaryOpKind::Greater => VirtualRegInst::Greater { dst, r1, r2 },
                    BinaryOpKind::GreaterEqual => VirtualRegInst::GreaterEqual { dst, r1, r2 },
                    BinaryOpKind::Less => VirtualRegInst::Less { dst, r1, r2 },
                    BinaryOpKind::LessEqual => VirtualRegInst::LessEqual { dst, r1, r2 },

                    // gonna be changed
                    BinaryOpKind::And => VirtualRegInst::And { dst, r1, r2 },
                    BinaryOpKind::Or => VirtualRegInst::Or { dst, r1, r2 },
                };

                self.basic_block_stream.emit_instruction(instruction);

                self.register_allocator.free_register(r1);
                self.register_allocator.free_register(r2);

                dst
            }
            HirExprKind::Unary { right, operator } => {
                let r1 = self.visit_expression(right);
                let dst = self.allocate_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => VirtualRegInst::Negate { dst, r1 },
                    UnaryOpKind::Not => VirtualRegInst::Not { dst, r1 },
                };

                self.basic_block_stream.emit_instruction(instruction);

                self.register_allocator.free_register(r1);

                dst
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dst = self.allocate_register();

                // Number of registers being used by current frame
                let registers = self.register_allocator.max_allocated_register() + 1;
                let call_instruction = VirtualRegInst::Call { registers };

                self.basic_block_stream.emit_instruction(call_instruction);

                for (dst, argument) in arguments.iter().enumerate() {
                    let r1 = self.visit_expression(argument);
                    let dst = Register::new(dst as u8);

                    let instruction = VirtualRegInst::StoreLocal { dst, r1 };

                    self.basic_block_stream.emit_instruction(instruction);
                }

                let callee_register = self.visit_expression(callee);

                dst
            }

            HirExprKind::VariableRef(id) => {
                let r1 = *self.nodes_register.get(id).unwrap();
                let dst = self.allocate_register();

                let instruction = VirtualRegInst::LoadLocal { dst, r1 };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::FunctionRef(id) => {
                let dst = self.allocate_register();

                let value = *self.nodes_bb.get(id).unwrap();

                let instruction = VirtualRegInst::FunctionConst { dst, value };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::StringLiteral(value) => {
                let dst = self.allocate_register();

                let instruction = VirtualRegInst::StringConst {
                    dst,
                    value: value.to_owned(),
                };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::BooleanLiteral(value) => {
                let dst = self.allocate_register();

                let instruction = VirtualRegInst::BooleanConst { dst, value: *value };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::NumberLiteral(value) => {
                let dst = self.allocate_register();

                let instruction = VirtualRegInst::NumberConst { dst, value: *value };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
        }
    }
}
