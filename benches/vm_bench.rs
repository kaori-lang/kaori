use criterion::{Criterion, black_box, criterion_group, criterion_main};
use kaori::program::run_program;
use std::fs;

fn bench_execute(c: &mut Criterion) {
    c.bench_function("run_code", |b| {
        let source_path = "test_suite/recursive_fib.kr";
        let source = match fs::read_to_string(source_path) {
            Ok(source) => source,
            Err(error) => {
                eprintln!("Error reading source file: {}", error);
                return;
            }
        };

        b.iter(|| {
            // Measure only the program execution

            black_box(run_program(&source));
        });
    });
}

criterion_group!(benches, bench_execute);
criterion_main!(benches);
