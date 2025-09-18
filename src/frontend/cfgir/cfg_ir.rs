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
    basic_block::Terminator, basic_block_stream::BasicBlockStream, block_id::BlockId,
    cfg_instruction::CfgInstruction, register::Register, register_allocator::RegisterAllocator,
};

pub struct CfgIr {
    basic_block_stream: BasicBlockStream,
    register_allocator: RegisterAllocator,
    nodes_register: HashMap<HirId, Register>,
    nodes_bb: HashMap<HirId, BlockId>,
}

impl CfgIr {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            basic_block_stream: BasicBlockStream::default(),
            register_allocator: RegisterAllocator::new(),
            nodes_register: HashMap::new(),
            nodes_bb: HashMap::new(),
        }
    }

    pub fn check(&mut self, declarations: &[HirDecl]) {
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
                let dst = self.register_allocator.alloc_register();

                let instruction = CfgInstruction::LoadLocal { dst, r1 };

                self.basic_block_stream.emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                for parameter in parameters {
                    let register = self.register_allocator.alloc_register();
                    self.nodes_register.insert(parameter.id, register);
                }

                let function_bb = *self.nodes_bb.get(&declaration.id).unwrap();

                self.basic_block_stream.current_basic_block = function_bb;

                for node in body {
                    self.visit_ast_node(node);
                }

                self.register_allocator.free_all_registers();
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

                self.basic_block_stream.get_basic_block(then_bb).terminator =
                    Terminator::Jump(terminator_block);

                self.basic_block_stream.get_basic_block(else_bb).terminator =
                    Terminator::Jump(terminator_block);

                self.basic_block_stream
                    .get_basic_block(condition_bb)
                    .terminator = Terminator::Conditional { then_bb, else_bb };

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

                let condition_bb = self.basic_block_stream.create_basic_block();
                let block_bb = self.basic_block_stream.create_basic_block();

                self.basic_block_stream.current_basic_block = condition_bb;
                self.visit_expression(condition);

                self.basic_block_stream.current_basic_block = block_bb;
                self.visit_statement(block);

                let terminator_bb = self.basic_block_stream.create_basic_block();

                self.basic_block_stream
                    .get_basic_block(condition_bb)
                    .terminator = Terminator::JumpIfFalse(terminator_bb);

                self.basic_block_stream.get_basic_block(block_bb).terminator =
                    Terminator::Jump(condition_bb);

                self.basic_block_stream.current_basic_block = terminator_bb;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let r1 = self.visit_expression(expr);

                    let instruction = CfgInstruction::Return { r1 };

                    self.basic_block_stream.emit_instruction(instruction);
                }
                let current_bb = self.basic_block_stream.current_basic_block;

                self.basic_block_stream
                    .get_basic_block(current_bb)
                    .terminator = Terminator::Return;

                let bb = self.basic_block_stream.create_basic_block();

                self.basic_block_stream.current_basic_block = bb;
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> Register {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let dst = self.visit_expression(left);
                let r1 = self.visit_expression(right);

                let instruction = CfgInstruction::StoreLocal { dst, r1 };
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
                let dst = self.register_allocator.alloc_register();

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

                    // gonna be changed
                    BinaryOpKind::And => CfgInstruction::And { dst, r1, r2 },
                    BinaryOpKind::Or => CfgInstruction::Or { dst, r1, r2 },
                };

                self.basic_block_stream.emit_instruction(instruction);

                self.register_allocator.free_register(r1);
                self.register_allocator.free_register(r2);

                dst
            }
            HirExprKind::Unary { right, operator } => {
                let r1 = self.visit_expression(right);
                let dst = self.register_allocator.alloc_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => CfgInstruction::Negate { dst, r1 },
                    UnaryOpKind::Not => CfgInstruction::Not { dst, r1 },
                };

                self.basic_block_stream.emit_instruction(instruction);
                self.register_allocator.free_register(r1);

                dst
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dst = self.register_allocator.alloc_register();

                // Number of registers being used by current frame
                let registers = self.register_allocator.max_allocated_register() + 1;
                let call_instruction = CfgInstruction::Call { registers };

                self.basic_block_stream.emit_instruction(call_instruction);

                for (dst, argument) in arguments.iter().enumerate() {
                    let r1 = self.visit_expression(argument);
                    let dst = Register::new(dst as u8);

                    let instruction = CfgInstruction::StoreLocal { dst, r1 };

                    self.basic_block_stream.emit_instruction(instruction);
                }

                let callee_register = self.visit_expression(callee);

                dst
            }

            HirExprKind::VariableRef(id) => {
                let r1 = *self.nodes_register.get(id).unwrap();
                let dst = self.register_allocator.alloc_register();

                let instruction = CfgInstruction::LoadLocal { dst, r1 };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::FunctionRef(id) => {
                let dst = self.register_allocator.alloc_register();

                let value = *self.nodes_bb.get(id).unwrap();

                let instruction = CfgInstruction::FunctionConst { dst, value };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::StringLiteral(value) => {
                let dst = self.register_allocator.alloc_register();

                let instruction = CfgInstruction::StringConst {
                    dst,
                    value: value.to_owned(),
                };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::BooleanLiteral(value) => {
                let dst = self.register_allocator.alloc_register();

                let instruction = CfgInstruction::BooleanConst { dst, value: *value };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
            HirExprKind::NumberLiteral(value) => {
                let dst = self.register_allocator.alloc_register();

                let instruction = CfgInstruction::NumberConst { dst, value: *value };

                self.basic_block_stream.emit_instruction(instruction);

                dst
            }
        }
    }
}
