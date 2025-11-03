use std::collections::HashMap;

use crate::{
    error::kaori_error::KaoriError,
    kaori_error,
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
    basic_block::{BasicBlock, Terminator},
    cfg_constants::CfgConstants,
    cfg_function::CfgFunction,
    cfg_instruction::CfgInstruction,
    operand::Operand,
};

pub fn build_cfgs(declarations: &[HirDecl]) -> Result<Vec<CfgFunction>, KaoriError> {
    let mut functions = HashMap::new();

    for declaration in declarations {
        if let HirDeclKind::Function { .. } = &declaration.kind {
            let index = functions.len();

            functions.insert(declaration.id, index);
        }
    }

    let mut cfgs = Vec::new();

    for declaration in declarations {
        if let HirDeclKind::Function { .. } = &declaration.kind {
            let mut ctx = CfgContext::new(&functions);

            ctx.visit_declaration(declaration)?;

            let cfg = CfgFunction::new(
                ctx.basic_blocks,
                ctx.constants.constants,
                ctx.variables.len(),
            );

            cfgs.push(cfg);
        }
    }

    Ok(cfgs)
}

pub struct CfgContext<'a> {
    index: usize,
    variables: HashMap<HirId, Operand>,
    constants: CfgConstants,
    basic_blocks: Vec<BasicBlock>,
    active_loops: ActiveLoops,
    functions: &'a HashMap<HirId, usize>,
}

impl<'a> CfgContext<'a> {
    pub fn new(functions: &'a HashMap<HirId, usize>) -> Self {
        Self {
            index: 0,
            variables: HashMap::new(),
            constants: CfgConstants::default(),
            basic_blocks: Vec::new(),
            active_loops: ActiveLoops::default(),
            functions,
        }
    }

    pub fn create_variable(&mut self, id: HirId) -> Operand {
        let variable = Operand::Variable(self.variables.len());

        self.variables.insert(id, variable);

        variable
    }

    fn emit_instruction(&mut self, instruction: CfgInstruction) {
        let basic_block = &mut self.basic_blocks[self.index];

        if let Terminator::None = basic_block.terminator {
            basic_block.instructions.push(instruction);
        };
    }

    fn set_terminator(&mut self, terminator: Terminator) {
        let basic_block = &mut self.basic_blocks[self.index];

        if let Terminator::None = basic_block.terminator {
            basic_block.terminator = terminator;
        };
    }

    fn create_bb(&mut self) -> usize {
        let index = self.basic_blocks.len();

        let basic_block = BasicBlock::new(index);

        self.basic_blocks.push(basic_block);

        index
    }

    fn visit_nodes(&mut self, nodes: &[HirNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.visit_ast_node(node)?;
        }

        Ok(())
    }

    fn visit_ast_node(&mut self, node: &HirNode) -> Result<(), KaoriError> {
        match node {
            HirNode::Declaration(declaration) => self.visit_declaration(declaration)?,
            HirNode::Statement(statement) => self.visit_statement(statement)?,
        };

        Ok(())
    }

    fn visit_declaration(&mut self, declaration: &HirDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            HirDeclKind::Variable { right, .. } => {
                let src = self.visit_expression(right);
                let dest = self.create_variable(declaration.id);

                let instruction = CfgInstruction::move_(dest, src);

                self.emit_instruction(instruction);
            }

            HirDeclKind::Function {
                body,
                parameters,
                return_ty,
            } => {
                let _entry_bb = self.create_bb();

                for parameter in parameters {
                    self.create_variable(parameter.id);
                }

                for node in body {
                    self.visit_ast_node(node)?;
                }

                match self.basic_blocks[self.index].terminator {
                    Terminator::Return { .. } => {}
                    _ => {
                        if return_ty.is_some() {
                            return Err(kaori_error!(
                                declaration.span,
                                "expected a return statement"
                            ));
                        }

                        self.set_terminator(Terminator::Return { src: None })
                    }
                }
            }
            HirDeclKind::Struct { .. } => {}
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &HirStmt) -> Result<(), KaoriError> {
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
                self.visit_nodes(nodes)?;
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
                    src,
                    r#true: then_bb,
                    r#false: else_bb,
                });

                self.index = then_bb;
                self.visit_statement(then_branch)?;
                self.set_terminator(Terminator::Goto(terminator_block));

                self.index = else_bb;
                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
                self.set_terminator(Terminator::Goto(terminator_block));

                self.index = terminator_block;
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init)?;
                }

                let condition_bb = self.create_bb();
                let block_bb = self.create_bb();
                let terminator_bb = self.create_bb();
                let increment_bb = self.create_bb();

                self.set_terminator(Terminator::Goto(condition_bb));

                self.index = condition_bb;
                let src = self.visit_expression(condition);
                self.set_terminator(Terminator::Branch {
                    src,
                    r#true: block_bb,
                    r#false: terminator_bb,
                });

                self.index = block_bb;
                self.active_loops.push(increment_bb, terminator_bb);
                self.visit_statement(block)?;
                self.active_loops.pop();
                self.set_terminator(Terminator::Goto(increment_bb));

                self.index = increment_bb;
                if let Some(increment) = increment {
                    self.visit_statement(increment)?;
                }
                self.set_terminator(Terminator::Goto(condition_bb));

                self.index = terminator_bb;
            }
            HirStmtKind::Break => {
                let label = self.active_loops.top();

                self.set_terminator(Terminator::Goto(label.terminator_bb_index));
            }
            HirStmtKind::Continue => {
                let label = self.active_loops.top();

                self.set_terminator(Terminator::Goto(label.increment_bb_index));
            }
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let src = self.visit_expression(expr);

                    self.set_terminator(Terminator::Return { src: Some(src) });
                } else {
                    self.set_terminator(Terminator::Return { src: None });
                }
            }
        };

        Ok(())
    }

    fn visit_logical_or(&mut self, id: HirId, left: &HirExpr, right: &HirExpr) -> Operand {
        let dest = self.create_variable(id);

        let src1 = self.visit_expression(left);

        self.emit_instruction(CfgInstruction::move_(dest, src1));

        let src2_bb = self.create_bb();
        let terminator = self.create_bb();

        self.set_terminator(Terminator::Branch {
            src: dest,
            r#true: terminator,
            r#false: src2_bb,
        });

        self.index = src2_bb;

        let src2 = self.visit_expression(right);

        self.emit_instruction(CfgInstruction::move_(dest, src2));

        self.set_terminator(Terminator::Goto(terminator));

        self.index = terminator;

        dest
    }

    fn visit_logical_and(&mut self, id: HirId, left: &HirExpr, right: &HirExpr) -> Operand {
        let dest = self.create_variable(id);

        let src1 = self.visit_expression(left);

        self.emit_instruction(CfgInstruction::move_(dest, src1));

        let src2_bb = self.create_bb();
        let terminator = self.create_bb();

        self.set_terminator(Terminator::Branch {
            src: dest,
            r#true: src2_bb,
            r#false: terminator,
        });

        self.index = src2_bb;

        let src2 = self.visit_expression(right);
        self.emit_instruction(CfgInstruction::move_(dest, src2));
        self.set_terminator(Terminator::Goto(terminator));

        self.index = terminator;

        dest
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> Operand {
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
                BinaryOpKind::And => self.visit_logical_and(expression.id, left, right),
                BinaryOpKind::Or => self.visit_logical_or(expression.id, left, right),
                _ => {
                    let src1 = self.visit_expression(left);
                    let src2 = self.visit_expression(right);
                    let dest = self.create_variable(expression.id);

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
                let dest = self.create_variable(expression.id);

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => CfgInstruction::negate(dest, src),
                    UnaryOpKind::Not => CfgInstruction::not(dest, src),
                };

                self.emit_instruction(instruction);

                dest
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dest = self.create_variable(expression.id);

                let arguments_src = arguments
                    .iter()
                    .map(|argument| self.visit_expression(argument))
                    .collect::<Vec<Operand>>();

                let src = self.visit_expression(callee);

                for (index, src) in arguments_src.iter().copied().enumerate() {
                    let dest = Operand::Variable(index);

                    let instruction = CfgInstruction::move_arg(dest, src);

                    self.emit_instruction(instruction);
                }

                let instruction = CfgInstruction::call(dest, src);

                self.emit_instruction(instruction);

                dest
            }

            HirExprKind::Variable(id) => *self
                .variables
                .get(id)
                .expect("Variable not found for HirId"),

            HirExprKind::Function(id) => {
                let value = *self
                    .functions
                    .get(id)
                    .expect("FunctionRef points to a missing variable node");

                self.constants.push_function(value)
            }
            HirExprKind::String(value) => self.constants.push_string(value.to_owned()),
            HirExprKind::Boolean(value) => self.constants.push_boolean(*value),
            HirExprKind::Number(value) => self.constants.push_number(*value),
        }
    }
}
