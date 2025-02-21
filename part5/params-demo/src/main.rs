use std::env;

fn main() {
    // 将输入的参数放入一个String vec中
    // let args: Vec<String> = env::args().collect();
    // 跳过第一个参数，它是应用程序的名字
    let args: Vec<String> = env::args().skip(1).collect();
    // 打印args每个参数
    for (index, arg) in args.iter().enumerate() {
        println!("index:{} arg:{}", index, arg);
    }

    // 这里通过下标获取args每个参数
    // if args.len() >= 3 {
    //     let name: String = args[0].parse().unwrap();
    //     println!("name: {}", name);
    //     let first_num_str: String = args[1].parse().unwrap();
    //     println!("first_num_str: {}", first_num_str);
    //
    //     let second_num_str: String = args[2].parse().unwrap();
    //     println!("second_num_str: {}", second_num_str);
    // }

    // 参数转换，使用parse函数实现，在Rust底层调用的是FromStr::from_str关联函数实现类型转换。
    // 只要对目标类型实现了FromStr trait，就可以用parse把字符串转换成目标类型。
    // 标准库中已经给无数种类型实现了FromStr，
    // 如果要转换到用户定义类型，只要手动实现FromStr就行。
    let name: String = args[0].parse().unwrap();
    println!("name: {}", name);
    let first_num: i32 = args[1].parse().unwrap();
    println!("first num:{}", first_num);
    let second_num: f64 = args[2].parse().unwrap();
    println!("second num:{}", second_num);

    // 通过::<T>极速鱼语法调用parse方法时，
    // Rust编译器无法确定需要转换的数据类型是f32还是f64类型。
    // 因此，需要手动指定转换后的类型为f64类型。
    let second_num = args[2].parse::<f64>().unwrap();
    println!("second num:{}", second_num);
}
