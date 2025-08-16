#![allow(clippy::new_without_default)]

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

use super::{bytecode::Bytecode, instruction::Instruction};

pub struct BytecodeGenerator<'a> {
    bytecode: &'a mut Bytecode,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(bytecode: &'a mut Bytecode) -> Self {
        Self { bytecode }
    }

    pub fn generate(&mut self, declarations: &[ResolvedDecl]) -> Result<(), KaoriError> {
        let jump_main = self.emit(Instruction::Nothing);

        for (index, declaration) in declarations.iter().enumerate() {
            if index == declarations.len() - 1 {
                let offset = self.bytecode.instructions.len();
                self.bytecode.instructions[jump_main] = Instruction::Jump(offset as u16);
            }

            self.visit_declaration(declaration)?;
        }

        Ok(())
    }

    pub fn emit(&mut self, instruction: Instruction) -> usize {
        let index = self.bytecode.instructions.len();

        self.bytecode.instructions.push(instruction);

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
                let instruction_ptr = self.bytecode.instructions.len();

                self.visit_nodes(body)?;

                let value = Value::function_ref(instruction_ptr);

                self.bytecode
                    .constant_pool
                    .define_function_constant(*id, value);
            }
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &ResolvedStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            ResolvedStmtKind::Expression(expression) => {
                self.visit_expression(expression)?;

                self.emit(Instruction::Pop);
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

                self.bytecode.instructions[jump_if_false] =
                    Instruction::JumpIfFalse(self.bytecode.instructions.len() as u16);

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.bytecode.instructions[jump_end] =
                    Instruction::Jump(self.bytecode.instructions.len() as u16);
            }
            ResolvedStmtKind::WhileLoop {
                condition, block, ..
            } => {
                let start = self.bytecode.instructions.len();

                self.visit_expression(condition)?;

                let jump_if_false = self.emit(Instruction::Nothing);

                self.visit_statement(block)?;

                self.emit(Instruction::Jump(start as u16));

                self.bytecode.instructions[jump_if_false] =
                    Instruction::JumpIfFalse(self.bytecode.instructions.len() as u16);
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
                let index = self
                    .bytecode
                    .constant_pool
                    .load_constant(Value::number(*value));

                self.emit(Instruction::LoadConst(index as u16));
            }
            ResolvedExprKind::BooleanLiteral(value) => {
                let index = self
                    .bytecode
                    .constant_pool
                    .load_constant(Value::boolean(*value));

                self.emit(Instruction::LoadConst(index as u16));
            }
            //ResolvedExprKind::StringLiteral(value) => self.emit_constant(Value::str(value.to_owned())),
            ResolvedExprKind::FunctionCall { callee, arguments } => {
                self.visit_expression(callee)?;

                self.emit(Instruction::Call);

                for argument in arguments {
                    self.visit_expression(argument)?;
                    self.emit(Instruction::Declare);
                }
            }
            ResolvedExprKind::VariableRef { offset, .. } => {
                self.emit(Instruction::LoadLocal(*offset as u16));
            }
            ResolvedExprKind::FunctionRef { function_id, .. } => {
                let index = self
                    .bytecode
                    .constant_pool
                    .load_function_constant(*function_id);

                self.emit(Instruction::LoadConst(index as u16));
            }
            _ => {}
        };

        Ok(())
    }
}
