#![allow(clippy::new_without_default)]

use crate::{
    error::kaori_error::KaoriError,
    frontend::syntax::operator::{BinaryOp, UnaryOp},
};

use super::basic_block::{BasicBlock, CfgInst, Terminator};

pub struct CfgIr {
    blocks: Vec<BasicBlock>,
    current: usize,
}

impl CfgIr {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            current: 0,
        }
    }

    pub fn emit(&mut self, instruction: CfgInst) {
        self.blocks[self.current].instructions.push(instruction);
    }

    pub fn create_block(&mut self) -> usize {
        let index = self.blocks.len();

        let block = BasicBlock::default();

        self.blocks.push(block);

        index
    }

    pub fn update_block_terminal(&mut self, index: usize, terminator: Terminator) {
        self.blocks[index].terminator = terminator;
    }

    pub fn start(&mut self, declarations: &[ResolvedDecl]) -> Result<(), KaoriError> {
        for declaration in declarations {
            self.visit_declaration(declaration)?;
        }

        Ok(())
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
            ResolvedDeclKind::Variable { offset, right, .. } => {
                self.visit_expression(right)?;
                self.emit(CfgInst::StoreLocal(*offset));
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

                self.emit(CfgInst::Pop);
            }
            ResolvedStmtKind::Print(expression) => {
                self.visit_expression(expression)?;

                self.emit(CfgInst::Print);
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
                let condition_block = self.create_block();
                let loop_block = self.create_block();

                self.current = condition_block;

                self.visit_expression(condition)?;

                self.current = loop_block;

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

                if let ResolvedExprKind::LocalRef { offset, .. } = left.kind {
                    self.emit(CfgInst::StoreLocal(offset));
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
                    BinaryOp::Add => self.emit(CfgInst::Plus),
                    BinaryOp::Subtract => self.emit(CfgInst::Minus),
                    BinaryOp::Multiply => self.emit(CfgInst::Multiply),
                    BinaryOp::Divide => self.emit(CfgInst::Divide),
                    BinaryOp::Modulo => self.emit(CfgInst::Modulo),

                    BinaryOp::And => self.emit(CfgInst::And),
                    BinaryOp::Or => self.emit(CfgInst::Or),

                    BinaryOp::Equal => self.emit(CfgInst::Equal),
                    BinaryOp::NotEqual => self.emit(CfgInst::NotEqual),

                    BinaryOp::Greater => self.emit(CfgInst::Greater),
                    BinaryOp::GreaterEqual => self.emit(CfgInst::GreaterEqual),
                    BinaryOp::Less => self.emit(CfgInst::Less),
                    BinaryOp::LessEqual => self.emit(CfgInst::LessEqual),
                };
            }
            ResolvedExprKind::Unary { right, operator } => {
                self.visit_expression(right)?;

                match operator {
                    UnaryOp::Negate => self.emit(CfgInst::Negate),
                    UnaryOp::Not => self.emit(CfgInst::Not),
                };
            }
            ResolvedExprKind::NumberLiteral(value) => {
                self.emit(CfgInst::NumberConst(*value));
            }
            ResolvedExprKind::BooleanLiteral(value) => {
                self.emit(CfgInst::BooleanConst(*value));
            }
            ResolvedExprKind::StringLiteral(value) => {
                self.emit(CfgInst::StringConst(value.to_owned()));
            }
            ResolvedExprKind::FunctionCall {
                callee, arguments, ..
            } => {
                self.visit_expression(callee)?;

                self.emit(CfgInst::Call);

                for (offset, argument) in arguments.iter().enumerate() {
                    self.visit_expression(argument)?;
                    self.emit(CfgInst::StoreLocal(offset));
                }
            }
            ResolvedExprKind::LocalRef { offset, .. } => {
                self.emit(CfgInst::LoadLocal(*offset));
            }
            ResolvedExprKind::GlobalRef { id, .. } => {
                self.emit(CfgInst::LoadGlobal(*id));
            }
            _ => {}
        };

        Ok(())
    }
}
