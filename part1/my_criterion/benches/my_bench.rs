use criterion::{black_box, criterion_group, criterion_main, Criterion};
use my_criterion::factorial;

// 基准测试函数封装
pub fn criterion_benchmark(c: &mut Criterion) {
    println!("bench factorial start...");
    c.bench_function("factorial(10)", |b| b.iter(|| factorial(black_box(10))));
    println!("bench factorial end");
}

// 使用criterion_group宏生成一个名为benches的基准组，
// 并通过criterion_main宏生成一个main函数并执行基准测试
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
