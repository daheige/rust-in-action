use std::ffi::c_double; // 64位的浮点数

// // 这里的c_int 相当于C的signed int (int)类型，32位的整型数字，
// 这个类型几乎总是i32，但在一些系统上可能会有所不同。
// 从技术上讲，C标准只要求该类型是一个有符号整数，至少是一个short，
// 在有些系统下将其定义为i16。
use std::ffi::c_int;

unsafe extern "C" {
    // 求一个int数字的绝对值
    fn abs(num: c_int) -> c_int;

    // 当然也可以使用下面的方式因为c_int对应的数字是32位的
    // fn abs(num: i32) -> i32;

    // 求一个数字的平方根
    fn sqrt(num: c_double) -> c_double;
}

fn main() {
    // 通过unsafe关键字修饰，调用c语言提供的函数
    unsafe {
        let num = -10;
        println!("-10的绝对值是：{}", abs(num));

        let x = 64.0;
        println!("64的平方根是：{}", sqrt(x));
    }
}
