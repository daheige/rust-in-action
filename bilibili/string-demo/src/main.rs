fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{s}"); // 将打印 `hello, world!`
    println!("str:{}", s);

    {
        let x = String::from("====hello,world!====");
        println!("{x}");
    }

    let x = 5;
    let y = x;
    println!("{y}");
    println!("{x}");

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!");
    println!("{s2}, world!");

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
    // println!("{s}");

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，
    println!("{}", x); // 所以在后面可继续使用 x

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{some_integer}");
}
