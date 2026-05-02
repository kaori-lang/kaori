use std::{
    sync::{LazyLock, Mutex, OnceLock},
    time::Instant,
};

use logos::Logos;

use crate::{
    bytecode::{
        Function, emit_bytecode::compile, function_scope::Constant,
        optimize_bytecode::optimize_bytecode,
    },
    diagnostics::error::Error,
    runtime::{value::Value, vm::Vm},
    syntax::{parser::Parser, token::Token},
    util::string_interner::StringInterner,
};

pub static INTERNER: LazyLock<Mutex<StringInterner>> =
    LazyLock::new(|| Mutex::new(StringInterner::default()));

pub static FUNCTIONS: OnceLock<Vec<Function<Value>>> = OnceLock::new();

pub fn compile_source_code(source: &str) -> Result<Vec<Function<Constant>>, Error> {
    let tokens = Token::lexer(source).spanned();
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let mut bytecode = compile(&ast)?;

    optimize_bytecode(&mut bytecode);

    for function in bytecode.iter() {
        println!("{}", function);
    }

    Ok(bytecode)
}

fn initialize_functions_table(functions: Vec<Function<Constant>>) -> Vec<Function<Value>> {
    functions
        .into_iter()
        .map(|function| {
            let Function {
                instructions,
                registers_count,
                constants,
                parameters,
                captures,
            } = function;

            let constants = constants
                .into_iter()
                .map(|constant| match constant {
                    Constant::Number(value) => Value::number(value),
                    Constant::String(index) => Value::string(index),
                    Constant::FunctionIndex(index) => Value::function(index),
                })
                .collect();

            Function {
                instructions,
                registers_count,
                constants,
                parameters,
                captures,
            }
        })
        .collect()
}

pub fn run_program(source: &str) -> Result<(), Error> {
    let bytecode = compile_source_code(source)?;

    FUNCTIONS.set(initialize_functions_table(bytecode)).unwrap();

    let start = Instant::now();

    let mut vm = Vm::new();
    vm.run()?;

    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);

    Ok(())
}
