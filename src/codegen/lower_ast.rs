use core::panic;
use std::collections::{HashMap, HashSet};

use crate::{
    codegen::{
        environment::{Environment, Register},
        operand::Operand,
    },
    compiler::Compiler,
    diagnostics::error::Error,
    runtime::{function::Function, instruction::Instruction},
    syntax::{
        ast::{Ast, Node, NodeId, Spanned},
        ops::BinaryOp,
    },
    util::string_interner::Symbol,
};

pub fn lower_ast(ast: Ast, compiler: &mut Compiler) -> Result<usize, Error> {
    let id = ast.last();

    let mut free_variables = HashMap::new();
    let mut env = Environment::new();

    let mut function = Function::default();

    let mut lowerer = Lower::new(&ast, compiler, &mut free_variables, &mut env, &mut function);

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
    pub fn new(
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

    pub fn lower_materializing(
        &mut self,
        id: NodeId,
        dest: Option<Register>,
    ) -> Result<Register, Error> {
        let src = self.lower_expression(id, dest)?;

        Ok(match src {
            Operand::Constant(src) => {
                let dest = dest.unwrap_or_else(|| self.env.allocate_temp());
                let src = self.function.store_constant(src);

                self.function.emit_instruction(Instruction::LoadConst { dest: dest.into(), src });

                dest
            }
            Operand::Register(src) => src,
        })
    }

    pub fn lower_jump_if_false(&mut self, id: NodeId) -> Result<usize, Error> {
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
                            BinaryOp::Equal => {
                                Instruction::JumpIfNotEqualK { src1: src1.into(), src2, offset: 0 }
                            }
                            BinaryOp::NotEqual => {
                                Instruction::JumpIfEqualK { src1: src1.into(), src2, offset: 0 }
                            }
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
                            BinaryOp::Greater => Instruction::JumpIfLessEqualRK {
                                src1: src1.into(),
                                src2,
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessRK { src1: src1.into(), src2, offset: 0 }
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
                            BinaryOp::Greater => Instruction::JumpIfLessEqualKR {
                                src1,
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => {
                                Instruction::JumpIfLessKR { src1, src2: src2.into(), offset: 0 }
                            }
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Constant(_), Operand::Constant(_)) => {
                        let src = self.lower_materializing(id, None)?;
                        Ok(self.function.emit_instruction(Instruction::JumpIfFalse {
                            src: src.into(),
                            offset: 0,
                        }))
                    }
                }
            }
            _ => {
                let src = self.lower_materializing(id, None)?;
                Ok(self
                    .function
                    .emit_instruction(Instruction::JumpIfFalse { src: src.into(), offset: 0 }))
            }
        }
    }

    pub fn lower_jump_if_true(&mut self, id: NodeId) -> Result<usize, Error> {
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
                            BinaryOp::LessEqual => Instruction::JumpIfLessEqual {
                                src1: src1.into(),
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::Greater => Instruction::JumpIfLess {
                                src1: src2.into(),
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => Instruction::JumpIfLessEqual {
                                src1: src2.into(),
                                src2: src1.into(),
                                offset: 0,
                            },
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Register(src1), Operand::Constant(src2)) => {
                        let src2 = self.function.store_constant(src2);

                        Ok(self.function.emit_instruction(match operator {
                            BinaryOp::Equal => {
                                Instruction::JumpIfEqualK { src1: src1.into(), src2, offset: 0 }
                            }
                            BinaryOp::NotEqual => {
                                Instruction::JumpIfNotEqualK { src1: src1.into(), src2, offset: 0 }
                            }
                            BinaryOp::Less => {
                                Instruction::JumpIfLessRK { src1: src1.into(), src2, offset: 0 }
                            }
                            BinaryOp::LessEqual => Instruction::JumpIfLessEqualRK {
                                src1: src1.into(),
                                src2,
                                offset: 0,
                            },
                            BinaryOp::Greater => Instruction::JumpIfLessKR {
                                src1: src2,
                                src2: src1.into(),
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => Instruction::JumpIfLessEqualKR {
                                src1: src2,
                                src2: src1.into(),
                                offset: 0,
                            },
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
                            BinaryOp::NotEqual => Instruction::JumpIfNotEqualK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::Less => {
                                Instruction::JumpIfLessKR { src1, src2: src2.into(), offset: 0 }
                            }
                            BinaryOp::LessEqual => Instruction::JumpIfLessEqualKR {
                                src1,
                                src2: src2.into(),
                                offset: 0,
                            },
                            BinaryOp::Greater => Instruction::JumpIfLessRK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            BinaryOp::GreaterEqual => Instruction::JumpIfLessEqualRK {
                                src1: src2.into(),
                                src2: src1,
                                offset: 0,
                            },
                            _ => unreachable!(),
                        }))
                    }
                    (Operand::Constant(_), Operand::Constant(_)) => {
                        let src = self.lower_materializing(id, None)?;

                        Ok(self.function.emit_instruction(Instruction::JumpIfTrue {
                            src: src.into(),
                            offset: 0,
                        }))
                    }
                }
            }
            _ => {
                let src = self.lower_materializing(id, None)?;
                Ok(self
                    .function
                    .emit_instruction(Instruction::JumpIfTrue { src: src.into(), offset: 0 }))
            }
        }
    }

    pub fn patch_jump(&mut self, index: usize, new_offset: i32) {
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
            | Instruction::JumpIfNotEqualK { offset, .. } => *offset = new_offset,
            _ => {
                panic!("tried to patch a non-jump instruction at index {index}")
            }
        }
    }

    pub fn patch_arguments(&mut self) {
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
                _ => unreachable!("instruction at index {index} has no dest to patch"),
            }
        }
    }

    pub fn block_returns(&self, id: NodeId) -> bool {
        match *self.ast.node(id) {
            Node::Return(..) => true,
            Node::Block { ref statements, tail } => {
                let statements = statements.iter().copied().any(|e| self.block_returns(e));
                let expression = if let Some(id) = tail { self.block_returns(id) } else { false };

                statements || expression
            }
            Node::If { then_branch, else_branch, .. } => {
                let then_returns = self.block_returns(then_branch);
                let else_returns =
                    if let Some(id) = else_branch { self.block_returns(id) } else { false };

                then_returns && else_returns
            }
            _ => false,
        }
    }

    pub fn prevent_return(&self, id: NodeId) -> Result<(), Error> {
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
