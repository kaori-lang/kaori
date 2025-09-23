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

use super::{basic_block::Terminator, cfg::Cfg, virtual_reg_inst::VirtualRegInst};

pub struct CfgBuilder<'a> {
    cfgs: &'a mut Vec<Cfg>,
    nodes_register: HashMap<HirId, usize>,
    functions: HashMap<HirId, usize>,
    register: usize,
}

impl<'a> CfgBuilder<'a> {
    pub fn new(cfgs: &'a mut Vec<Cfg>) -> Self {
        Self {
            cfgs,
            register: 0,
            nodes_register: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn allocate_register(&mut self) -> usize {
        let register = self.register;

        self.register += 1;

        register
    }

    fn current_cfg(&mut self) -> &mut Cfg {
        self.cfgs.last_mut().unwrap()
    }

    pub fn build_ir(&mut self, declarations: &[HirDecl]) {
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
                let src1 = self.visit_expression(right);
                let dest = self.allocate_register();

                self.nodes_register.insert(declaration.id, dest);

                let instruction = VirtualRegInst::Move { dest, src1 };

                self.current_cfg().emit_instruction(instruction);
            }

            HirDeclKind::Function { body, parameters } => {
                self.functions.insert(declaration.id, self.cfgs.len());

                let cfg = Cfg::new();

                self.cfgs.push(cfg);

                for parameter in parameters {
                    let register = self.allocate_register();
                    self.nodes_register.insert(parameter.id, register);
                }

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

                let condition_bb = self.current_cfg().current_bb;
                let then_bb = self.current_cfg().create_bb();
                let else_bb = self.current_cfg().create_bb();
                let terminator_block = self.current_cfg().create_bb();

                self.current_cfg().current_bb = then_bb;
                self.visit_statement(then_branch);

                if let Some(branch) = else_branch {
                    self.current_cfg().current_bb = else_bb;
                    self.visit_statement(branch);
                }

                self.current_cfg().get_bb(condition_bb).terminator = Terminator::Branch {
                    r#true: then_bb,
                    r#false: else_bb,
                };

                self.current_cfg().get_bb(then_bb).terminator = Terminator::Goto(terminator_block);

                self.current_cfg().get_bb(else_bb).terminator = Terminator::Goto(terminator_block);

                self.current_cfg().current_bb = terminator_block;
            }
            HirStmtKind::Loop {
                init,
                condition,
                block,
            } => {
                if let Some(init) = init {
                    self.visit_declaration(init);
                }
                let previous_bb = self.current_cfg().current_bb;
                let condition_bb = self.current_cfg().create_bb();
                let block_bb = self.current_cfg().create_bb();
                let terminator_bb = self.current_cfg().create_bb();

                self.current_cfg().current_bb = condition_bb;
                self.visit_expression(condition);

                self.current_cfg().current_bb = block_bb;
                self.visit_statement(block);

                self.current_cfg().get_bb(previous_bb).terminator = Terminator::Goto(condition_bb);

                self.current_cfg().get_bb(condition_bb).terminator = Terminator::Branch {
                    r#true: block_bb,
                    r#false: terminator_bb,
                };

                self.current_cfg().get_bb(block_bb).terminator = Terminator::Goto(condition_bb);

                self.current_cfg().current_bb = terminator_bb;
            }
            HirStmtKind::Break => {}
            HirStmtKind::Continue => {}
            HirStmtKind::Return(expr) => {
                if let Some(expr) = expr {
                    let source = self.visit_expression(expr);

                    let instruction = VirtualRegInst::Return { source };

                    self.current_cfg().emit_instruction(instruction);
                }
                let current_bb = self.current_cfg().current_bb;

                self.current_cfg().get_bb(current_bb).terminator = Terminator::Return;
            }
        };
    }

    fn visit_expression(&mut self, expression: &HirExpr) -> usize {
        match &expression.kind {
            HirExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src1 = self.visit_expression(right);

                let instruction = VirtualRegInst::Move { dest, src1 };

                self.current_cfg().emit_instruction(instruction);

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

                self.current_cfg().emit_instruction(instruction);

                dest
            }
            HirExprKind::Unary { right, operator } => {
                let src1 = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => VirtualRegInst::Negate { dest, src1 },
                    UnaryOpKind::Not => VirtualRegInst::Not { dest, src1 },
                };

                self.current_cfg().emit_instruction(instruction);

                dest
            }
            HirExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register();

                let call_instruction = VirtualRegInst::Call;

                self.current_cfg().emit_instruction(call_instruction);

                for (dest, argument) in arguments.iter().enumerate() {
                    let src1 = self.visit_expression(argument);

                    let instruction = VirtualRegInst::Move { dest, src1 };

                    self.current_cfg().emit_instruction(instruction);
                }

                let callee_register = self.visit_expression(callee);

                dest
            }

            HirExprKind::VariableRef(id) => *self.nodes_register.get(id).unwrap(),
            HirExprKind::FunctionRef(id) => {
                let dest = self.allocate_register();

                let value = *self.functions.get(id).unwrap();

                let instruction = VirtualRegInst::FunctionConst { dest, value };

                self.current_cfg().emit_instruction(instruction);

                dest
            }
            HirExprKind::StringLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = VirtualRegInst::StringConst {
                    dest,
                    value: value.to_owned(),
                };

                self.current_cfg().emit_instruction(instruction);

                dest
            }
            HirExprKind::BooleanLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = VirtualRegInst::BooleanConst {
                    dest,
                    value: *value,
                };

                self.current_cfg().emit_instruction(instruction);

                dest
            }
            HirExprKind::NumberLiteral(value) => {
                let dest = self.allocate_register();

                let instruction = VirtualRegInst::NumberConst {
                    dest,
                    value: *value,
                };

                self.current_cfg().emit_instruction(instruction);

                dest
            }
        }
    }
}
