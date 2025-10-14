#![feature(test)]
extern crate test;
use test::Bencher;

// 求1-n之间的数字之和
pub fn sum(n: i32) -> i32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }

    return total;
}

// 需要先将rust版本切换为nightly版本
// rustup override set nightly
// 再运行 cargo bench命令才可以正常运行内置的基准测试
#[bench]
fn bench_sum(b: &mut Bencher) {
    b.iter(|| {
        for i in 1..=10 {
            println!("sum({}) = {}", i, sum(i));
        }
    })
}
