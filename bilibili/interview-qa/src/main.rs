mod advance;

use std::fs;

fn main() {
    println!("Hello, world!");
    // 定义变量
    let x = 5;
    let y: f64 = 10.5;
    let z = "hello";
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // 元组
    let point: (i32, i32) = (1, 2); // 定义元组类型
    let (x, y) = point; // 解构
    println!("The value of point is: {:?}", point);
    println!("x is: {} y is: {}", x, y);
    println!("x is: {} y is: {}", point.0, point.1);

    // 数组
    let numbers = [1, 2, 3, 4, 5];
    let first = numbers[0];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", numbers[1]);
    println!("numbers: {:?}", numbers);

    // 切片，从指向数组的底层
    let arr = [1, 2, 3];
    // let s = &arr[0..4]; // range end index 4 out of range for slice of length 3
    // 创建一个指向原数组子集的不可变切片
    let s = &arr[0..3]; // 不能超过数组的长度，s是一个切片类型，不可变
    println!("The value of arr is: {:?}", s);

    // 定义结构体
    let p = Point { x: 5, y: 10 };
    println!("p.x: {}, p.y: {}", p.x, p.y);

    let i = 32; // 自动推导i为i32类型
    println!("The value of i is: {}", i);

    // Rust 不允许声明未初始化的变量。所有的变量都必须在使用之前被正确地初始化。尝试声明一个未初始化的变量会导致编译错误。
    // let x; // consider giving `x` an explicit type，编译报错
    // println!("The value of x is: {}", x);

    let mut s = String::from("hello");
    let m = &mut s; // 创建一个指向s的可变引用，这里的m实际上是 let m: &mut String 类型
    println!("The value of m is: {}", m);
    m.push_str(", world!"); // 对m进行可变引用修改，它会直接反应到s变量上
    println!("The value of s is: {}", s); // s此时是hello,world!
    print_str(&s);

    // 这里的data是一个字符串引用，字面量
    // 创建一个 MyStruct 实例，其中数据字段与 s 相关联
    //
    // MyStruct 结构体有一个名为 data 的字段，它的类型为 &'a str。这意味着 data 字段必须在其所有者（即 MyStruct 实例）的作用域内保持有效。
    // 因此，在 main 函数中创建 my_struct 实例时，我们可以安全地将 &s 作为 data 字段的值，因为 s 在 my_struct 的作用域内始终有效。
    // 通过生命周期绑定，Rust 可以在编译时确保引用的生命周期不会超过其所有者的生命周期，从而避免了悬垂引用和无效内存访问等问题。
    let s = MyStruct { data: "daheige" };
    println!("The value of s is: {}", s.data);

    // match模式匹配
    let x = Some(5); // x类型其实是一个Option，Option<i32> 类型
                     // 使用 match 表达式来检查 x 是否为 Some 或 None 类型的值，并根据结果执行相应的代码块
    match x {
        Some(i) => println!("The value of x is: {}", i),
        None => println!("There is no value"),
    }

    // 也可以使用if let Some进行解构，模式匹配
    // if let 结构是 match 表达式的简化版，适用于只需要处理一种情况的情况
    if let Some(i) = x {
        println!("The value of x is: {}", i);
    } else {
        println!("There is no value");
    }

    let (x, y) = (1, 2); // let 语句中解构,模式匹配
    println!("The value of x is: {} y is: {}", x, y);

    // 下面的用法用的比较少
    let Some(x) = Some(5) else { todo!() };
    println!("The value of x is: {}", x);

    // 使用enum进行模式匹配
    let c = Color::Red;
    print_color(c);

    // 当一个 match 表达式没有涵盖所有可能的模式时，Rust 编译器会报错。
    // 为了防止这种情况，你可以使用通配符 _ 来表示忽略其他任何模式
    let x = Some(5);
    match x {
        Some(i) => {
            println!("Got the x value: {}", i);
        }
        None => {
            println!("There is no value");
        }
    }

    // 错误处理
    let res = read_file("test.txt");
    // 结果使用 match 模式匹配，Ok,Err
    // 其中Ok表示成功读取文件，结果放在s变量中，Err表示出错了的结果放在e中
    // 这里使用了解构和模式匹配组合使用
    match res {
        Ok(s) => println!("The file of content is: {}", s),
        Err(e) => println!("There was an error: {}", e),
    }

    // 这里我使用了unwrap_or函数设置默认值
    let s = read_file_to_string("test.txt").unwrap_or("".to_string()); // 如果读取失败，内容默认为空字符串
    println!("The value of s is: {}", s);

    // 这里直接终止，一般在程序启动，初始化可以使用unwrap，或者你明确不会发生panic可以使用unwrap，
    // 否则其他情况建议使用match模式匹配或if let Ok(v)方式解构
    // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    // let content = read_file_to_string("hello.txt").unwrap();

    // 当然你还可以使用 expect("xxx")明确指定错误提示
    // 这样提示信息，更加清晰，好处是：你可以使用 expect 方法添加自定义错误消息
    // failed to read file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    // let content = read_file_to_string("hello.txt").expect("failed to read file");

    // 在 Rust 中，不可变引用是一种特殊的指针类型，它允许你访问一个值但不能修改它。不可变引用是通过 & 符号和变量名来创建的。
    //
    // 当创建一个不可变引用时，Rust 会确保该引用所指向的数据在其整个生命周期内不会被修改。
    // 这意味着当你有一个不可变引用到某个数据时，其他代码无法在同一作用域内修改这个数据。
    // 这种机制有助于防止并发问题，并提高了内存安全性。
    let mut x = 5;
    let y = &x; // 这里的y是不可变引用
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // *y = 10; // 改变y的值，编译器会拒绝编译
    // 不可变引用对于提高 Rust 程序的安全性和效率非常有帮助，因为它可以在不复制数据的情况下共享数据，
    // 同时确保数据不会被意外修改
    // ^^^^^^^ `y` is a `&` reference, so it cannot be written to

    x = 10;
    println!("The value of x is: {}", x);

    let mut x = 5; // 这里的x是可变类型
    let y = &mut x; // 这里的y是一个可变引用
                    // 通过 *y 进行解引用以修改 x 的值
    *y = 11; // 通过可变引用改变x的值
    println!("The value of x is: {}", x);

    let x = 12;
    let y = 13;
    // 支持简写语法来初始化结构体，特别是当结构体的字段名与你用来初始化它们的变量名相同时
    // 这里我们没有指定结构体成员的名字，因为它们与初始化时使用的变量名相同。
    let p = Point { x, y };
    println!("The value of p.x is: {}", p.x);

    // 使用结构体上的关联函数创建实例对象
    let p = Point::new(1, 2);
    p.print()
}

// 生命周期需要先定义，再使用
// 当我们调用这个函数并传入 &s 时，Rust 编译器会推断出 'a 应该等于 s 的作用域
// fn print_str<'a>(s: &'a str) {
//     println!("The value of s is: {}", s);
// }
// 其实这里的‘a可以省略生命周期
fn print_str(s: &str) {
    println!("The value of s is: {}", s);
}

// 生命周期绑定
// 生命周期绑定 是指在结构体、枚举或其他类型定义中，
// 将某个字段的生命周期与整个类型实例的生命周期相关联。这使得编译器能够确保该字段在整个类型实例的生命周期内都是有效的。
//
// 这里定义了一个结构体，内部中有一个data字段是引用类型，执行字符串对象的引用，为了保证整个过程中，字段的引用一直有效
// 因此使用了'a来做生命周期标注，使得编译器能够确保该字段在整个类型实例的生命周期内都是有效的。
struct MyStruct<'a> {
    data: &'a str,
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn print(&self) {
        println!("Point x = {}, y = {}", self.x, self.y);
    }
}
enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(c: Color) {
    // 对于枚举必须穷举完毕
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        // _ => println!("anything"),
    }
}

// 返回Result类型，如果读取文件成功，就返回Ok(string)作为值，否则就是Err(err)
fn read_file(path: &str) -> Result<String, std::io::Error> {
    // 将文件内容读到字符串中
    fs::read_to_string(path)
}

fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let s = fs::read_to_string(path)?; // 这里使用了?简写模式，进行错误处理，提前返回了error
    Ok(s)
}
