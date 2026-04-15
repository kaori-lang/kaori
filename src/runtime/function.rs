use crate::{
    bytecode::{self, instruction::Instruction},
    runtime::value::Value,
};

use super::gc::Gc;

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constants: Vec<Value>,
}

use std::fmt::{self, Display, Formatter};

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (ip, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{:04}  {}", ip, instr)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub fn from_compiled(functions: Vec<bytecode::Function>, gc: &mut Gc) -> Vec<Function> {
    let mut runtime_functions: Vec<Function> = Vec::with_capacity(functions.len());

    for function in functions {
        let bytecode::Function {
            instructions,
            constants,
            registers_count,
        } = function;

        let constants = constants
            .into_iter()
            .map(|constant| match constant {
                bytecode::Constant::Number(value) => Value::number(value),
                bytecode::Constant::Boolean(value) => Value::boolean(value),
                bytecode::Constant::String(value) => gc.allocate_string(&value),
                bytecode::Constant::FunctionIndex(index) => {
                    let ptr = unsafe { runtime_functions.as_ptr().add(index) };

                    Value::function(ptr)
                }
                bytecode::Constant::Nil => Value::default(),
            })
            .collect();

        runtime_functions.push(Function {
            instructions,
            registers_count,
            constants,
        });
    }

    runtime_functions
}
