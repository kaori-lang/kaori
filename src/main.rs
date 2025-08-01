use std::fs;

use kaori::{
    backend::{codegen::bytecode_generator::BytecodeGenerator, vm::kaori_vm::KaoriVM},
    error::kaori_error::KaoriError,
    frontend::{
        scanner::{lexer::Lexer, token_stream::TokenStream},
        semantic::{resolver::Resolver, type_checker::TypeChecker},
        syntax::parser::Parser,
    },
};

fn main() {
    if let Ok(source) = fs::read_to_string("src/code/main.kaori") {
        if let Err(err) = run_program(source.clone()) {
            err.report(&source);
        }
    }
}

pub fn run_program(source: String) -> Result<(), KaoriError> {
    let mut lexer = Lexer::new(source.clone());

    let tokens = lexer.tokenize()?;

    let token_stream = TokenStream::new(source.clone(), tokens);

    let mut parser = Parser::new(token_stream);

    let mut declarations = parser.declarations()?;

    let mut resolver = Resolver::new();
    resolver.resolve(&mut declarations)?;

    let mut type_checker = TypeChecker::new();

    type_checker.check(&mut declarations)?;

    let mut bytecode_generator = BytecodeGenerator::new();

    let bytecode = bytecode_generator.generate(&mut declarations)?;

    let vm = KaoriVM::new(bytecode);

    vm.execute_instructions();

    Ok(())
}
