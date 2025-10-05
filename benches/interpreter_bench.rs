use std::fs;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use kaori::{program::compile_source_code, virtual_machine::interpreter::Interpreter};

fn bench_execute(criterion: &mut Criterion) {
    let source_path = "test_suite/test.kr";
    let source = match fs::read_to_string(source_path) {
        Ok(source) => source,
        Err(error) => return,
    };

    let bytecode = match compile_source_code(source.to_owned()) {
        Ok(bytecode) => bytecode,
        Err(error) => {
            error.report(&source);
            return;
        }
    };

    let mut interpreter = Box::new(Interpreter::new(bytecode.instructions, bytecode.constants));

    criterion.bench_function("execute_instructions", |bencher| {
        bencher.iter(|| {
            interpreter.execute_instructions();
            black_box(&mut interpreter);
        });
    });
}

criterion_group!(benches, bench_execute);
criterion_main!(benches);
