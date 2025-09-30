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
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    operand::Variable,
};

pub struct CfgBuilder {
    pub cfg_ir: CfgIr,
    current_bb: BlockId,
    variable: usize,
    nodes_variable: HashMap<HirId, Variable>,
    nodes_block: HashMap<HirId, BlockId>,
}

impl CfgBuilder {
    pub fn new() -> Self {
        Self {
            cfg_ir: CfgIr::default(),
            current_bb: BlockId(0),
            variable: 0,
            nodes_variable: HashMap::new(),
            nodes_block: HashMap::new(),
        }
    }

    fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let id = self.current_bb;
        let basic_block = self.cfg_ir.basic_blocks.get_mut(id.0).unwrap();

        if let Terminator::Return { .. } = basic_block.terminator {
            return;
        }

        basic_block.instructions.push(instruction);
    }

    fn create_bb(&mut self) -> BlockId {
        let size = self.cfg_ir.basic_blocks.len();

        let id = BlockId(size);

        let basic_block = BasicBlock::new(id);

        self.cfg_ir.basic_blocks.push(basic_block);

        id
    }

    fn create_cfg(&mut self) -> BlockId {
        let basic_block = self.create_bb();

        self.cfg_ir.cfgs.push(basic_block);

        basic_block
    }

    fn set_terminator(&mut self, id: BlockId, terminator: Terminator) {
        self.cfg_ir.basic_blocks.get_mut(id.0).unwrap().terminator = terminator;
    }

    fn create_variable(&mut self) -> Variable {
        let variable = Variable(self.variable);

        self.variable += 1;

        variable
    }

    fn free_all_variables(&mut self) {
        self.variable = 0;
    }

    pub fn build_ir(&mut self, declarations: &[HirDecl]) {
        for declaration in declarations {
            let cfg_root = self.create_cfg();

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
                let dest = self.create_variable();

                self.nodes_variable.insert(declaration.id, dest);

                let instruction = CfgInstruction::move_(dest, src);

                self.emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                self.current_bb = *self.nodes_block.get(&declaration.id).unwrap();

                for parameter in parameters {
                    let variable = self.create_variable();
                    self.nodes_variable.insert(parameter.id, variable);
                }

                for node in body {
                    self.visit_ast_node(node);
                }

                let last_bb = self.current_bb;

                match self.cfg_ir.basic_blocks[self.current_bb.0].terminator {
                    Terminator::Return { .. } => {}
                    _ => self.set_terminator(last_bb, Terminator::Return { src: None }),
                }

                self.free_all_variables();
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
                let src = self.visit_expression(condition);

                let condition_bb = self.current_bb;
                let then_bb = self.create_bb();
                let else_bb = self.create_bb();
                let terminator_block = self.create_bb();

                self.current_bb = then_bb;
                self.visit_statement(then_branch);

                if let Some(branch) = else_branch {
                    self.current_bb = else_bb;
                    self.visit_statement(branch);
                }

                self.set_terminator(
                    condition_bb,
                    Terminator::Branch {
                        src: src.into(),
                        r#true: then_bb,
                        r#false: else_bb,
                    },
                );

                self.set_terminator(then_bb, Terminator::Goto(terminator_block));

                self.set_terminator(else_bb, Terminator::Goto(terminator_block));

                self.current_bb = terminator_block;
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init);
                }
                let previous_bb = self.current_bb;
                let condition_bb = self.create_bb();
                let block_bb = self.create_bb();
                let terminator_bb = self.create_bb();

                self.current_bb = condition_bb;
                let src = self.visit_expression(condition);

                self.current_bb = block_bb;
                self.visit_statement(block);

                self.set_terminator(previous_bb, Terminator::Goto(condition_bb));

                self.set_terminator(
                    condition_bb,
                    Terminator::Branch {
                        src: src.into(),
                        r#true: block_bb,
                        r#false: terminator_bb,
                    },
                );

                self.set_terminator(block_bb, Terminator::Goto(condition_bb));

                self.current_bb = terminator_bb;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let src = self.visit_expression(expr).into();

                    self.set_terminator(self.current_bb, Terminator::Return { src: Some(src) });
                } else {
                    self.set_terminator(self.current_bb, Terminator::Return { src: None });
                }
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> Variable {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src = self.visit_expression(right);

                let instruction = CfgInstruction::move_(dest, src);

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.visit_expression(left);
                let src2 = self.visit_expression(right);
                let dest = self.create_variable();

                let instruction = match operator.kind {
                    BinaryOpKind::Add => CfgInstruction::add(dest, src1, src2),
                    BinaryOpKind::Subtract => CfgInstruction::subtract(dest, src1, src2),
                    BinaryOpKind::Multiply => CfgInstruction::multiply(dest, src1, src2),
                    BinaryOpKind::Divide => CfgInstruction::divide(dest, src1, src2),
                    BinaryOpKind::Modulo => CfgInstruction::modulo(dest, src1, src2),

                    BinaryOpKind::Equal => CfgInstruction::equal(dest, src1, src2),
                    BinaryOpKind::NotEqual => CfgInstruction::not_equal(dest, src1, src2),
                    BinaryOpKind::Greater => CfgInstruction::greater(dest, src1, src2),
                    BinaryOpKind::GreaterEqual => CfgInstruction::greater_equal(dest, src1, src2),

                    BinaryOpKind::Less => CfgInstruction::less(dest, src1, src2),
                    BinaryOpKind::LessEqual => CfgInstruction::less_equal(dest, src1, src2),

                    BinaryOpKind::And => CfgInstruction::and(dest, src1, src2),
                    BinaryOpKind::Or => CfgInstruction::or(dest, src1, src2),
                };

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let dest = self.create_variable();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => CfgInstruction::negate(dest, src),
                    UnaryOpKind::Not => CfgInstruction::not(dest, src),
                };

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dest = self.create_variable();

                let call_instruction = CfgInstruction::Call;

                self.emit_instruction(call_instruction);

                for (dest, argument) in arguments.iter().enumerate() {
                    let src = self.visit_expression(argument);

                    /*   let instruction = CfgInstruction::Move { dest, src };

                    self.emit_instruction(instruction); */
                }

                let callee_variable = self.visit_expression(callee);

                dest
            }

            HirExprKind::VariableRef(id) => *self.nodes_variable.get(id).unwrap(),
            HirExprKind::FunctionRef(id) => {
                let dest = self.create_variable();

                let value = *self.nodes_block.get(id).unwrap();

                let instruction = CfgInstruction::function_const(dest, value);

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::StringLiteral(value) => {
                let dest = self.create_variable();

                let instruction = CfgInstruction::string_const(dest, value);

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::BooleanLiteral(value) => {
                let dest = self.create_variable();

                let instruction = CfgInstruction::boolean_const(dest, *value);

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::NumberLiteral(value) => {
                let dest = self.create_variable();

                let instruction = CfgInstruction::number_const(dest, *value);

                self.emit_instruction(instruction);

                dest
            }
        }
    }
}
