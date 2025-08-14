#![allow(clippy::new_without_default)]

use std::collections::HashMap;

use crate::{
    backend::vm::value::Value,
    error::kaori_error::KaoriError,
    frontend::{
        semantic::{
            resolved_ast_node::ResolvedAstNode,
            resolved_decl::{ResolvedDecl, ResolvedDeclKind},
            resolved_expr::{ResolvedExpr, ResolvedExprKind},
            resolved_stmt::{ResolvedStmt, ResolvedStmtKind},
        },
        syntax::operator::{BinaryOp, UnaryOp},
    },
};

use super::{constant_pool::ConstantPool, instruction::Instruction};

pub struct BytecodeGenerator<'a> {
    instructions: &'a mut Vec<Instruction>,
    constant_pool: &'a mut ConstantPool,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(
        instructions: &'a mut Vec<Instruction>,
        constant_pool: &'a mut ConstantPool,
    ) -> Self {
        Self {
            instructions,
            constant_pool,
        }
    }

    pub fn generate(&mut self, declarations: &[ResolvedDecl]) -> Result<(), KaoriError> {
        for declaration in declarations {
            self.visit_declaration(declaration)?;
        }

        Ok(())
    }

    pub fn emit(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    fn visit_nodes(&mut self, nodes: &[ResolvedAstNode]) -> Result<(), KaoriError> {
        for node in nodes {
            self.visit_ast_node(node)?;
        }

        Ok(())
    }

    fn visit_ast_node(&mut self, node: &ResolvedAstNode) -> Result<(), KaoriError> {
        match node {
            ResolvedAstNode::Declaration(declaration) => self.visit_declaration(declaration),
            ResolvedAstNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &ResolvedDecl) -> Result<(), KaoriError> {
        match &declaration.kind {
            ResolvedDeclKind::Variable { right, .. } => {
                self.visit_expression(right)?;
                self.emit(Instruction::Declare);
            }
            ResolvedDeclKind::Function { body, id, .. } => {
                let instruction_ptr = self.instructions.len();

                self.visit_nodes(body)?;

                let value = Value::function_ref(instruction_ptr);
                self.constant_pool.define_function_constant(*id, value);
            }
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &ResolvedStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            ResolvedStmtKind::Expression(expression) => {
                self.visit_expression(expression)?;
            }
            ResolvedStmtKind::Print(expression) => {
                self.visit_expression(expression)?;

                self.emit(Instruction::Print);
            }
            ResolvedStmtKind::Block(nodes) => {
                self.emit(Instruction::EnterScope);

                for node in nodes {
                    self.visit_ast_node(node)?;
                }

                self.emit(Instruction::ExitScope);
            }
            ResolvedStmtKind::If {
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
            ResolvedStmtKind::WhileLoop { condition, block } => {
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
        match &expression.kind {
            ResolvedExprKind::Assign { left, right } => {
                self.visit_expression(right)?;

                if let ResolvedExprKind::VariableRef { offset, .. } = left.kind {
                    self.emit(Instruction::StoreLocal(offset as u16));
                };
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
            ResolvedExprKind::NumberLiteral(value) => {
                let index = self.constant_pool.load_constant(Value::number(*value));

                self.emit(Instruction::LoadConst(index as u16));
            }
            ResolvedExprKind::BooleanLiteral(value) => {
                let index = self.constant_pool.load_constant(Value::boolean(*value));

                self.emit(Instruction::LoadConst(index as u16));
            }
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
            ResolvedExprKind::VariableRef { offset, .. } => {
                self.emit(Instruction::LoadLocal(*offset as u16));
            }
            ResolvedExprKind::FunctionRef { function_id, .. } => {
                let index = self.constant_pool.load_function_constant(*function_id);

                self.emit(Instruction::LoadConst(index as u16));
            }
            _ => {}
        };

        Ok(())
    }
}
