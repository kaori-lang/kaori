#![allow(clippy::new_without_default)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::{
        semantic::visitor::Visitor,
        syntax::{
            ast_node::ASTNode,
            declaration::{Decl, DeclKind},
            expression::{Expr, ExprKind},
            operator::{BinaryOp, UnaryOp},
            statement::{Stmt, StmtKind},
        },
    },
};

use super::{const_value::ConstValue, instruction::Instruction};

pub struct BytecodeGenerator {
    instructions: Vec<Instruction>,
    constant_pool: Vec<ConstValue>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
        }
    }

    pub fn generate(&mut self, ast: &mut [ASTNode]) -> Result<(), KaoriError> {
        self.emit(Instruction::EnterScope);

        for i in 0..ast.len() {
            if let Some(ASTNode::Declaration(decl)) = ast.get(i)
                && let DeclKind::Function { .. } = &decl.kind
            {
                //self.environment.declare(type_annotation.clone());
            }
        }

        for i in 0..ast.len() {
            if let Some(node) = ast.get_mut(i) {
                self.visit_ast_node(node)?;
            }
        }

        self.emit(Instruction::ExitScope);

        Ok(())
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn emit_constant(&mut self, other: ConstValue) {
        let mut index = 0;

        while index < self.constant_pool.len() {
            let current = &self.constant_pool[index];

            if current.equal(&other) {
                break;
            }

            index += 1;
        }

        if index == self.constant_pool.len() {
            self.constant_pool.push(other);
        }

        self.emit(Instruction::LoadConst(index));
    }

    pub fn create_placeholder(&mut self) -> usize {
        let index = self.instructions.len();

        self.instructions.push(Instruction::Nothing);

        index
    }

    pub fn update_placeholder(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }
}

impl Visitor<()> for BytecodeGenerator {
    fn visit_ast_node(&mut self, ast_node: &mut ASTNode) -> Result<(), KaoriError> {
        match ast_node {
            ASTNode::Declaration(declaration) => self.visit_declaration(declaration),
            ASTNode::Statement(statement) => self.visit_statement(statement),
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Decl) -> Result<(), KaoriError> {
        match &mut declaration.kind {
            DeclKind::Variable { right, .. } => {
                self.visit_expression(right)?;
                self.emit(Instruction::Declare);
            }
            DeclKind::Function {
                parameters, block, ..
            } => {
                for parameter in parameters {
                    self.visit_declaration(parameter)?;
                }

                let StmtKind::Block(declarations) = &mut block.kind else {
                    unreachable!()
                };

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }
            }
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &mut Stmt) -> Result<(), KaoriError> {
        match &mut statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression.as_mut())?;
            }
            StmtKind::Print(expression) => {
                self.visit_expression(expression.as_mut())?;

                self.emit(Instruction::Print);
            }
            StmtKind::Block(declarations) => {
                self.emit(Instruction::EnterScope);

                for declaration in declarations {
                    self.visit_ast_node(declaration)?;
                }

                self.emit(Instruction::ExitScope);
            }
            StmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;

                let jump_if_false_placeholder = self.create_placeholder();

                self.visit_statement(then_branch)?;

                let jump_end_placeholder = self.create_placeholder();

                self.update_placeholder(
                    jump_if_false_placeholder,
                    Instruction::JumpIfFalse(self.instructions.len()),
                );

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }

                self.update_placeholder(
                    jump_end_placeholder,
                    Instruction::Jump(self.instructions.len()),
                );
            }
            StmtKind::WhileLoop { condition, block } => {
                let start = self.instructions.len();

                self.visit_expression(condition)?;

                let jump_if_false_placeholder = self.create_placeholder();

                self.visit_statement(block)?;

                self.emit(Instruction::Jump(start));

                self.update_placeholder(
                    jump_if_false_placeholder,
                    Instruction::JumpIfFalse(self.instructions.len()),
                );
            }
            _ => (),
        };

        Ok(())
    }
    fn visit_expression(&mut self, expression: &mut Expr) -> Result<(), KaoriError> {
        match &mut expression.kind {
            ExprKind::Assign { identifier, right } => {
                self.visit_expression(right)?;

                let ExprKind::Identifier { resolution, .. } = &identifier.kind else {
                    unreachable!();
                };

                if resolution.global {
                    self.emit(Instruction::StoreGlobal(resolution.offset));
                } else {
                    self.emit(Instruction::StoreLocal(resolution.offset));
                }
            }
            ExprKind::Binary {
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
                }
            }
            ExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.emit(Instruction::Negate),
                    UnaryOp::Not => self.emit(Instruction::Not),
                }
            }
            ExprKind::Identifier { resolution, .. } => {
                if resolution.global {
                    self.emit(Instruction::LoadGlobal(resolution.offset));
                } else {
                    self.emit(Instruction::LoadLocal(resolution.offset));
                }
            }
            ExprKind::NumberLiteral(value) => self.emit_constant(ConstValue::Number(*value)),
            ExprKind::BooleanLiteral(value) => self.emit_constant(ConstValue::Bool(*value)),
            /* ExprKind::StringLiteral(value) => {
                self.emit_constant(Value::Str(value.to_string()))
            } */
            _ => (),
        };

        Ok(())
    }
}
