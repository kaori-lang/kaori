use crate::{
    codegen::{
        environment::{Environment, Register},
        lower_ast::Lower,
        operand::{Constant, Operand},
    },
    diagnostics::error::Error,
    runtime::{function::Function, instruction::Instruction},
    syntax::{
        ast::{Node, NodeId},
        ops::{BinaryOp, UnaryOp},
    },
};

impl<'a> Lower<'a> {
    pub fn lower_expression(
        &mut self,
        id: NodeId,
        dest: Option<Register>,
    ) -> Result<Operand, Error> {
        let register = match *self.ast.node(id) {
            Node::Number(value) => {
                let constant = Constant::Number(value);

                Operand::Constant(constant)
            }
            Node::String(value) => {
                let constant = Constant::String(value);

                Operand::Constant(constant)
            }
            Node::Boolean(value) => {
                let constant = Constant::Boolean(value);

                Operand::Constant(constant)
            }
            Node::Nil => {
                let constant = Constant::Nil;

                Operand::Constant(constant)
            }
            Node::Identifier(name) => {
                let Some((_, register)) = self.env.lookup(name.value) else {
                    return Err(Error::new(
                        name.span,
                        self.compiler.current_file,
                        "undeclared variable".to_string(),
                    ));
                };

                let register = match dest {
                    Some(dest) if dest == register => dest,
                    Some(dest) => {
                        self.function.emit_instruction(Instruction::Move {
                            dest: dest.into(),
                            src: register.into(),
                        });

                        dest
                    }
                    None => register,
                };

                Operand::Register(register)
            }
            Node::Binary { operator, left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let src1 = self.lower_expression(left, None)?;
                let src2 = self.lower_expression(right, None)?;

                match (src1, src2) {
                    (Operand::Constant(src1), Operand::Constant(src2)) => {
                        // TODO CONST FOLD INSTEAD OF MATERIALIZING LOAD CONST
                        let src1 = self.lower_materializing(left, None)?;
                        let src2 = self.lower_materializing(right, None)?;

                        self.function.emit_instruction(match operator {
                            BinaryOp::Add => Instruction::Add {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Subtract => Instruction::Subtract {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Multiply => Instruction::Multiply {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Divide => Instruction::Divide {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Modulo => Instruction::Modulo {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Equal => Instruction::Equal {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::NotEqual => Instruction::NotEqual {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Less => Instruction::Less {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::LessEqual => Instruction::LessEqual {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Greater => Instruction::Less {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::GreaterEqual => Instruction::LessEqual {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                        });

                        self.env.free_temp(src1);
                        self.env.free_temp(src2);
                    }
                    (Operand::Register(src1), Operand::Constant(src2)) => {
                        let src2 = self.function.store_constant(src2);

                        self.function.emit_instruction(match operator {
                            BinaryOp::Add => {
                                Instruction::AddK { dest: dest.into(), src1: src1.into(), src2 }
                            }
                            BinaryOp::Subtract => Instruction::SubtractRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Multiply => Instruction::MultiplyK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Divide => {
                                Instruction::DivideRK { dest: dest.into(), src1: src1.into(), src2 }
                            }
                            BinaryOp::Modulo => {
                                Instruction::ModuloRK { dest: dest.into(), src1: src1.into(), src2 }
                            }
                            BinaryOp::Equal => {
                                Instruction::EqualK { dest: dest.into(), src1: src1.into(), src2 }
                            }
                            BinaryOp::NotEqual => Instruction::NotEqualK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Less => {
                                Instruction::LessRK { dest: dest.into(), src1: src1.into(), src2 }
                            }
                            BinaryOp::LessEqual => Instruction::LessEqualRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Greater => Instruction::LessKR {
                                dest: dest.into(),
                                src1: src2,
                                src2: src1.into(),
                            },
                            BinaryOp::GreaterEqual => Instruction::LessEqualKR {
                                dest: dest.into(),
                                src1: src2,
                                src2: src1.into(),
                            },
                        });

                        self.env.free_temp(src1);
                    }
                    (Operand::Constant(src1), Operand::Register(src2)) => {
                        let src1 = self.function.store_constant(src1);

                        self.function.emit_instruction(match operator {
                            BinaryOp::Add => Instruction::AddK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1,
                            },
                            BinaryOp::Multiply => Instruction::MultiplyK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1,
                            },
                            BinaryOp::Equal => Instruction::EqualK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1,
                            },
                            BinaryOp::NotEqual => Instruction::NotEqualK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1,
                            },
                            BinaryOp::Subtract => Instruction::SubtractKR {
                                dest: dest.into(),
                                src1,
                                src2: src2.into(),
                            },
                            BinaryOp::Divide => {
                                Instruction::DivideKR { dest: dest.into(), src1, src2: src2.into() }
                            }
                            BinaryOp::Modulo => {
                                Instruction::ModuloKR { dest: dest.into(), src1, src2: src2.into() }
                            }
                            BinaryOp::Less => {
                                Instruction::LessKR { dest: dest.into(), src1, src2: src2.into() }
                            }
                            BinaryOp::LessEqual => Instruction::LessEqualKR {
                                dest: dest.into(),
                                src1,
                                src2: src2.into(),
                            },
                            BinaryOp::Greater => Instruction::LessRK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1,
                            },
                            BinaryOp::GreaterEqual => Instruction::LessEqualRK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1,
                            },
                        });

                        self.env.free_temp(src2);
                    }
                    (Operand::Register(src1), Operand::Register(src2)) => {
                        self.function.emit_instruction(match operator {
                            BinaryOp::Add => Instruction::Add {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Subtract => Instruction::Subtract {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Multiply => Instruction::Multiply {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Divide => Instruction::Divide {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Modulo => Instruction::Modulo {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Equal => Instruction::Equal {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::NotEqual => Instruction::NotEqual {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Less => Instruction::Less {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::LessEqual => Instruction::LessEqual {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Greater => Instruction::Less {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::GreaterEqual => Instruction::LessEqual {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                        });

                        self.env.free_temp(src1);
                        self.env.free_temp(src2);
                    }
                }

                Operand::Register(dest)
            }
            Node::Unary { operator, operand } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_materializing(operand, None)?;

                self.function.emit_instruction(match operator {
                    UnaryOp::Negate => Instruction::Negate { dest: dest.into(), src: src.into() },
                    UnaryOp::Deref => Instruction::Deref { dest: dest.into(), src: src.into() },
                });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::LogicalNot(expression) => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_materializing(expression, None)?;

                self.function
                    .emit_instruction(Instruction::Not { dest: dest.into(), src: src.into() });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::LogicalAnd { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let src = self.lower_materializing(left, Some(dest))?;

                let jump_if_false = self
                    .function
                    .emit_instruction(Instruction::JumpIfFalse { src: src.into(), offset: 0 });

                self.lower_materializing(right, Some(dest))?;

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                Operand::Register(dest)
            }
            Node::LogicalOr { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let src = self.lower_materializing(left, Some(dest))?;

                let jump_if_true = self
                    .function
                    .emit_instruction(Instruction::JumpIfTrue { src: src.into(), offset: 0 });

                self.lower_materializing(right, Some(dest))?;

                self.patch_jump(
                    jump_if_true,
                    self.function.instructions.len() as i32 - jump_if_true as i32,
                );

                Operand::Register(dest)
            }
            Node::If { condition, then_branch, else_branch } => {
                let jump_if_false = self.lower_jump_if_false(condition)?;

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_materializing(then_branch, Some(dest))?;

                let jump_end = self.function.emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                if let Some(id) = else_branch {
                    self.lower_materializing(id, Some(dest))?;
                } else {
                    return Err(Error::new(
                        self.ast.span(id),
                        self.compiler.current_file,
                        "if being used as an expression must have `else` branch".to_string(),
                    ));
                }

                self.patch_jump(
                    jump_end,
                    self.function.instructions.len() as i32 - jump_end as i32,
                );

                Operand::Register(dest)
            }
            Node::Block { ref statements, tail } => {
                self.env.push_scope();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = self.ast.node(id) {
                        let dest = self.env.allocate_local();

                        self.env.declare_local(name.value, dest);

                        let src = self.function.store_nil_const();

                        self.function
                            .emit_instruction(Instruction::LoadConst { dest: dest.into(), src });
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    let dest = self.env.allocate_local();

                    self.env.declare_local(name.value, dest);

                    let src = self.function.store_nil_const();

                    self.function
                        .emit_instruction(Instruction::LoadConst { dest: dest.into(), src });
                }

                for id in statements.iter().copied() {
                    self.lower_statement(id)?;
                }

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                match tail {
                    Some(id) => {
                        self.lower_materializing(id, Some(dest))?;
                    }
                    None => {
                        let src = self.function.store_nil_const();

                        self.function
                            .emit_instruction(Instruction::LoadConst { dest: dest.into(), src });
                    }
                };

                self.env.pop_scope();

                Operand::Register(dest)
            }
            Node::FunctionCall { callee, ref arguments } => {
                for (index, argument) in arguments.iter().enumerate() {
                    let dest = Register::Local(index);

                    self.lower_materializing(*argument, Some(dest))?;

                    let index = self.function.instructions.len() - 1;

                    self.unpatched_arguments.push(index);
                }

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_materializing(callee, None)?;

                self.function.emit_instruction(Instruction::Call {
                    dest: dest.into(),
                    src: src.into(),
                    arity: arguments.len() as u8,
                });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::PropertyAccess { object, property } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let object = self.lower_materializing(object, None)?;
                let key = self.function.store_string_const(property.value);

                self.function.emit_instruction(Instruction::GetPropertyK {
                    dest: dest.into(),
                    object: object.into(),
                    key,
                });

                self.env.free_temp(object);

                Operand::Register(dest)
            }
            Node::Map { ref entries } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::CreateMap { dest: dest.into() });

                for (key, value) in entries.iter().copied() {
                    match (self.ast.node(key), value) {
                        (Node::Identifier(name), None) => {
                            let value = self.lower_materializing(key, None)?;
                            let key = self.function.store_string_const(name.value);

                            self.function.emit_instruction(Instruction::SetPropertyKR {
                                object: dest.into(),
                                key,
                                value: value.into(),
                            });
                        }
                        (Node::Identifier(name), Some(value)) => {
                            let value = self.lower_expression(value, None)?;
                            let key = self.function.store_string_const(name.value);

                            match value {
                                Operand::Constant(value) => {
                                    let value = self.function.store_constant(value);

                                    self.function.emit_instruction(Instruction::SetPropertyKK {
                                        object: dest.into(),
                                        key,
                                        value,
                                    });
                                }
                                Operand::Register(value) => {
                                    self.function.emit_instruction(Instruction::SetPropertyKR {
                                        object: dest.into(),
                                        key,
                                        value: value.into(),
                                    });
                                }
                            }
                        }
                        (_, None) => {
                            return Err(Error::new(
                                self.ast.span(key),
                                self.compiler.current_file,
                                "only identifier keys can omit value".to_string(),
                            ));
                        }
                        (_, Some(value)) => {
                            let key = self.lower_expression(key, None)?;
                            let value = self.lower_expression(value, None)?;

                            match (key, value) {
                                (Operand::Register(key), Operand::Register(value)) => {
                                    self.function.emit_instruction(Instruction::SetProperty {
                                        object: dest.into(),
                                        key: key.into(),
                                        value: value.into(),
                                    });
                                }
                                (Operand::Register(key), Operand::Constant(value)) => {
                                    let value = self.function.store_constant(value);

                                    self.function.emit_instruction(Instruction::SetPropertyRK {
                                        object: dest.into(),
                                        key: key.into(),
                                        value,
                                    });
                                }
                                (Operand::Constant(key), Operand::Register(value)) => {
                                    let key = self.function.store_constant(key);

                                    self.function.emit_instruction(Instruction::SetPropertyKR {
                                        object: dest.into(),
                                        key,
                                        value: value.into(),
                                    });
                                }
                                (Operand::Constant(key), Operand::Constant(value)) => {
                                    let key = self.function.store_constant(key);
                                    let value = self.function.store_constant(value);

                                    self.function.emit_instruction(Instruction::SetPropertyKK {
                                        object: dest.into(),
                                        key,
                                        value,
                                    });
                                }
                            }
                        }
                    };
                }

                Operand::Register(dest)
            }
            Node::Lambda { ref parameters, block } => {
                let mut env = Environment::with_parent(std::mem::take(self.env));
                let mut function = Function::default();

                let mut inner_self = Lower::new(
                    self.ast,
                    self.compiler,
                    self.free_variables,
                    &mut env,
                    &mut function,
                );

                let free_variables = inner_self.analyze_function(id);

                for parameter in parameters.iter().copied() {
                    let dest = inner_self.env.allocate_local();

                    inner_self.env.declare_local(parameter.value, dest);
                }

                for capture in free_variables.iter().copied() {
                    if inner_self.env.lookup_in_parent(capture.value).is_some() {
                        let dest = inner_self.env.allocate_local();

                        inner_self.env.declare_local(capture.value, dest);
                    } else {
                        return Err(Error::new(
                            capture.span,
                            self.compiler.current_file,
                            "undeclared variable".to_string(),
                        ));
                    }
                }

                let src = inner_self.lower_materializing(block, None)?;

                if !inner_self.block_returns(block) {
                    inner_self.function.emit_instruction(Instruction::Return { src: src.into() });
                }

                inner_self.patch_arguments();

                function.frame_size = inner_self.env.frame_size;
                function.arity = parameters.len();

                *self.env = std::mem::take(&mut env.parent.unwrap_or_default());

                let index = self.compiler.functions.len();
                self.compiler.functions.push(function);

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::CreateClosure {
                    dest: dest.into(),
                    src: index as u32,
                    captures: free_variables.len() as u8,
                });

                for capture in free_variables.iter().copied() {
                    let (_, register) = self
                        .env
                        .lookup(capture.value)
                        .expect("name must've been declared to reach this point of the code");

                    self.function
                        .emit_instruction(Instruction::CaptureValue { src: register.into() });
                }

                Operand::Register(dest)
            }
            Node::Import { .. }
            | Node::WhileLoop { .. }
            | Node::Function { .. }
            | Node::Return(..)
            | Node::Break
            | Node::Continue
            | Node::Variable { .. }
            | Node::Constant { .. }
            | Node::Ref { .. }
            | Node::Assign { .. } => {
                self.lower_statement(id)?;

                let src = self.function.store_nil_const();
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst { dest: dest.into(), src });

                Operand::Register(dest)
            }
        };

        Ok(register)
    }
}
