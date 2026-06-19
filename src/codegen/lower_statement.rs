use crate::{
    codegen::{environment::Environment, lower_ast::Lower, operand::Operand},
    diagnostics::error::Error,
    runtime::{function::Function, instruction::Instruction},
    syntax::{
        ast::{Node, NodeId},
        ops::UnaryOp,
    },
};

impl<'a> Lower<'a> {
    pub fn lower_statement(&mut self, id: NodeId) -> Result<(), Error> {
        match *self.ast.node(id) {
            Node::Variable { left, right } => {
                let dest = self.env.allocate_local();

                self.lower_materializing(right, Some(dest))?;

                self.env.declare_local(left.value, dest);
            }
            Node::Constant { left, right } => {
                let dest = self.env.allocate_local();

                self.lower_materializing(right, Some(dest))?;

                self.env.declare_local(left.value, dest);
            }
            Node::Ref { left, right } => {
                let dest = self.env.allocate_local();

                let src = self.lower_materializing(right, Some(dest))?;

                self.env.declare_local(left.value, dest);

                self.function.emit_instruction(Instruction::CreateRef {
                    dest: dest.into(),
                    src: src.into(),
                });
            }
            Node::Assign { left, right } => match *self.ast.node(left) {
                Node::Identifier(..) => {
                    let dest = self.lower_materializing(left, None)?;

                    let src = self.lower_materializing(right, Some(dest))?;

                    self.env.free_temp(src);
                }
                Node::PropertyAccess { object, property } => {
                    let value = self.lower_expression(right, None)?;
                    let object = self.lower_materializing(object, None)?;
                    let key = self.function.store_string_const(property.value);

                    match value {
                        Operand::Constant(value) => {
                            let value = self.function.store_constant(value);

                            self.function.emit_instruction(Instruction::SetPropertyKK {
                                object: object.into(),
                                key,
                                value,
                            });
                        }
                        Operand::Register(value) => {
                            self.function.emit_instruction(Instruction::SetPropertyKR {
                                object: object.into(),
                                key,
                                value: value.into(),
                            });
                        }
                    }
                }
                Node::Unary { operator: UnaryOp::Deref, operand } => {
                    let dest = self.lower_materializing(operand, None)?;
                    let src = self.lower_materializing(right, None)?;

                    self.function.emit_instruction(Instruction::DerefSet {
                        dest: dest.into(),
                        src: src.into(),
                    });
                }
                _ => {
                    return Err(Error::new(
                        self.ast.span(left),
                        self.compiler.current_file,
                        "expected a valid lhs".to_string(),
                    ));
                }
            },
            Node::WhileLoop { condition, block } => {
                let break_until = self.unpatched_break.len();
                let continue_until = self.unpatched_continue.len();

                self.loop_depth += 1;

                let jump_if_false = self.lower_jump_if_false(condition)?;

                let loop_body = self.function.instructions.len();

                self.lower_statement(block)?;

                let jump_if_true = self.lower_jump_if_true(condition)?;

                self.patch_jump(jump_if_true, loop_body as i32 - jump_if_true as i32);
                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                while self.unpatched_break.len() > break_until {
                    let index =
                        self.unpatched_break.pop().expect("Expected a break instruction index");

                    self.patch_jump(index, self.function.instructions.len() as i32 - index as i32);
                }

                while self.unpatched_continue.len() > continue_until {
                    let index = self
                        .unpatched_continue
                        .pop()
                        .expect("Expected a continue instruction index");

                    self.patch_jump(index, jump_if_true as i32 - index as i32);
                }

                self.loop_depth -= 1;
            }
            Node::If { condition, then_branch, else_branch } => {
                let jump_if_false = self.lower_jump_if_false(condition)?;

                self.lower_statement(then_branch)?;

                let jump_end = self.function.emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32 - jump_if_false as i32,
                );

                if let Some(id) = else_branch {
                    self.lower_statement(id)?;
                }

                self.patch_jump(
                    jump_end,
                    self.function.instructions.len() as i32 - jump_end as i32,
                );
            }
            Node::Return(expression) => {
                let src = self.lower_materializing(expression, None)?;

                self.function.emit_instruction(Instruction::Return { src: src.into() });

                self.env.free_temp(src);
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

                if let Some(id) = tail {
                    self.lower_statement(id)?;
                }

                self.env.pop_scope();
            }
            Node::Function { ref parameters, block, name } => {
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
                            inner_self.compiler.current_file,
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

                let (_, dest) = self.env.lookup(name.value).expect("function must be declared");

                let index = self.compiler.functions.len();
                self.compiler.functions.push(function);

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
            }
            Node::Import { ref path, ref bindings } => {
                let index = self.compiler.compile_file(path)?;

                let object = self.env.allocate_temp();

                self.function.emit_instruction(Instruction::CreateClosure {
                    dest: object.into(),
                    src: index as u32,
                    captures: 0,
                });

                self.function.emit_instruction(Instruction::Call {
                    dest: object.into(),
                    src: object.into(),
                    arity: 0,
                });

                match bindings.is_empty() {
                    true => {
                        let dest = self.env.allocate_local();

                        self.env.declare_local(path.last().unwrap().value, dest);

                        self.function.emit_instruction(Instruction::Move {
                            dest: dest.into(),
                            src: object.into(),
                        });
                    }
                    false => {
                        for binding in bindings.iter().copied() {
                            let dest = self.env.allocate_local();

                            self.env.declare_local(binding.value, dest);

                            let key = self.function.store_string_const(binding.value);

                            self.function.emit_instruction(Instruction::GetPropertyK {
                                dest: dest.into(),
                                object: object.into(),
                                key,
                            });
                        }
                    }
                }
            }
            Node::Break => {
                if self.loop_depth == 0 {
                    return Err(Error::new(
                        self.ast.span(id),
                        self.compiler.current_file,
                        "`break` statement found outside a loop".to_string(),
                    ));
                }

                let index = self.function.instructions.len();

                self.unpatched_break.push(index);

                self.function.emit_instruction(Instruction::Jump { offset: 0 });
            }
            Node::Continue => {
                if self.loop_depth == 0 {
                    return Err(Error::new(
                        self.ast.span(id),
                        self.compiler.current_file,
                        "`continue` statement found outside a loop".to_string(),
                    ));
                }

                let index = self.function.instructions.len();

                self.unpatched_continue.push(index);

                self.function.emit_instruction(Instruction::Jump { offset: 0 });
            }
            _ => {
                let register = self.lower_materializing(id, None)?;
                self.env.free_temp(register);
            }
        }

        Ok(())
    }
}
