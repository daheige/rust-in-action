use std::env;

fn main() {
    // 将程序CLI输入的参数放入一个String vec中
    // let args: Vec<String> = env::args().collect();
    // 跳过第一个参数，它是程序的名字params-demo
    let args: Vec<String> = env::args().skip(1).collect();
    // 打印cli参数
    // println!("args:{:?}", args);

    let name: String = args[0].parse().unwrap();
    println!("name: {}", name);
    let first_num: i32 = args[1].parse().unwrap();
    println!("first num:{}", first_num);
    let second_num: f64 = args[2].parse().unwrap();
    println!("second num:{}", second_num);
    // ::<T>极速鱼语法
    let second_num = args[2].parse::<f64>().unwrap();
    println!("second num:{}", second_num);
}
