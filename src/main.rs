use std::env::args;
use std::process::ExitCode;

// TODO: remove this line suppression after using the import.
#[allow(unused_imports)]
use std::{fs, time::Instant};

use kaori::{
    compiler::compile_source_code, error::kaori_error::KaoriError,
    virtual_machine::interpreter::Interpreter,
};

fn main() -> ExitCode {
    let source_to_run = Some("test_suite/test.kr".to_owned()); // Debug purposes
    //let source_to_run = args().nth(1);

    if source_to_run.is_none() {
        eprintln!("Error: No path was found for the program's source!");
        return ExitCode::FAILURE;
    }

    let source_path = source_to_run.unwrap();
    println!("{source_path}");
    if let Ok(source) = fs::read_to_string(source_path) {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
            return ExitCode::FAILURE;
        }

        return ExitCode::SUCCESS;
    }

    eprintln!("Error: Could not read the file by the given path.");
    ExitCode::FAILURE
}

// TODO: remove the lint suppressions after using unused variables.
#[allow(unused_variables)]
pub fn run_program(source: String) -> Result<(), KaoriError> {
    let bytecode = compile_source_code(source)?;

    let mut interpreter = Interpreter::new(bytecode.instructions, bytecode.constant_pool);

    let start = Instant::now();

    interpreter.execute_instructions()?;

    println!("Vm executed in: {:#?}", start.elapsed());
    Ok(())
}

/*
HirStmtKind::Loop {
    init,
    condition,
    block,
} => {
    if let Some(init) = init {
        self.visit_declaration(init);
    }
    let previous_bb = self.current_bb;
    let condition_bb = self.create_bb();
    let block_bb = self.create_bb();
    let terminator_bb = self.create_bb();

    self.current_bb = condition_bb;
    let src = self.visit_expression(condition);

    self.current_bb = block_bb;
    self.visit_statement(block);

    self.set_terminator(previous_bb, Terminator::Goto(condition_bb));

    self.set_terminator(
        condition_bb,
        Terminator::Branch {
            src: src.into(),
            r#true: block_bb,
            r#false: terminator_bb,
        },
    );

    self.set_terminator(block_bb, Terminator::Goto(condition_bb));

    self.current_bb = terminator_bb;
} */
