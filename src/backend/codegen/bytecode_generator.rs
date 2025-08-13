#![allow(clippy::new_without_default)]

use crate::{
    backend::vm::value::Value,
    error::kaori_error::KaoriError,
    frontend::{
        semantic::resolved_expr::{ResolvedExpr, ResolvedExprKind},
        syntax::operator::BinaryOp,
    },
};

use super::{constant_pool::ConstantPool, instruction::Instruction};

pub struct BytecodeGenerator<'a> {
    instructions: &'a mut Vec<Instruction>,
    constant_pool: &'a mut ConstantPool,
    functions: Vec<String>,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(
        instructions: &'a mut Vec<Instruction>,
        constant_pool: &'a mut ConstantPool,
    ) -> Self {
        Self {
            instructions,
            constant_pool,
            functions: Vec::new(),
        }
    }

    pub fn generate(&mut self, declarations: &mut [Decl]) -> Result<(), KaoriError> {
        self.emit(Instruction::EnterScope);

        for declaration in declarations {
            if let DeclKind::Function { block, .. } = &mut declaration.kind {
                let instruction_ptr = self.instructions.len();

                self.emit_constant(Value::Function(instruction_ptr));
                self.emit(Instruction::Declare);

                let jump_end = self.emit(Instruction::Nothing);

                if let StmtKind::Block(nodes) = &mut block.kind {
                    for node in nodes {
                        self.visit_ast_node(node)?;
                    }
                }

                self.instructions[jump_end] =
                    Instruction::Jump(self.instructions.len() as i16 - jump_end as i16);
            }
        }

        self.emit(Instruction::ExitScope);

        Ok(())
    }

    pub fn emit(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    pub fn emit_constant(&mut self, constant: Value) {
        let index = self.constant_pool.add_constant(constant);

        self.emit(Instruction::LoadConst(index as u16));
    }

    fn visit_ast_node(&mut self, node: &mut ResolvedAstNode) -> Result<(), KaoriError> {
        match node {
            ResolvedAstNode::Declaration(declaration) => self.visit_declaration(declaration),
            ResolvedAstNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError> {
        match &mut declaration.kind {
            DeclKind::Variable { right, .. } => {
                self.visit_expression(right)?;
                self.emit(Instruction::Declare);
            }
            DeclKind::Function { block, name, .. } => {}
            _ => {}
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression)?;
            }
            StmtKind::Print(expression) => {
                self.visit_expression(expression)?;

                self.emit(Instruction::Print);
            }
            StmtKind::Block(nodes) => {
                self.emit(Instruction::EnterScope);

                for node in nodes {
                    self.visit_ast_node(node)?;
                }

                self.emit(Instruction::ExitScope);
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;

                let jump_if_false = self.emit(Instruction::Nothing);

                self.visit_statement(then_branch)?;

                let jump_end = self.emit(Instruction::Nothing);

                self.instructions[jump_if_false] =
                    Instruction::JumpIfFalse(self.instructions.len() as i16 - jump_if_false as i16);

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.instructions[jump_end] =
                    Instruction::Jump(self.instructions.len() as i16 - jump_end as i16);
            }
            StmtKind::WhileLoop { condition, block } => {
                let start = self.instructions.len();

                self.visit_expression(condition)?;

                let jump_if_false = self.emit(Instruction::Nothing);

                self.visit_statement(block)?;

                self.emit(Instruction::Jump(
                    start as i16 - self.instructions.len() as i16,
                ));

                self.instructions[jump_if_false] =
                    Instruction::JumpIfFalse(self.instructions.len() as i16 - jump_if_false as i16);
            }
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &ResolvedExpr) -> Result<(), KaoriError> {
        match &mut expression.kind {
            ResolvedExprKind::Assign { identifier, right } => {
                self.visit_expression(right)?;

                let ResolvedExprKind::Identifier { resolution, .. } = &identifier.kind else {
                    unreachable!();
                };

                if resolution.global {
                    self.emit(Instruction::StoreGlobal(resolution.offset as u16));
                } else {
                    self.emit(Instruction::StoreLocal(resolution.offset as u16));
                }
            }
            ResolvedExprKind::Binary {
                left,
                right,
                operator,
            } => {
                self.visit_expression(left)?;
                self.visit_expression(right)?;

                match operator {
                    BinaryOp::Plus => self.emit(Instruction::Plus),
                    BinaryOp::Minus => self.emit(Instruction::Minus),
                    BinaryOp::Multiply => self.emit(Instruction::Multiply),
                    BinaryOp::Divide => self.emit(Instruction::Divide),
                    BinaryOp::Modulo => self.emit(Instruction::Modulo),

                    BinaryOp::And => self.emit(Instruction::And),
                    BinaryOp::Or => self.emit(Instruction::Or),

                    BinaryOp::Equal => self.emit(Instruction::Equal),
                    BinaryOp::NotEqual => self.emit(Instruction::NotEqual),

                    BinaryOp::Greater => self.emit(Instruction::Greater),
                    BinaryOp::GreaterEqual => self.emit(Instruction::GreaterEqual),
                    BinaryOp::Less => self.emit(Instruction::Less),
                    BinaryOp::LessEqual => self.emit(Instruction::LessEqual),
                };
            }
            ResolvedExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.emit(Instruction::Negate),
                    UnaryOp::Not => self.emit(Instruction::Not),
                };
            }
            ResolvedExprKind::VariableRef { offset, ty } => {
                self.emit(Instruction::LoadLocal(offset as u16))
            }

            ResolvedExprKind::NumberLiteral(value) => self.emit_constant(Value::number(*value)),
            ResolvedExprKind::BooleanLiteral(value) => self.emit_constant(Value::boolean(*value)),
            //ResolvedExprKind::StringLiteral(value) => self.emit_constant(Value::str(value.to_owned())),
            ResolvedExprKind::FunctionCall { callee, arguments } => {
                self.visit_expression(callee)?;

                self.emit(Instruction::EnterFunction);

                for argument in arguments {
                    self.visit_expression(argument)?;
                    self.emit(Instruction::Declare);
                }

                self.emit(Instruction::ExitFunction);
            }
            _ => (),
        };

        Ok(())
    }
}
