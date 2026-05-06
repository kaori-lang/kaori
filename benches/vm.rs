use criterion::{Criterion, criterion_group, criterion_main};
use kaori::{program::compile_source_code, runtime::vm::Vm};

fn benches(c: &mut Criterion) {
    let source = std::fs::read_to_string("examples/kaori/iterative_fib.kr").unwrap();
    compile_source_code(&source).unwrap();

    c.bench_function("iterative_fib", |b| {
        b.iter(|| {
            let mut vm = Vm::new();
            vm.run().unwrap();
        });
    });
}

criterion_group!(bench_group, benches);
criterion_main!(bench_group);
