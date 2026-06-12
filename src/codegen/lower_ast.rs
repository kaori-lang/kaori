use core::panic;
use std::collections::{HashMap, HashSet};

use crate::{
    codegen::{
        environment::{Environment, Register},
        operand::{Constant, Operand},
    },
    compiler::Compiler,
    diagnostics::error::Error,
    runtime::{function::Function, instruction::Instruction, value::Value},
    syntax::{
        ast::{Ast, Node, NodeId, Spanned},
        ops::{BinaryOp, UnaryOp},
    },
    util::string_interner::Symbol,
};

pub fn lower_ast(ast: Ast, compiler: &mut Compiler) -> Result<usize, Error> {
    let id = ast.last();

    let mut free_variables = HashMap::new();
    let mut env = Environment::new();

    let mut constants = Vec::new();
    let mut instructions = Vec::new();

    let mut lowerer = Lower::new(
        &ast,
        compiler,
        &mut free_variables,
        &mut env,
        &mut constants,
        &mut instructions,
    );

    //lowerer.prevent_return(id)?;

    let src = lowerer.lower_materializing(id, None)?;

    lowerer.emit_instruction(Instruction::Return { src: src.into() });

    lowerer.patch_arguments();

    let index = compiler.functions.len();

    let function = Function {
        instructions,
        constants,
        frame_size: env.frame_size,
        arity: 0,
    };

    compiler.functions.push(function);

    Ok(index)
}

pub struct Lower<'a> {
    pub ast: &'a Ast,
    pub compiler: &'a mut Compiler,
    pub free_variables: &'a mut HashMap<NodeId, HashSet<Spanned<Symbol>>>,
    pub env: &'a mut Environment,
    pub constants: &'a mut Vec<Value>,
    pub instructions: &'a mut Vec<Instruction>,
    pub unpatched_arguments: Vec<usize>,
}

impl<'a> Lower<'a> {
    fn new(
        ast: &'a Ast,
        compiler: &'a mut Compiler,
        free_variables: &'a mut HashMap<NodeId, HashSet<Spanned<Symbol>>>,
        env: &'a mut Environment,
        constants: &'a mut Vec<Value>,
        instructions: &'a mut Vec<Instruction>,
    ) -> Self {
        Self {
            ast,
            compiler,
            free_variables,
            env,
            constants,
            instructions,
            unpatched_arguments: Vec::new(),
        }
    }

    pub fn emit_instruction(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();

        self.instructions.push(instruction);

        index
    }

    fn lower_materializing(
        &mut self,
        id: NodeId,
        dest: Option<Register>,
    ) -> Result<Register, Error> {
        let src = self.lower_expression(id, dest)?;

        Ok(match src {
            Operand::Constant(src) => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.store_constant(src);

                self.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src: src.into(),
                });

                dest
            }
            Operand::Register(src) => src,
        })
    }

    fn lower_statement(&mut self, id: NodeId) -> Result<(), Error> {
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

                self.emit_instruction(Instruction::CreateRef {
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
                Node::MemberAccess { object, property } => {
                    let value = self.lower_materializing(right, None)?;
                    let object = self.lower_materializing(object, None)?;
                    let key = self.lower_materializing(property, None)?;

                    self.emit_instruction(Instruction::SetField {
                        object: object.into(),
                        key: key.into(),
                        value: value.into(),
                    });
                }
                Node::Unary { operator: UnaryOp::Deref, operand } => {
                    let dest = self.lower_materializing(operand, None)?;
                    let src = self.lower_materializing(right, None)?;

                    self.emit_instruction(Instruction::DerefSet {
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
                let src = self.lower_materializing(condition, None)?;

                let jump_if_false = self.lower_jump_if_false(src);
                self.env.free_temp(src);

                let loop_body = self.instructions.len();

                self.lower_statement(block)?;

                let src = self.lower_materializing(condition, None)?;

                let jump_if_true = self.lower_jump_if_true(src);
                self.env.free_temp(src);

                self.patch_jump(
                    jump_if_true,
                    loop_body as i32 - jump_if_true as i32,
                );
                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );
            }
            Node::If { condition, then_branch, else_branch } => {
                let src = self.lower_materializing(condition, None)?;

                self.env.free_temp(src);

                let jump_if_false = self.lower_jump_if_false(src);

                self.lower_statement(then_branch)?;

                let jump_end =
                    self.emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                if let Some(id) = else_branch {
                    self.lower_statement(id)?;
                }

                self.patch_jump(
                    jump_end,
                    self.instructions.len() as i32 - jump_end as i32,
                );
            }
            Node::Return(expression) => {
                let src = self.lower_materializing(expression, None)?;

                self.emit_instruction(Instruction::Return { src: src.into() });

                self.env.free_temp(src);
            }
            Node::Block { ref statements, tail } => {
                self.env.push_scope();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = self.ast.node(id) {
                        let dest = self.env.allocate_local();
                        self.env.declare_local(name.value, dest);

                        let src = self.store_nil_const();

                        self.emit_instruction(Instruction::LoadConst {
                            dest: dest.into(),
                            src: src.into(),
                        });
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    let dest = self.env.allocate_local();
                    self.env.declare_local(name.value, dest);

                    let src = self.store_nil_const();

                    self.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src: src.into(),
                    });
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
                let mut env =
                    Environment::with_parent(std::mem::take(self.env));

                let mut constants = Vec::new();
                let mut instructions = Vec::new();

                let mut inner_self = Lower::new(
                    self.ast,
                    self.compiler,
                    self.free_variables,
                    &mut env,
                    &mut constants,
                    &mut instructions,
                );

                let free_variables = inner_self.analyze_function(id);

                for parameter in parameters.iter().copied() {
                    let dest = inner_self.env.allocate_local();

                    inner_self.env.declare_local(parameter.value, dest);
                }

                for capture in free_variables.iter().copied() {
                    if inner_self.env.lookup_in_parent(capture.value).is_some()
                    {
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
                    inner_self.emit_instruction(Instruction::Return {
                        src: src.into(),
                    });
                }

                inner_self.patch_arguments();

                let frame_size = inner_self.env.frame_size;
                let function = Function {
                    instructions,
                    constants,
                    frame_size,
                    arity: parameters.len(),
                };

                *self.env = std::mem::take(&mut env.parent.unwrap_or_default());

                let (_, dest) = self
                    .env
                    .lookup(name.value)
                    .expect("function must be declared");

                let index = self.compiler.functions.len();
                self.compiler.functions.push(function);

                self.emit_instruction(Instruction::CreateClosure {
                    dest: dest.into(),
                    src: index as u32,
                    captures: free_variables.len() as u8,
                });

                for capture in free_variables.iter().copied() {
                    let (_, register) = self
                        .env
                        .lookup(capture.value)
                        .expect("name must've been declared to reach this point of the code");

                    self.emit_instruction(Instruction::CaptureValue {
                        src: register.into(),
                    });
                }
            }
            Node::Import { ref path, ref bindings } => {
                let index = self.compiler.compile_file(path)?;

                let object = self.env.allocate_temp();

                self.emit_instruction(Instruction::CreateClosure {
                    dest: object.into(),
                    src: index as u32,
                    captures: 0,
                });

                self.emit_instruction(Instruction::Call {
                    dest: object.into(),
                    src: object.into(),
                });

                if bindings.is_empty() {
                    let dest = self.env.allocate_local();

                    self.env.declare_local(path.last().unwrap().value, dest);

                    self.emit_instruction(Instruction::Move {
                        dest: dest.into(),
                        src: object.into(),
                    });
                } else {
                    for binding in bindings.iter().copied() {
                        let dest = self.env.allocate_local();

                        self.env.declare_local(binding.value, dest);

                        let key = {
                            let src = self.store_string_const(binding.value);
                            let dest = self.env.allocate_temp();

                            self.emit_instruction(Instruction::LoadConst {
                                dest: dest.into(),
                                src: src.into(),
                            });

                            dest
                        };

                        self.emit_instruction(Instruction::GetField {
                            dest: dest.into(),
                            object: object.into(),
                            key: key.into(),
                        });

                        self.env.free_temp(key);
                    }
                }
            }
            Node::Break => todo!(),
            Node::Continue => todo!(),
            _ => {
                let register = self.lower_materializing(id, None)?;
                self.env.free_temp(register);
            }
        }

        Ok(())
    }

    fn lower_expression(
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
                        self.emit_instruction(Instruction::Move {
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

                        self.emit_instruction(match operator {
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
                        let src2 = self.store_constant(src2);

                        self.emit_instruction(match operator {
                            BinaryOp::Add => Instruction::AddK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Subtract => Instruction::SubtractRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Multiply => Instruction::MultiplyK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Divide => Instruction::DivideRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Modulo => Instruction::ModuloRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Equal => Instruction::EqualK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::NotEqual => Instruction::NotEqualK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Less => Instruction::LessRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::LessEqual => Instruction::LessEqualRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Greater => Instruction::LessKR {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::LessEqualKR {
                                    dest: dest.into(),
                                    src1: src2.into(),
                                    src2: src1.into(),
                                }
                            }
                        });

                        self.env.free_temp(src1);
                    }
                    (Operand::Constant(src1), Operand::Register(src2)) => {
                        let src1 = self.store_constant(src1);

                        self.emit_instruction(match operator {
                            BinaryOp::Add => Instruction::AddK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::Multiply => Instruction::MultiplyK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::Equal => Instruction::EqualK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::NotEqual => Instruction::NotEqualK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::Subtract => Instruction::SubtractKR {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Divide => Instruction::DivideKR {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Modulo => Instruction::ModuloKR {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Less => Instruction::LessKR {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::LessEqual => Instruction::LessEqualKR {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2: src2.into(),
                            },
                            BinaryOp::Greater => Instruction::LessRK {
                                dest: dest.into(),
                                src1: src2.into(),
                                src2: src1.into(),
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::LessEqualRK {
                                    dest: dest.into(),
                                    src1: src2.into(),
                                    src2: src1.into(),
                                }
                            }
                        });

                        self.env.free_temp(src2);
                    }
                    (Operand::Register(src1), Operand::Register(src2)) => {
                        self.emit_instruction(match operator {
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

                self.emit_instruction(match operator {
                    UnaryOp::Negate => Instruction::Negate {
                        dest: dest.into(),
                        src: src.into(),
                    },
                    UnaryOp::Deref => Instruction::Deref {
                        dest: dest.into(),
                        src: src.into(),
                    },
                });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::LogicalNot(expression) => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_materializing(expression, None)?;

                self.emit_instruction(Instruction::Not {
                    dest: dest.into(),
                    src: src.into(),
                });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::LogicalAnd { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_materializing(left, Some(dest))?;

                let jump_if_false = self.lower_jump_if_false(dest);

                self.lower_materializing(right, Some(dest))?;

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
                );

                Operand::Register(dest)
            }
            Node::LogicalOr { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_materializing(left, Some(dest))?;

                let jump_if_true = self.lower_jump_if_true(dest);

                self.lower_materializing(right, Some(dest))?;

                self.patch_jump(
                    jump_if_true,
                    self.instructions.len() as i32 - jump_if_true as i32,
                );

                Operand::Register(dest)
            }
            Node::If { condition, then_branch, else_branch } => {
                let src = self.lower_materializing(condition, None)?;
                self.env.free_temp(src);

                let jump_if_false = self.lower_jump_if_false(src);

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_materializing(then_branch, Some(dest))?;

                let jump_end =
                    self.emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.instructions.len() as i32 - jump_if_false as i32,
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
                    self.instructions.len() as i32 - jump_end as i32,
                );

                Operand::Register(dest)
            }
            Node::Block { ref statements, tail } => {
                self.env.push_scope();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = self.ast.node(id) {
                        let dest = self.env.allocate_local();

                        self.env.declare_local(name.value, dest);

                        let src = self.store_nil_const();

                        self.emit_instruction(Instruction::LoadConst {
                            dest: dest.into(),
                            src: src.into(),
                        });
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    let dest = self.env.allocate_local();

                    self.env.declare_local(name.value, dest);

                    let src = self.store_nil_const();

                    self.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src: src.into(),
                    });
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
                        let src = self.store_nil_const();

                        self.emit_instruction(Instruction::LoadConst {
                            dest: dest.into(),
                            src: src.into(),
                        });
                    }
                };

                self.env.pop_scope();

                Operand::Register(dest)
            }
            Node::FunctionCall { callee, ref arguments } => {
                for (index, argument) in arguments.iter().enumerate() {
                    let dest = Register::Local(index);

                    self.lower_materializing(*argument, Some(dest))?;

                    let index = self.instructions.len() - 1;

                    self.unpatched_arguments.push(index);
                }

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.lower_materializing(callee, None)?;

                self.emit_instruction(Instruction::Call {
                    dest: dest.into(),
                    src: src.into(),
                });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::MemberAccess { object, property } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let object = self.lower_materializing(object, None)?;
                let key = self.lower_materializing(property, None)?;

                self.emit_instruction(Instruction::GetField {
                    dest: dest.into(),
                    object: object.into(),
                    key: key.into(),
                });

                self.env.free_temp(object);
                self.env.free_temp(key);

                Operand::Register(dest)
            }
            Node::Map { ref entries } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.emit_instruction(Instruction::CreateMap {
                    dest: dest.into(),
                });

                for (key_id, value_id) in entries.iter().copied() {
                    let key = if let Node::Identifier(symbol) =
                        *self.ast.node(key_id)
                    {
                        let dest = self.env.allocate_temp();
                        let src = self.store_string_const(symbol.value);

                        self.emit_instruction(Instruction::LoadConst {
                            dest: dest.into(),
                            src: src.into(),
                        });

                        dest
                    } else if value_id.is_none() {
                        return Err(Error::new(
                            self.ast.span(key_id),
                            self.compiler.current_file,
                            "expected a value for that key, only identifier keys can omit value"
                                .to_string(),
                        ));
                    } else {
                        self.lower_materializing(key_id, None)?
                    };

                    let value = if let Some(id) = value_id {
                        self.lower_materializing(id, None)
                    } else {
                        self.lower_materializing(key_id, None)
                    }?;

                    self.emit_instruction(Instruction::SetField {
                        object: dest.into(),
                        key: key.into(),
                        value: value.into(),
                    });

                    self.env.free_temp(key);
                    self.env.free_temp(value);
                }

                Operand::Register(dest)
            }
            Node::Lambda { ref parameters, block } => {
                let mut env =
                    Environment::with_parent(std::mem::take(self.env));
                let mut constants = Vec::new();
                let mut instructions = Vec::new();

                let mut inner_self = Lower::new(
                    self.ast,
                    self.compiler,
                    self.free_variables,
                    &mut env,
                    &mut constants,
                    &mut instructions,
                );

                let free_variables = inner_self.analyze_function(id);

                for parameter in parameters.iter().copied() {
                    let dest = inner_self.env.allocate_local();

                    inner_self.env.declare_local(parameter.value, dest);
                }

                for capture in free_variables.iter().copied() {
                    if inner_self.env.lookup_in_parent(capture.value).is_some()
                    {
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
                    inner_self.emit_instruction(Instruction::Return {
                        src: src.into(),
                    });
                }

                inner_self.patch_arguments();

                let frame_size = inner_self.env.frame_size;
                let function = Function {
                    instructions,
                    constants,
                    frame_size,
                    arity: parameters.len(),
                };

                *self.env = std::mem::take(&mut env.parent.unwrap_or_default());

                let index = self.compiler.functions.len();
                self.compiler.functions.push(function);

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.emit_instruction(Instruction::CreateClosure {
                    dest: dest.into(),
                    src: index as u32,
                    captures: free_variables.len() as u8,
                });

                for capture in free_variables.iter().copied() {
                    let (_, register) = self
                        .env
                        .lookup(capture.value)
                        .expect("name must've been declared to reach this point of the code");

                    self.emit_instruction(Instruction::CaptureValue {
                        src: register.into(),
                    });
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

                let src = self.store_nil_const();
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src: src.into(),
                });

                Operand::Register(dest)
            }
        };

        Ok(register)
    }

    fn lower_jump_if_true(&mut self, register: Register) -> usize {
        let last = self.instructions.last().copied();

        match last {
            Some(Instruction::Equal { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqual { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::Less { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfLess {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqual { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfLessEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::EqualK { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfEqualK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqualK { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfNotEqualK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessRK { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfLessRK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqualRK { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfLessEqualRK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessKR { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfLessKR {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqualKR { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfLessEqualKR {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            _ => self.emit_instruction(Instruction::JumpIfTrue {
                src: register.into(),
                offset: 0,
            }),
        }
    }

    fn lower_jump_if_false(&mut self, register: Register) -> usize {
        let last = self.instructions.last().copied();

        match last {
            Some(Instruction::Equal { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfNotEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqual { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfEqual {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::Less { src1, src2, .. }) => {
                self.instructions.pop();

                self.emit_instruction(Instruction::JumpIfLessEqual {
                    src1: src2,
                    src2: src1,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqual { src1, src2, .. }) => {
                self.instructions.pop();

                self.emit_instruction(Instruction::JumpIfLess {
                    src1: src2,
                    src2: src1,
                    offset: 0,
                })
            }
            Some(Instruction::EqualK { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfNotEqualK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::NotEqualK { src1, src2, .. }) => {
                self.instructions.pop();
                self.emit_instruction(Instruction::JumpIfEqualK {
                    src1,
                    src2,
                    offset: 0,
                })
            }
            Some(Instruction::LessRK { src1, src2, .. }) => {
                self.instructions.pop();

                self.emit_instruction(Instruction::JumpIfLessEqualKR {
                    src1: src2,
                    src2: src1,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqualRK { src1, src2, .. }) => {
                self.instructions.pop();

                self.emit_instruction(Instruction::JumpIfLessKR {
                    src1: src2,
                    src2: src1,
                    offset: 0,
                })
            }
            Some(Instruction::LessKR { src1, src2, .. }) => {
                self.instructions.pop();

                self.emit_instruction(Instruction::JumpIfLessEqualRK {
                    src1: src2,
                    src2: src1,
                    offset: 0,
                })
            }
            Some(Instruction::LessEqualKR { src1, src2, .. }) => {
                self.instructions.pop();

                self.emit_instruction(Instruction::JumpIfLessRK {
                    src1: src2,
                    src2: src1,
                    offset: 0,
                })
            }
            _ => self.emit_instruction(Instruction::JumpIfFalse {
                src: register.into(),
                offset: 0,
            }),
        }
    }

    fn patch_jump(&mut self, index: usize, new_offset: i32) {
        match &mut self.instructions[index] {
            Instruction::Jump { offset }
            | Instruction::JumpIfTrue { offset, .. }
            | Instruction::JumpIfFalse { offset, .. }
            | Instruction::JumpIfEqual { offset, .. }
            | Instruction::JumpIfNotEqual { offset, .. }
            | Instruction::JumpIfLess { offset, .. }
            | Instruction::JumpIfLessRK { offset, .. }
            | Instruction::JumpIfLessKR { offset, .. }
            | Instruction::JumpIfLessEqual { offset, .. }
            | Instruction::JumpIfLessEqualRK { offset, .. }
            | Instruction::JumpIfLessEqualKR { offset, .. }
            | Instruction::JumpIfEqualK { offset, .. }
            | Instruction::JumpIfNotEqualK { offset, .. } => {
                *offset = new_offset
            }
            _ => {
                panic!("tried to patch a non-jump instruction at index {index}")
            }
        }
    }

    fn patch_arguments(&mut self) {
        let frame_size = self.env.frame_size;

        for index in self.unpatched_arguments.iter().copied() {
            match &mut self.instructions[index] {
                Instruction::Add { dest, .. }
                | Instruction::AddK { dest, .. }
                | Instruction::Subtract { dest, .. }
                | Instruction::SubtractRK { dest, .. }
                | Instruction::SubtractKR { dest, .. }
                | Instruction::Multiply { dest, .. }
                | Instruction::MultiplyK { dest, .. }
                | Instruction::Divide { dest, .. }
                | Instruction::DivideRK { dest, .. }
                | Instruction::DivideKR { dest, .. }
                | Instruction::Modulo { dest, .. }
                | Instruction::ModuloRK { dest, .. }
                | Instruction::ModuloKR { dest, .. }
                | Instruction::Equal { dest, .. }
                | Instruction::EqualK { dest, .. }
                | Instruction::NotEqual { dest, .. }
                | Instruction::NotEqualK { dest, .. }
                | Instruction::Less { dest, .. }
                | Instruction::LessRK { dest, .. }
                | Instruction::LessKR { dest, .. }
                | Instruction::LessEqual { dest, .. }
                | Instruction::LessEqualRK { dest, .. }
                | Instruction::LessEqualKR { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::Negate { dest, .. }
                | Instruction::Move { dest, .. }
                | Instruction::LoadConst { dest, .. }
                | Instruction::CreateMap { dest }
                | Instruction::GetField { dest, .. }
                | Instruction::CreateClosure { dest, .. }
                | Instruction::CreateRef { dest, .. }
                | Instruction::Deref { dest, .. }
                | Instruction::Call { dest, .. } => {
                    let new_dest = Register::Temp(dest.0 as usize + frame_size);
                    *dest = new_dest.into();
                }
                _ => unreachable!(
                    "instruction at index {index} has no dest to patch"
                ),
            }
        }
    }

    fn block_returns(&self, id: NodeId) -> bool {
        match *self.ast.node(id) {
            Node::Return(..) => true,
            Node::Block { ref statements, tail } => {
                let statements =
                    statements.iter().copied().any(|e| self.block_returns(e));
                let expression = if let Some(id) = tail {
                    self.block_returns(id)
                } else {
                    false
                };

                statements || expression
            }
            Node::If { then_branch, else_branch, .. } => {
                let then_returns = self.block_returns(then_branch);
                let else_returns = if let Some(id) = else_branch {
                    self.block_returns(id)
                } else {
                    false
                };

                then_returns && else_returns
            }
            _ => false,
        }
    }

    fn prevent_return(&self, id: NodeId) -> Result<(), Error> {
        match *self.ast.node(id) {
            Node::Return(..) => {
                return Err(Error::new(
                    self.ast.span(id),
                    self.compiler.current_file,
                    "return is not allowed in the global scope".to_string(),
                ));
            }
            Node::Block { ref statements, tail } => {
                for id in statements.iter().copied() {
                    self.prevent_return(id)?;
                }

                if let Some(id) = tail {
                    self.prevent_return(id)?;
                }
            }
            Node::If { then_branch, else_branch, .. } => {
                self.prevent_return(then_branch)?;

                if let Some(id) = else_branch {
                    self.prevent_return(id)?;
                }
            }
            Node::WhileLoop { block, .. } => self.prevent_return(block)?,
            _ => {}
        };

        Ok(())
    }
}
