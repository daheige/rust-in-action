use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_addr::add2;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add2(1,2)", |b| b.iter(|| add2(black_box(1),black_box(2))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);