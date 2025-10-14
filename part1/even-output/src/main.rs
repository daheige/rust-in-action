fn main() {
    // 输出0-20之间的偶数
    for num in 0..=20 {
        if num % 2 == 0 {
            println!("current even number: {}", num);
        }
    }
}
