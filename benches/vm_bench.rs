use std::fs;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use kaori::program::compile_source_code;

fn bench_execute(c: &mut Criterion) {
    let source_path = "test_suite/test.kr";
    let source = match fs::read_to_string(source_path) {
        Ok(source) => source,
        Err(error) => return,
    };

    c.bench_function("vm", |bencher| {});
}

criterion_group!(benches, bench_execute);
criterion_main!(benches);
