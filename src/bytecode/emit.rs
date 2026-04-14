use std::collections::HashMap;

use crate::{
    ast::{binary_op::BinaryOpKind, unary_op::UnaryOpKind},
    error::kaori_error::KaoriError,
    hir::{
        decl::{Decl, DeclKind},
        expr::{Expr, ExprKind},
        node_id::NodeId,
        stmt::{Stmt, StmtKind},
    },
};

use super::{constants::Constants, function::Function, instruction::Instruction};

pub fn build_functions_graph(declarations: &[Decl]) -> Result<Vec<Function>, KaoriError> {
    let mut node_to_function = HashMap::new();

    let counter: usize = 0;
    for declaration in declarations {
        match &declaration.kind {
            DeclKind::Function { .. } => {
                node_to_function.insert(declaration.id, counter);
            }
        }

        counter += 1;
    }

    let mut functions = Vec::new();

    for declaration in declarations {
        match &declaration.kind {
            DeclKind::Function { .. } => {
                let mut ctx = FunctionContext::new(&node_to_function);

                ctx.visit_declaration(declaration)?;

                let id = *node_to_function.get(&declaration.id).unwrap();

                let function = Function::new(
                    id,
                    ctx.basic_blocks,
                    ctx.constants.constants,
                    ctx.next_register,
                );

                functions.push(function);
            }
        }
    }

    Ok(functions)
}

pub struct FunctionContext<'a> {
    next_register: u8,
    registers: HashMap<NodeId, u8>,
    constants: Constants,
    instructions: Vec<Instruction>,
    active_loops: ActiveLoops,
    node_to_function: &'a HashMap<NodeId, usize>,
}

impl<'a> FunctionContext<'a> {
    pub fn new(node_to_function: &'a HashMap<NodeId, usize>) -> Self {
        Self {
            registers: HashMap::new(),
            next_register: 0,
            constants: Constants::default(),
            instructions: Vec::new(),
            active_loops: ActiveLoops::default(),
            node_to_function,
        }
    }

    pub fn allocate_register(&mut self) -> u8 {
        let register = self.next_register;

        self.next_register += 1;

        register
    }

    fn emit_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn visit_declaration(&mut self, declaration: &Decl) -> Result<(), KaoriError> {
        match &declaration.kind {
            DeclKind::Function { body, parameters } => {
                for parameter in parameters {
                    self.visit_expression(parameter);
                }

                for statement in body {
                    self.visit_statement(statement)?;
                }

                let src = self.constants.push_nil();
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::LoadConst { dest, src });

                self.emit_instruction(Instruction::Return { src: dest });
            }
        };

        Ok(())
    }

    fn visit_statement(&mut self, statement: &Stmt) -> Result<(), KaoriError> {
        match &statement.kind {
            StmtKind::Expression(expression) => {
                self.visit_expression(expression);
            }
            StmtKind::Print(expression) => {
                let src = self.visit_expression(expression);
                let instruction = Instruction::Print { src };
                self.emit_instruction(instruction);
            }
            StmtKind::Block(statements) => {
                for stmt in statements {
                    self.visit_statement(stmt)?;
                }
            }
            StmtKind::Branch {
                condition,
                then_branch,
                else_branch,
            } => {
                let src = self.visit_expression(condition);
            }
            StmtKind::Loop {
                init,
                condition,
                block,
                increment,
            } => {}
            StmtKind::Break => {}
            StmtKind::Continue => {}
            StmtKind::Return(expr) => {
                let src = if let Some(expr) = expr {
                    self.visit_expression(expr)
                } else {
                    let src = self.constants.push_nil();

                    let dest = self.allocate_register();
                    let instruction = Instruction::LoadConst { dest, src };

                    self.emit_instruction(instruction);
                    dest
                };

                self.emit_instruction(Instruction::Return { src });
            }
        };

        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expr) -> u8 {
        match &expression.kind {
            ExprKind::Parameter(id) => {
                let register = self.allocate_register();
                self.registers.insert(*id, register);

                register
            }
            ExprKind::DeclareAssign { id, right } => {
                let dest = self.allocate_register();
                let src = self.visit_expression(right);
                self.registers.insert(*id, dest);

                let instruction = Instruction::Move { dest, src };
                self.emit_instruction(instruction);

                dest
            }
            ExprKind::Assign { left, right } => {
                let dest = self.visit_expression(left);
                let src = self.visit_expression(right);
                let instruction = Instruction::Move { dest, src };
                self.emit_instruction(instruction);

                dest
            }
            ExprKind::LogicalAnd { left, right } => {
                let dest = self.visit_expression(left);

                let true_bb = self.create_bb();
                let false_bb = self.create_bb();

                self.set_terminator(Terminator::Branch {
                    src: dest,
                    r#true: true_bb,
                    r#false: false_bb,
                });

                self.index = true_bb;
                let src = self.visit_expression(right);
                self.emit_instruction(Instruction::Move { dest, src });
                self.set_terminator(Terminator::Goto(false_bb));

                self.index = false_bb;

                dest
            }
            ExprKind::LogicalOr { left, right } => {
                let dest = self.visit_expression(left);

                let true_bb = self.create_bb();
                let false_bb = self.create_bb();

                self.set_terminator(Terminator::Branch {
                    src: dest,
                    r#true: true_bb,
                    r#false: false_bb,
                });

                self.index = false_bb;
                let src = self.visit_expression(right);
                self.emit_instruction(Instruction::Move { dest, src });
                self.set_terminator(Terminator::Goto(true_bb));

                self.index = true_bb;

                dest
            }
            ExprKind::LogicalNot { expr } => {
                let src = self.visit_expression(expr);
                let dest = self.allocate_register();
                let instruction = Instruction::Not { dest, src };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let src1 = self.visit_expression(left);
                let src2 = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    BinaryOpKind::Add => Instruction::Add { dest, src1, src2 },
                    BinaryOpKind::Subtract => Instruction::Subtract { dest, src1, src2 },
                    BinaryOpKind::Multiply => Instruction::Multiply { dest, src1, src2 },
                    BinaryOpKind::Divide => Instruction::Divide { dest, src1, src2 },
                    BinaryOpKind::Modulo => Instruction::Modulo { dest, src1, src2 },
                    BinaryOpKind::Equal => Instruction::Equal { dest, src1, src2 },
                    BinaryOpKind::NotEqual => Instruction::NotEqual { dest, src1, src2 },
                    BinaryOpKind::Greater => Instruction::Greater { dest, src1, src2 },
                    BinaryOpKind::GreaterEqual => Instruction::GreaterEqual { dest, src1, src2 },
                    BinaryOpKind::Less => Instruction::Less { dest, src1, src2 },
                    BinaryOpKind::LessEqual => Instruction::LessEqual { dest, src1, src2 },
                    BinaryOpKind::Power => Instruction::Power { dest, src1, src2 },
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::Unary { right, operator } => {
                let src = self.visit_expression(right);
                let dest = self.allocate_register();

                let instruction = match operator.kind {
                    UnaryOpKind::Negate => Instruction::Negate { dest, src },
                };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::FunctionCall { callee, arguments } => {
                let dest = self.allocate_register();

                let arguments_src = arguments
                    .iter()
                    .map(|argument| self.visit_expression(argument))
                    .collect::<Vec<u8>>();

                let src = self.visit_expression(callee);

                for (dest, src) in arguments_src.iter().copied().enumerate() {
                    let instruction = Instruction::Move {
                        dest: dest as u8,
                        src,
                    };

                    self.emit_instruction(instruction);
                }

                let instruction = Instruction::Call { dest, src };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::MemberAccess { object, property } => {
                let dest = self.allocate_register();

                let object = self.visit_expression(object);
                let key = self.visit_expression(property);
                let instruction = Instruction::GetField { dest, object, key };

                self.emit_instruction(instruction);

                dest
            }
            ExprKind::Variable(id) => *self
                .registers
                .get(id)
                .expect("Variable not found for NodeId"),

            ExprKind::Function(id) => {
                let index = *self
                    .node_to_function
                    .get(id)
                    .expect("FunctionRef points to a missing variable node");

                let src = self.constants.push_function_index(index);
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::String(value) => {
                let src = self.constants.push_string(value.to_owned());
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::Boolean(value) => {
                let src = self.constants.push_boolean(*value);
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }

            ExprKind::Number(value) => {
                let src = self.constants.push_number(*value);
                let dest = self.allocate_register();

                self.emit_instruction(Instruction::LoadConst { dest, src });
                dest
            }
            ExprKind::DictLiteral { fields } => {
                let dest = self.allocate_register();
                let instruction = Instruction::CreateDict { dest };

                self.emit_instruction(instruction);

                for (key, value) in fields {
                    let key = self.visit_expression(key);
                    let value = self.visit_expression(value);

                    let instruction = Instruction::SetField {
                        object: dest,
                        key,
                        value,
                    };
                    self.emit_instruction(instruction);
                }

                dest
            }
        }
    }
}
