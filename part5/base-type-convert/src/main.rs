fn main() {
    // 使用as关键字显式转换
    // 字符类型转换为整型
    let c = 'A';
    let c_num = c as u8; // 使用as将字符转换为u8类型
    println!("A char to u8 = {}", c_num);

    // 整型转换为字符类型
    let num: u8 = 65;
    let c2 = num as char;
    println!("65 to char = {}", c2);

    // 布尔类型转换为整型
    let b = true;
    let b2 = false;
    let b_num = b as i32;
    let b2_num = b2 as i32;
    println!("true to i32 = {}", b_num);
    println!("false to i32 = {}", b2_num);

    // 将浮点数转换为无符号的整型，使用as显式转换
    let f: f32 = 12.3;
    let num = f as u32;
    println!("f32 12.3 to u32 = {}", num);

    // 整型转换为浮点数，会发生隐式强制转换
    let i: i32 = 12;
    let f = i as f64; // 隐式转换
    println!("12 to f64 = {}", f);

    let n: i32 = 11;
    let f: f64 = 11.0;
    // 使用as将n强制转换为f64后，
    // 再比较这两个数字
    if n as f64 == f {
        println!("two number is same");
    }

    // 下面的代码将i32类型转换为char类型是无法运行的。
    // 因为字符型的数据范围只和8-bit的整型数据范围是一致的，
    // 字符型和其他数据类型（例如：f32、f64、i64和u64等）的数据范围不一致。
    // let n: i32 = 65;
    // let c = n as char;
    // println!("i32 type 65 to char = {}", c);

    // 在Rust中浮点数的默认类型是f64，这是因为f64类型在当前的Rust编译器中拥有最优化的性能表现。
    // 如果你需要使用f32（单精度浮点数），你需要显式指定它。
    // 下面的代码也是无法运行，和上面一样。
    // let f = 12.3;
    // let n = f as char;
    // println!("f32 type 65 to char = {}", c);
}
