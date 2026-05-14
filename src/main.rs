#[allow(unused_imports)]
use std::{env::args, process::ExitCode};
#[allow(unused_imports)]
use std::{fs, time::Instant};

use clap::{Arg, Command};

use kaori::program::run_program;
use std::path::PathBuf;

/* fn main() {
    let matches = Command::new("kaori")
        .arg(Arg::new("file").required(true))
        .get_matches();

    let file: PathBuf = matches.get_one::<String>("file").unwrap().into();

    match fs::read_to_string(&file) {
        Ok(source) => {
            if let Err(error) = run_program(&source) {
                error.report(&source);
            }
        }
        Err(_) => eprintln!("Error: Could not read the file by the given path."),
    };
} */

fn main() {
    let source = fs::read_to_string("main.kr").expect("could not read main.kr");

    if let Err(error) = run_program(&source) {
        error.report(&source);
    }
}

fn compile_block(
    &self,
    functions: &mut Vec<Function>,
    function: &mut Function,
    scope: &mut Scope,
    registers: &mut Registers,
    expressions: &[ExprId],
) -> Register {
    for expression in expressions.iter().copied() {
        let expression = self.ast.get(expression);

        if let Expr::Function { name, .. } = &expression
            && let Some(name) = name
        {
            self.compile_expression(functions, function, scope, registers, *name);
        }
    }

    expressions
        .iter()
        .copied()
        .fold(Self::nil(function, registers), |_, expression| {
            self.compile_expression(functions, function, scope, registers, expression)
        })
}

fn compile_expression(
    &self,
    functions: &mut Vec<Function>,
    function: &mut Function,
    scope: &mut Scope,
    registers: &mut Registers,
    expression: ExprId,
) -> Register {
    match *self.ast.get(expression) {
        Expr::NativeFunction { .. } => {
            todo!()
        }
        Expr::Function {
            ref parameters,
            block,
            name,
        } => {
            let index = functions.len();

            let function = Function::default();
            functions.push(function);

            let function = functions.last_mut().unwrap();

            let dest = match name {
                Some(name) => self.compile_expression(functions, function, scope, registers, name),
                None => registers.allocate_register(),
            };

            function.emit_instruction(Instruction::CreateClosure {
                dest,
                src: index as u32,
            });

            for capture in self.captures.get(&expression).unwrap().iter().copied() {
                let src = Self::lookup_or_declare(scope, registers, capture);

                function.emit_instruction(Instruction::CaptureValue { dest, src });
            }

            let mut scope = Scope::default();

            scope.enter_scope();

            for parameter in parameters.iter().copied() {
                self.compile_expression(functions, function, &mut scope, registers, parameter);
            }

            for capture in self.captures.get(&expression).unwrap().iter().copied() {
                Self::lookup_or_declare(&mut scope, registers, capture);
            }

            let src = self.compile_expression(functions, function, &mut scope, registers, block);

            if !self.expression_returns(block) {
                function.emit_instruction(Instruction::Return { src });
            }

            patch_function_arguments(function);

            scope.exit_scope();

            dest
        }
    }
}
