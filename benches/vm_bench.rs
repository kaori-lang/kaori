use std::fs;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use kaori::{program::compile_source_code, virtual_machine::kaori_vm::KaoriVM};

fn bench_execute(c: &mut Criterion) {
    let source_path = "test_suite/test.kr";
    let source = match fs::read_to_string(source_path) {
        Ok(source) => source,
        Err(error) => return,
    };

    c.bench_function("vm", |bencher| {
        let bytecode = match compile_source_code(source.to_owned()) {
            Ok(bytecode) => bytecode,
            Err(error) => {
                error.report(&source);
                return;
            }
        };

        let mut vm = KaoriVM::new(bytecode.instructions, bytecode.constants);
        bencher.iter(|| {
            vm.run();
        });
    });
}

criterion_group!(benches, bench_execute);
criterion_main!(benches);
