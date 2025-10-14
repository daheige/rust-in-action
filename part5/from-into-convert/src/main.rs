// 自定义数据类型Number
#[derive(Debug)]
struct Number {
    value: i32,
}

// 实现From trait的from方法，实现自定义数据转换
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

fn main() {
    let num = Number::from(12);
    println!("num is {:?}", num);

    // 将i32调用into方法转换为Number类型
    let n: Number = 13.into();
    println!("n is {:?}", n);
    println!("n inner value:{}", n.value);
}
