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

use super::basic_block::{BasicBlock, CfgInstruction};

pub struct CfgIr {
    blocks: Vec<BasicBlock>,
}

impl<'a> CfgIr {
    pub fn new(blocks: Vec<BasicBlock>) -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn emit(&mut self, instruction: CfgInstruction) {}

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
            ResolvedDeclKind::Variable { offset, right, .. } => {
                self.visit_expression(right)?;
                self.emit(CfgInstruction::StoreLocal(*offset));
            }
            ResolvedDeclKind::Function { body, id, .. } => {
                self.visit_nodes(body)?;
            }
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &ResolvedStmt) -> Result<(), KaoriError> {
        match &statement.kind {
            ResolvedStmtKind::Expression(expression) => {
                self.visit_expression(expression)?;

                self.emit(CfgInstruction::Pop);
            }
            ResolvedStmtKind::Print(expression) => {
                self.visit_expression(expression)?;

                self.emit(CfgInstruction::Print);
            }
            ResolvedStmtKind::Block(nodes) => {
                for node in nodes {
                    self.visit_ast_node(node)?;
                }
            }
            ResolvedStmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;

                self.visit_statement(then_branch)?;

                if let Some(branch) = else_branch {
                    self.visit_statement(branch)?;
                }
            }
            ResolvedStmtKind::WhileLoop { condition, block } => {
                self.visit_expression(condition)?;

                self.visit_statement(block)?;
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
                    self.emit(CfgInstruction::StoreLocal(offset));
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
                    BinaryOp::Add => self.emit(CfgInstruction::Plus),
                    BinaryOp::Subtract => self.emit(CfgInstruction::Minus),
                    BinaryOp::Multiply => self.emit(CfgInstruction::Multiply),
                    BinaryOp::Divide => self.emit(CfgInstruction::Divide),
                    BinaryOp::Modulo => self.emit(CfgInstruction::Modulo),

                    BinaryOp::And => self.emit(CfgInstruction::And),
                    BinaryOp::Or => self.emit(CfgInstruction::Or),

                    BinaryOp::Equal => self.emit(CfgInstruction::Equal),
                    BinaryOp::NotEqual => self.emit(CfgInstruction::NotEqual),

                    BinaryOp::Greater => self.emit(CfgInstruction::Greater),
                    BinaryOp::GreaterEqual => self.emit(CfgInstruction::GreaterEqual),
                    BinaryOp::Less => self.emit(CfgInstruction::Less),
                    BinaryOp::LessEqual => self.emit(CfgInstruction::LessEqual),
                };
            }
            ResolvedExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.emit(CfgInstruction::Negate),
                    UnaryOp::Not => self.emit(CfgInstruction::Not),
                };
            }
            ResolvedExprKind::NumberLiteral(value) => {
                self.emit(CfgInstruction::NumberConst(*value));
            }
            ResolvedExprKind::BooleanLiteral(value) => {
                self.emit(CfgInstruction::BooleanConst(*value));
            }
            ResolvedExprKind::StringLiteral(value) => {
                self.emit(CfgInstruction::StringConst(value.to_owned()));
            }
            ResolvedExprKind::FunctionCall { callee, arguments } => {
                self.visit_expression(callee)?;

                self.emit(CfgInstruction::Call);

                for (offset, argument) in arguments.iter().enumerate() {
                    self.visit_expression(argument)?;
                    self.emit(CfgInstruction::StoreLocal(offset));
                }
            }
            ResolvedExprKind::VariableRef { offset, .. } => {
                self.emit(CfgInstruction::LoadLocal(*offset));
            }
            ResolvedExprKind::FunctionRef { function_id, .. } => {
                self.emit(CfgInstruction::FunctionConst {
                    function_id: *function_id,
                });
            }
            _ => {}
        };

        Ok(())
    }
}
