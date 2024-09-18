use std::error::Error;
use std::ffi::CString; // 导入ffi的CString

// 引入标准输入和输出模块
use std::io::{stdin, stdout, Write};
use std::os::raw::c_char;

// #[link标记属性用于关联静态库libfoo.a文件
#[link(name = "foo")]
extern "C" {
    // 在Rust中调用C语言外部函数接口，需要通过extern "C"块定义外部函数签名。
    fn print_app_info();
    fn greet(name: *const c_char);
}

// 获取用户输入的字符串
fn prompt(s: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", s); // 输出提示语
                     // 刷新输出流，确保所有中间缓冲的内容都已经完全输出了。
    stdout().flush()?;

    // 获取标准输入的字符串，将其放入input中
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("call c code begin...");
    // 通过unsafe调用C代码中的自定义函数
    unsafe {
        print_app_info();
    }

    let name = prompt("what's your name?")?;
    // 通过CString::new创建一个与C语言兼容的字符串CString对象。
    // 这个CString字符串对象是安全的，它用于处理传统的C风格字符串(由单个空字节终止的非空字节序列)，
    // 这类字符串可以安全地与C代码进行互操作。
    let c_name = CString::new(name)?;

    // 通过unsafe块的方式调用greet函数
    unsafe {
        // 这里在c_name上面调用as_ptr方法，返回结果是指向该C字符串的内部指针，对应C语言的char*类型
        // 这个C字符串的内部指针作为greet函数的参数
        greet(c_name.as_ptr());
    }

    Ok(())
}
