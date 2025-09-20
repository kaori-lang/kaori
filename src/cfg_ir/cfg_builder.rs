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
    virtual_reg_inst::VirtualRegInst,
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
                    let bb_id = self.basic_block_stream.create_bb();

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
                let src1 = self.visit_expression(right);
                let dest = self.allocate_register();

                self.nodes_register.insert(declaration.id, dest);

                let instruction = VirtualRegInst::Move { dest, src1 };

                self.basic_block_stream.emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                for parameter in parameters {
                    let register = self.allocate_register();
                    self.nodes_register.insert(parameter.id, register);
                }

                let function_bb = *self.nodes_bb.get(&declaration.id).unwrap();

                self.basic_block_stream.current_bb = function_bb;

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

                let condition_bb = self.basic_block_stream.current_bb;
                let then_bb = self.basic_block_stream.create_bb();
                let else_bb = self.basic_block_stream.create_bb();
                let terminator_block = self.basic_block_stream.create_bb();

                self.basic_block_stream.set_current_bb(then_bb);
                self.visit_statement(then_branch);

                if let Some(branch) = else_branch {
                    self.basic_block_stream.set_current_bb(else_bb);
                    self.visit_statement(branch);
                }

                self.basic_block_stream.get_bb(condition_bb).terminator = Terminator::Branch {
                    r#true: then_bb,
                    r#false: else_bb,
                };

                self.basic_block_stream.get_bb(then_bb).terminator =
                    Terminator::Goto(terminator_block);

                self.basic_block_stream.get_bb(else_bb).terminator =
                    Terminator::Goto(terminator_block);

                self.basic_block_stream.set_current_bb(terminator_block);
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init);
                }
                let previous_bb = self.basic_block_stream.current_bb;
                let condition_bb = self.basic_block_stream.create_bb();
                let block_bb = self.basic_block_stream.create_bb();
                let terminator_bb = self.basic_block_stream.create_bb();

                self.basic_block_stream.current_bb = condition_bb;
                self.visit_expression(condition);

                self.basic_block_stream.current_bb = block_bb;
                self.visit_statement(block);

                self.basic_block_stream.get_bb(previous_bb).terminator =
                    Terminator::Goto(condition_bb);

                self.basic_block_stream.get_bb(condition_bb).terminator = Terminator::Branch {
                    r#true: block_bb,
                    r#false: terminator_bb,
                };

                self.basic_block_stream.get_bb(block_bb).terminator =
                    Terminator::Goto(condition_bb);

                self.basic_block_stream.current_bb = terminator_bb;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let source = self.visit_expression(expr);

                    let instruction = VirtualRegInst::Return { source };

                    self.basic_block_stream.emit_instruction(instruction);
                }
                let current_bb = self.basic_block_stream.current_bb;

                self.basic_block_stream.get_bb(current_bb).terminator = Terminator::Return;
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> usize {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src1 = self.visit_expression(right);

                let instruction = VirtualRegInst::Move { dest, src1 };

                self.basic_block_stream.emit_instruction(instruction);

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
                    BinaryOpKind::Add => VirtualRegInst::Add { dest, src1, src2 },
                    BinaryOpKind::Subtract => VirtualRegInst::Subtract { dest, src1, src2 },
                    BinaryOpKind::Multiply => VirtualRegInst::Multiply { dest, src1, src2 },
                    BinaryOpKind::Divide => VirtualRegInst::Divide { dest, src1, src2 },
                    BinaryOpKind::Modulo => VirtualRegInst::Modulo { dest, src1, src2 },

                    BinaryOpKind::Equal => VirtualRegInst::Equal { dest, src1, src2 },
                    BinaryOpKind::NotEqual => VirtualRegInst::NotEqual { dest, src1, src2 },
                    BinaryOpKind::Greater => VirtualRegInst::Greater { dest, src1, src2 },
                    BinaryOpKind::GreaterEqual => VirtualRegInst::GreaterEqual { dest, src1, src2 },
                    BinaryOpKind::Less => VirtualRegInst::Less { dest, src1, src2 },
                    BinaryOpKind::LessEqual => VirtualRegInst::LessEqual { dest, src1, src2 },

                    // gonna be changed
                    BinaryOpKind::And => VirtualRegInst::And { dest, src1, src2 },
                    BinaryOpKind::Or => VirtualRegInst::Or { dest, src1, src2 },
                };

                self.basic_block_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::Unary { right, operator } => {
                let src1 = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => VirtualRegInst::Negate { dest, src1 },
                    UnaryOpKind::Not => VirtualRegInst::Not { dest, src1 },
                };

                self.basic_block_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register();

                let call_instruction = VirtualRegInst::Call;

                self.basic_block_stream.emit_instruction(call_instruction);

                for (dest, argument) in arguments.iter().enumerate() {
                    let src1 = self.visit_expression(argument);

                    let instruction = VirtualRegInst::Move { dest, src1 };

                    self.basic_block_stream.emit_instruction(instruction);
                }

                let callee_register = self.visit_expression(callee);

                dest
            }

            HirExprKind::VariableRef(id) => *self.nodes_register.get(id).unwrap(),
            HirExprKind::FunctionRef(id) => {
                let dest = self.allocate_register();

                let value = *self.nodes_bb.get(id).unwrap();

                let instruction = VirtualRegInst::FunctionConst { dest, value };

                self.basic_block_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::StringLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = VirtualRegInst::StringConst {
                    dest,
                    value: value.to_owned(),
                };

                self.basic_block_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::BooleanLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = VirtualRegInst::BooleanConst {
                    dest,
                    value: *value,
                };

                self.basic_block_stream.emit_instruction(instruction);

                dest
            }
            HirExprKind::NumberLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = VirtualRegInst::NumberConst {
                    dest,
                    value: *value,
                };

                self.basic_block_stream.emit_instruction(instruction);

                dest
            }
        }
    }
}
