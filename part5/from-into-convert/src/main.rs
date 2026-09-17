// 自定义数据类型Number
#[derive(Debug)]
struct Number {
    value: i32,
}

// 实现From trait的from方法，实现自定义数据转换
// 将 i32 类型数字，转换为 Number 类型，实现 trait From<T>: Sized 特征
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

struct Wrapper(String);

// 只实现这一个：
impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Wrapper(s)
    }
}

fn main() {
    let num = Number::from(12);
    println!("num is {:?}", num);

    // 将i32调用into方法转换为Number类型
    // 由于 Into trait 底层调用的是 U::from函数
    // fn into(self) -> U {
    //     U::from(self)
    // }
    // 实际上调用的就是 Number::from 关联函数，实现了类型转换
    let n: Number = 13.into();
    println!("n is {:?}", n);
    println!("n inner value:{}", n.value);

    let w1 = Wrapper::from(String::from("a")); // From
    let w2: Wrapper = String::from("b").into(); // Into
    assert_eq!(w1.0, "a");
    assert_eq!(w2.0, "b");
}
