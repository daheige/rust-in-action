// 引入rand包的相关模块
use rand::{Rng, rng};

fn main() {
    // 创建rand实例
    let mut rng = rng();
    // 生成(0,1]的浮点数
    // 由于rust edition 2024 gen是关键字，这个库使用rng.random替代r#gen
    println!("random f64: {}", rng.random::<f64>());

    // 生成随机i32数字
    println!("random i32: {}", rng.random::<i32>());
    println!("random u8: {}", rng.random::<u8>());
    println!("random u32: {}", rng.random::<u32>());
    println!("random u64: {}", rng.random::<u64>());

    // 生成i32随机数
    let x: i32 = rng.random_range(100..999);
    println!("x:{}", x);

    // 生成区间的随机数[1,10)
    let i: i64 = rng.random_range(1..10);
    println!("random number i:{}", i);

    let m: u32 = rng.random_range(1..100);
    println!("random number m:{}", m);
}
