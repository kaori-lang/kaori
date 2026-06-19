use core::panic;
use std::collections::{HashMap, HashSet};

use crate::{
    codegen::{
        environment::{Environment, Register},
        operand::{Constant, Operand},
    },
    compiler::Compiler,
    diagnostics::error::Error,
    runtime::{function::Function, instruction::Instruction},
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

    let mut function = Function::default();

    let mut lowerer = Lower::new(
        &ast,
        compiler,
        &mut free_variables,
        &mut env,
        &mut function,
    );

    //lowerer.prevent_return(id)?;

    let src = lowerer.lower_materializing(id, None)?;

    lowerer.patch_arguments();

    function.emit_instruction(Instruction::Return { src: src.into() });

    let index = compiler.functions.len();

    function.frame_size = env.frame_size;

    compiler.functions.push(function);

    Ok(index)
}

pub struct Lower<'a> {
    pub ast: &'a Ast,
    pub compiler: &'a mut Compiler,
    pub free_variables: &'a mut HashMap<NodeId, HashSet<Spanned<Symbol>>>,
    pub env: &'a mut Environment,
    pub function: &'a mut Function,
    pub inside_loop: bool,
    pub unpatched_continue: Vec<usize>,
    pub unpatched_break: Vec<usize>,
    pub loop_depth: usize,
    pub unpatched_arguments: Vec<usize>,
}

impl<'a> Lower<'a> {
    fn new(
        ast: &'a Ast,
        compiler: &'a mut Compiler,
        free_variables: &'a mut HashMap<NodeId, HashSet<Spanned<Symbol>>>,
        env: &'a mut Environment,
        function: &'a mut Function,
    ) -> Self {
        Self {
            ast,
            compiler,
            free_variables,
            env,
            function,
            inside_loop: false,
            unpatched_continue: Vec::new(),
            unpatched_break: Vec::new(),
            loop_depth: 0,
            unpatched_arguments: Vec::new(),
        }
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
                let src = self.function.store_constant(src);

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
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
                    let value = self.lower_materializing(right, None)?;
                    let object = self.lower_materializing(object, None)?;
                    let key = self
                        .function
                        .store_constant(Constant::String(property.value));

                    self.function.emit_instruction(
                        Instruction::SetPropertyKR {
                            object: object.into(),
                            key,
                            value: value.into(),
                        },
                    );
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

                self.patch_jump(
                    jump_if_true,
                    loop_body as i32 - jump_if_true as i32,
                );
                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32
                        - jump_if_false as i32,
                );

                while self.unpatched_break.len() > break_until {
                    let index = self
                        .unpatched_break
                        .pop()
                        .expect("Expected a break instruction index");

                    self.patch_jump(
                        index,
                        self.function.instructions.len() as i32 - index as i32,
                    );
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

                let jump_end = self
                    .function
                    .emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32
                        - jump_if_false as i32,
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

                self.function
                    .emit_instruction(Instruction::Return { src: src.into() });

                self.env.free_temp(src);
            }
            Node::Block { ref statements, tail } => {
                self.env.push_scope();

                for id in statements.iter().copied() {
                    if let Node::Function { name, .. } = self.ast.node(id) {
                        let dest = self.env.allocate_local();
                        self.env.declare_local(name.value, dest);

                        let src = self.function.store_nil_const();

                        self.function.emit_instruction(
                            Instruction::LoadConst { dest: dest.into(), src },
                        );
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    let dest = self.env.allocate_local();
                    self.env.declare_local(name.value, dest);

                    let src = self.function.store_nil_const();

                    self.function.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src,
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
                    inner_self.function.emit_instruction(Instruction::Return {
                        src: src.into(),
                    });
                }

                inner_self.patch_arguments();

                function.frame_size = inner_self.env.frame_size;
                function.arity = parameters.len();

                *self.env = std::mem::take(&mut env.parent.unwrap_or_default());

                let (_, dest) = self
                    .env
                    .lookup(name.value)
                    .expect("function must be declared");

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

                    self.function.emit_instruction(Instruction::CaptureValue {
                        src: register.into(),
                    });
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

                if bindings.is_empty() {
                    let dest = self.env.allocate_local();

                    self.env.declare_local(path.last().unwrap().value, dest);

                    self.function.emit_instruction(Instruction::Move {
                        dest: dest.into(),
                        src: object.into(),
                    });
                } else {
                    for binding in bindings.iter().copied() {
                        let dest = self.env.allocate_local();

                        self.env.declare_local(binding.value, dest);

                        let key =
                            self.function.store_string_const(binding.value);

                        self.function.emit_instruction(
                            Instruction::GetPropertyK {
                                dest: dest.into(),
                                object: object.into(),
                                key,
                            },
                        );
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
                            BinaryOp::Add => Instruction::AddK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
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
                            BinaryOp::Divide => Instruction::DivideRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Modulo => Instruction::ModuloRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Equal => Instruction::EqualK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::NotEqual => Instruction::NotEqualK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
                            BinaryOp::Less => Instruction::LessRK {
                                dest: dest.into(),
                                src1: src1.into(),
                                src2,
                            },
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
                            BinaryOp::GreaterEqual => {
                                Instruction::LessEqualKR {
                                    dest: dest.into(),
                                    src1: src2,
                                    src2: src1.into(),
                                }
                            }
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
                            BinaryOp::Divide => Instruction::DivideKR {
                                dest: dest.into(),
                                src1,
                                src2: src2.into(),
                            },
                            BinaryOp::Modulo => Instruction::ModuloKR {
                                dest: dest.into(),
                                src1,
                                src2: src2.into(),
                            },
                            BinaryOp::Less => Instruction::LessKR {
                                dest: dest.into(),
                                src1,
                                src2: src2.into(),
                            },
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
                            BinaryOp::GreaterEqual => {
                                Instruction::LessEqualRK {
                                    dest: dest.into(),
                                    src1: src2.into(),
                                    src2: src1,
                                }
                            }
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

                self.function.emit_instruction(Instruction::Not {
                    dest: dest.into(),
                    src: src.into(),
                });

                self.env.free_temp(src);

                Operand::Register(dest)
            }
            Node::LogicalAnd { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let src = self.lower_materializing(left, Some(dest))?;

                let jump_if_false =
                    self.function.emit_instruction(Instruction::JumpIfFalse {
                        src: src.into(),
                        offset: 0,
                    });

                self.lower_materializing(right, Some(dest))?;

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32
                        - jump_if_false as i32,
                );

                Operand::Register(dest)
            }
            Node::LogicalOr { left, right } => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                let src = self.lower_materializing(left, Some(dest))?;

                let jump_if_true =
                    self.function.emit_instruction(Instruction::JumpIfTrue {
                        src: src.into(),
                        offset: 0,
                    });

                self.lower_materializing(right, Some(dest))?;

                self.patch_jump(
                    jump_if_true,
                    self.function.instructions.len() as i32
                        - jump_if_true as i32,
                );

                Operand::Register(dest)
            }
            Node::If { condition, then_branch, else_branch } => {
                let jump_if_false = self.lower_jump_if_false(condition)?;

                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.lower_materializing(then_branch, Some(dest))?;

                let jump_end = self
                    .function
                    .emit_instruction(Instruction::Jump { offset: 0 });

                self.patch_jump(
                    jump_if_false,
                    self.function.instructions.len() as i32
                        - jump_if_false as i32,
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

                        self.function.emit_instruction(
                            Instruction::LoadConst { dest: dest.into(), src },
                        );
                    }
                }

                if let Some(id) = tail
                    && let Node::Function { name, .. } = self.ast.node(id)
                {
                    let dest = self.env.allocate_local();

                    self.env.declare_local(name.value, dest);

                    let src = self.function.store_nil_const();

                    self.function.emit_instruction(Instruction::LoadConst {
                        dest: dest.into(),
                        src,
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
                        let src = self.function.store_nil_const();

                        self.function.emit_instruction(
                            Instruction::LoadConst { dest: dest.into(), src },
                        );
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
                let key = self
                    .function
                    .store_constant(Constant::String(property.value));

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

                self.function.emit_instruction(Instruction::CreateMap {
                    dest: dest.into(),
                });

                for (key, value) in entries.iter().copied() {
                    match (self.ast.node(key), value) {
                        (Node::Identifier(name), None) => {
                            let value = self.lower_materializing(key, None)?;
                            let key =
                                self.function.store_string_const(name.value);

                            self.function.emit_instruction(
                                Instruction::SetPropertyKR {
                                    object: dest.into(),
                                    key,
                                    value: value.into(),
                                },
                            );
                        }
                        (_, None) => {
                            todo!() // ERROR
                        }
                        (_, Some(value)) => {
                            let key = self.lower_expression(key, None)?;
                            let value = self.lower_expression(value, None)?;

                            match (key, value) {
                                (
                                    Operand::Register(key),
                                    Operand::Register(value),
                                ) => {
                                    self.function.emit_instruction(
                                        Instruction::SetProperty {
                                            object: dest.into(),
                                            key: key.into(),
                                            value: value.into(),
                                        },
                                    );
                                }
                                (
                                    Operand::Register(key),
                                    Operand::Constant(value),
                                ) => {
                                    let value =
                                        self.function.store_constant(value);

                                    self.function.emit_instruction(
                                        Instruction::SetPropertyRK {
                                            object: dest.into(),
                                            key: key.into(),
                                            value,
                                        },
                                    );
                                }
                                (
                                    Operand::Constant(key),
                                    Operand::Register(value),
                                ) => {
                                    let key = self.function.store_constant(key);

                                    self.function.emit_instruction(
                                        Instruction::SetPropertyKR {
                                            object: dest.into(),
                                            key,
                                            value: value.into(),
                                        },
                                    );
                                }
                                (
                                    Operand::Constant(key),
                                    Operand::Constant(value),
                                ) => {
                                    let key = self.function.store_constant(key);
                                    let value =
                                        self.function.store_constant(value);

                                    self.function.emit_instruction(
                                        Instruction::SetPropertyKK {
                                            object: dest.into(),
                                            key,
                                            value: value,
                                        },
                                    );
                                }
                            }
                        }
                    };
                }

                Operand::Register(dest)
            }
            Node::Lambda { ref parameters, block } => {
                let mut env =
                    Environment::with_parent(std::mem::take(self.env));
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
                    inner_self.function.emit_instruction(Instruction::Return {
                        src: src.into(),
                    });
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

                    self.function.emit_instruction(Instruction::CaptureValue {
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

                let src = self.function.store_nil_const();
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());

                self.function.emit_instruction(Instruction::LoadConst {
                    dest: dest.into(),
                    src,
                });

                Operand::Register(dest)
            }
        };

        Ok(register)
    }

    fn lower_jump_if_false(&mut self, id: NodeId) -> Result<usize, Error> {
        match *self.ast.node(id) {
            Node::Binary {
                operator:
                    operator @ (BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual),
                left,
                right,
            } => {
                let src1 = self.lower_expression(left, None)?;
                let src2 = self.lower_expression(right, None)?;

                match (src1, src2) {
                    (Operand::Register(src1), Operand::Register(src2)) => {
                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => Instruction::JumpIfNotEqual {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::NotEqual => Instruction::JumpIfEqual {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::Less => Instruction::JumpIfLessEqual {
                                src1: src2.into(),
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::LessEqual => Instruction::JumpIfLess {
                                src1: src2.into(),
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::Greater => Instruction::JumpIfLessEqual {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => Instruction::JumpIfLess {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Register(src1), Operand::Constant(src2)) => {
                        let src2 = self.function.store_constant(src2);
                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => Instruction::JumpIfNotEqualK {
                                src1: src1.into(),
                                src2,
                                offset: 0,
                            },
                            BinaryOp::NotEqual => Instruction::JumpIfEqualK {
                                src1: src1.into(),
                                src2,
                                offset: 0,
                            },
                            BinaryOp::Less => Instruction::JumpIfLessEqualKR {
                                src1: src2,
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::LessEqual => Instruction::JumpIfLessKR {
                                src1: src2,
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::Greater => {
                                Instruction::JumpIfLessEqualRK {
                                    src1: src1.into(),
                                    src2,
                                    offset: 0,
                                }
                            }
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessRK {
                                    src1: src1.into(),
                                    src2,
                                    offset: 0,
                                }
                            }
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Constant(src1), Operand::Register(src2)) => {
                        let src1 = self.function.store_constant(src1);
                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => Instruction::JumpIfNotEqualK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::NotEqual => Instruction::JumpIfEqualK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::Less => Instruction::JumpIfLessEqualRK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::LessEqual => Instruction::JumpIfLessRK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::Greater => {
                                Instruction::JumpIfLessEqualKR {
                                    src1,
                                    src2: src2.into(),
                                    offset: 0,
                                }
                            }
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessKR {
                                    src1,
                                    src2: src2.into(),
                                    offset: 0,
                                }
                            }
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Constant(_), Operand::Constant(_)) => {
                        let src = self.lower_materializing(id, None)?;
                        Ok(self.function.emit_instruction(
                            Instruction::JumpIfFalse {
                                src: src.into(),
                                offset: 0,
                            },
                        ))
                    }
                }
            }
            _ => {
                let src = self.lower_materializing(id, None)?;
                Ok(self.function.emit_instruction(Instruction::JumpIfFalse {
                    src: src.into(),
                    offset: 0,
                }))
            }
        }
    }

    fn lower_jump_if_true(&mut self, id: NodeId) -> Result<usize, Error> {
        match *self.ast.node(id) {
            Node::Binary {
                operator:
                    operator @ (BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual),
                left,
                right,
            } => {
                let src1 = self.lower_expression(left, None)?;
                let src2 = self.lower_expression(right, None)?;

                match (src1, src2) {
                    (Operand::Register(src1), Operand::Register(src2)) => {
                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => Instruction::JumpIfEqual {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::NotEqual => Instruction::JumpIfNotEqual {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::Less => Instruction::JumpIfLess {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::LessEqual => {
                                Instruction::JumpIfLessEqual {
                                    src1: src1.into(),
                                    src2: src2.into(),
                                    offset: 0,
                                }
                            }
                            BinaryOp::Greater => Instruction::JumpIfLess {
                                src1: src2.into(),
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessEqual {
                                    src1: src2.into(),
                                    src2: src1.into(),
                                    offset: 0,
                                }
                            }
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Register(src1), Operand::Constant(src2)) => {
                        let src2 = self.function.store_constant(src2);

                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => Instruction::JumpIfEqualK {
                                src1: src1.into(),
                                src2,
                                offset: 0,
                            },
                            BinaryOp::NotEqual => {
                                Instruction::JumpIfNotEqualK {
                                    src1: src1.into(),
                                    src2,
                                    offset: 0,
                                }
                            }
                            BinaryOp::Less => Instruction::JumpIfLessRK {
                                src1: src1.into(),
                                src2,
                                offset: 0,
                            },
                            BinaryOp::LessEqual => {
                                Instruction::JumpIfLessEqualRK {
                                    src1: src1.into(),
                                    src2,
                                    offset: 0,
                                }
                            }
                            BinaryOp::Greater => Instruction::JumpIfLessKR {
                                src1: src2,
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessEqualKR {
                                    src1: src2,
                                    src2: src1.into(),
                                    offset: 0,
                                }
                            }
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Constant(src1), Operand::Register(src2)) => {
                        let src1 = self.function.store_constant(src1);
                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => Instruction::JumpIfEqualK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::NotEqual => {
                                Instruction::JumpIfNotEqualK {
                                    src1: src2.into(),
                                    src2: src1,
                                    offset: 0,
                                }
                            }
                            BinaryOp::Less => Instruction::JumpIfLessKR {
                                src1,
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::LessEqual => {
                                Instruction::JumpIfLessEqualKR {
                                    src1,
                                    src2: src2.into(),
                                    offset: 0,
                                }
                            }
                            BinaryOp::Greater => Instruction::JumpIfLessRK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessEqualRK {
                                    src1: src2.into(),
                                    src2: src1,
                                    offset: 0,
                                }
                            }
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Constant(_), Operand::Constant(_)) => {
                        let src = self.lower_materializing(id, None)?;
                        Ok(self.function.emit_instruction(
                            Instruction::JumpIfTrue {
                                src: src.into(),
                                offset: 0,
                            },
                        ))
                    }
                }
            }
            _ => {
                let src = self.lower_materializing(id, None)?;
                Ok(self.function.emit_instruction(Instruction::JumpIfTrue {
                    src: src.into(),
                    offset: 0,
                }))
            }
        }
    }

    fn patch_jump(&mut self, index: usize, new_offset: i32) {
        match &mut self.function.instructions[index] {
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
            match &mut self.function.instructions[index] {
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
                | Instruction::GetProperty { dest, .. }
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
