use kaori::{program::compile_source_code, virtual_machine::interpreter::Interpreter};
use std::fs;

#[test]
fn execute_instructions_miri() {
    // Load your script
    let source_path = "test_suite/test.kr";
    let source = fs::read_to_string(source_path).expect("failed to read test.kr");

    // Compile to bytecode
    let bytecode = match compile_source_code(source) {
        Ok(bytecode) => bytecode,
        Err(error) => return,
    };

    // Run interpreter
    let mut interpreter = Interpreter::new(bytecode.instructions, bytecode.constants);
    interpreter.execute_instructions();
}
