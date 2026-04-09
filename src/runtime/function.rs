use crate::{
    bytecode::{self, instruction::Instruction},
    runtime::value::Value,
};

use super::gc::Gc;

pub struct Function {
    pub instructions: Vec<Instruction>,
    pub registers_count: u8,
    pub constant_pool: Vec<Value>,
}

pub fn from_compiled(functions: Vec<bytecode::Function>, gc: &mut Gc) -> Vec<Function> {
    functions
        .into_iter()
        .map(|function| {
            let bytecode::Function {
                instructions,
                constant_pool,
                registers_count,
            } = function;

            let constant_pool = constant_pool
                .into_iter()
                .map(|constant| match constant {
                    bytecode::Constant::Number(value) => Value::number(value),
                    bytecode::Constant::Boolean(value) => Value::boolean(value),
                    bytecode::Constant::String(value) => gc.allocate_string(&value),
                    bytecode::Constant::Function(index) => Value::function(index),
                })
                .collect();

            Function {
                instructions,
                registers_count,
                constant_pool,
            }
        })
        .collect()
}
