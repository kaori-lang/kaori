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
    active_loops::ActiveLoops,
    basic_block::{BasicBlock, BlockId, Terminator},
    cfg_instruction::CfgInstruction,
    cfg_ir::CfgIr,
    operand::Variable,
};

pub struct CfgBuilder {
    pub cfg_ir: CfgIr,
    current_bb: BlockId,
    variable: isize,
    nodes_variable: HashMap<HirId, Variable>,
    nodes_block: HashMap<HirId, BlockId>,
    active_loops: ActiveLoops,
}

impl Default for CfgBuilder {
    fn default() -> Self {
        Self {
            cfg_ir: CfgIr::default(),
            current_bb: BlockId(0),
            variable: 0,
            nodes_variable: HashMap::new(),
            nodes_block: HashMap::new(),
            active_loops: ActiveLoops::default(),
        }
    }
}

impl CfgBuilder {
    fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let basic_block = self.cfg_ir.basic_blocks.get_mut(self.current_bb.0).unwrap();

        let Terminator::None = basic_block.terminator else {
            return;
        };

        basic_block.instructions.push(instruction);
    }

    fn set_terminator(&mut self, terminator: Terminator) {
        let basic_block = self.cfg_ir.basic_blocks.get_mut(self.current_bb.0).unwrap();

        let Terminator::None = basic_block.terminator else {
            return;
        };

        basic_block.terminator = terminator;
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

    fn create_variable(&mut self) -> Variable {
        let variable = Variable(self.variable);

        self.variable += 1;

        variable
    }

    fn free_variables(&mut self) {
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

                match self.cfg_ir.basic_blocks[self.current_bb.0].terminator {
                    Terminator::Return { .. } => {}
                    _ => self.set_terminator(Terminator::Return { src: None }),
                }

                self.free_variables();
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
                let src = self.visit_expression(expression);

                let instruction = CfgInstruction::print(src);

                self.emit_instruction(instruction);
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

                let then_bb = self.create_bb();
                let else_bb = self.create_bb();
                let terminator_block = self.create_bb();

                self.set_terminator(Terminator::Branch {
                    src: src.into(),
                    r#true: then_bb,
                    r#false: else_bb,
                });

                self.current_bb = then_bb;
                self.visit_statement(then_branch);
                self.set_terminator(Terminator::Goto(terminator_block));

                self.current_bb = else_bb;
                if let Some(branch) = else_branch {
                    self.visit_statement(branch);
                }
                self.set_terminator(Terminator::Goto(terminator_block));

                self.current_bb = terminator_block;
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init);
                }

                let condition_bb = self.create_bb();
                let block_bb = self.create_bb();
                let terminator_bb = self.create_bb();
                let increment_bb = self.create_bb();

                self.set_terminator(Terminator::Goto(condition_bb));

                self.current_bb = condition_bb;
                let src = self.visit_expression(condition);
                self.set_terminator(Terminator::Branch {
                    src: src.into(),
                    r#true: block_bb,
                    r#false: terminator_bb,
                });

                self.current_bb = block_bb;
                self.active_loops.push(increment_bb, terminator_bb);
                self.visit_statement(block);
                self.active_loops.pop();
                self.set_terminator(Terminator::Goto(increment_bb));

                self.current_bb = increment_bb;
                if let Some(increment) = increment {
                    self.visit_statement(increment);
                }
                self.set_terminator(Terminator::Goto(condition_bb));

                self.current_bb = terminator_bb;
            }
            HirStmtKind::Break => {
                let label = self.active_loops.top();

                self.set_terminator(Terminator::Goto(label.terminator_bb));
            }
            HirStmtKind::Continue => {
                let label = self.active_loops.top();

                self.set_terminator(Terminator::Goto(label.increment_bb));
            }
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let src = self.visit_expression(expr).into();

                    self.set_terminator(Terminator::Return { src: Some(src) });
                } else {
                    self.set_terminator(Terminator::Return { src: None });
                }
            }
        };
    }

    fn visit_logical_or(&mut self, left: &HirExpr, right: &HirExpr) -> Variable {
        let dest = self.create_variable();

        let src1 = self.visit_expression(left);

        self.emit_instruction(CfgInstruction::move_(dest, src1));

        let src2_bb = self.create_bb();
        let terminator = self.create_bb();

        self.set_terminator(Terminator::Branch {
            src: dest.into(),
            r#true: terminator,
            r#false: src2_bb,
        });

        self.current_bb = src2_bb;

        let src2 = self.visit_expression(right);

        self.emit_instruction(CfgInstruction::move_(dest, src2));

        self.set_terminator(Terminator::Goto(terminator));

        self.current_bb = terminator;

        dest
    }

    fn visit_logical_and(&mut self, left: &HirExpr, right: &HirExpr) -> Variable {
        let dest = self.create_variable();

        let src1 = self.visit_expression(left);

        self.emit_instruction(CfgInstruction::move_(dest, src1));

        let src2_bb = self.create_bb();
        let terminator = self.create_bb();

        self.set_terminator(Terminator::Branch {
            src: dest.into(),
            r#true: src2_bb,
            r#false: terminator,
        });

        self.current_bb = src2_bb;

        let src2 = self.visit_expression(right);
        self.emit_instruction(CfgInstruction::move_(dest, src2));
        self.set_terminator(Terminator::Goto(terminator));

        self.current_bb = terminator;

        dest
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
            } => match operator.kind {
                BinaryOpKind::And => self.visit_logical_and(left, right),
                BinaryOpKind::Or => self.visit_logical_or(left, right),
                _ => {
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
                        BinaryOpKind::GreaterEqual => {
                            CfgInstruction::greater_equal(dest, src1, src2)
                        }
                        BinaryOpKind::Less => CfgInstruction::less(dest, src1, src2),
                        BinaryOpKind::LessEqual => CfgInstruction::less_equal(dest, src1, src2),
                        _ => unreachable!(),
                    };

                    self.emit_instruction(instruction);

                    dest
                }
            },
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

                let arguments_src = arguments
                    .iter()
                    .map(|argument| self.visit_expression(argument))
                    .collect::<Vec<Variable>>();

                let src = self.visit_expression(callee);

                let caller_size = self.variable;

                for src in arguments_src {
                    let dest = self.create_variable();
                    let instruction = CfgInstruction::move_(dest, src);

                    self.emit_instruction(instruction);
                }

                let instruction = CfgInstruction::call(dest, src, caller_size);

                self.emit_instruction(instruction);

                dest
            }

            HirExprKind::VariableRef(id) => *self
                .nodes_variable
                .get(id)
                .expect("VariableRef points to a missing variable node"),
            HirExprKind::FunctionRef(id) => {
                let value = *self
                    .nodes_block
                    .get(id)
                    .expect("FunctionRef points to a missing variable node");

                self.cfg_ir.constants.push_function_ref(value)
            }
            HirExprKind::String(value) => self.cfg_ir.constants.push_string(value.to_owned()),
            HirExprKind::Boolean(value) => self.cfg_ir.constants.push_boolean(*value),
            HirExprKind::Number(value) => self.cfg_ir.constants.push_number(*value),
        }
    }
}
