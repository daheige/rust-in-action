use std::error::Error;
use std::ffi::CString; // 导入ffi的CString

// 引入标准输入和输出模块
use std::io::{stdin, stdout, Write};
use std::os::raw::c_char;

// 在Rust中想使用c语言外部函数接口，需要通过extern块定义外部函数签名。
// 下面的 extern "C" 标识表示在rust语言中通过ffi机制使用C语言中库函数或自定义的函数。
//
// 需要强调的一个点：当你使用cc这个库，C代码会被编译为一个静态库文件libfoo.a文件。
// 生成静态库或动态库后，就可以使用 Rust 的 #[link(name = “foo”)] 属性来链接库libfoo.a文件，
// 这样就可以在Rust代码中调用 C 函数了。
// 在link链接时，需要注意的一点：
// link链接的名字必须要和build.rs中，在cc实例对象上调用compile方法传递的参数一致
#[link(name = "foo")]
extern "C" {
    fn print_app_info();
    fn hello();
    fn greet(name: *const c_char);
}

fn prompt(s: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", s); // 需要输入之前的提示语

    // 先刷新下输出流，确保所有中间缓冲的内容都已经完全输出了
    stdout().flush()?;

    // 获取用户在终端输入的内容
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 调用C代码中的自定义函数，这几个函数是相对来说不安全的，
    // 所以需要调用的地方加上unsafe关键字包裹，明确告诉这非安全的调用
    unsafe {
        print_app_info();
    }

    println!("call c code begin...");
    // 调用hello函数，它是相对来说安全的函数，所以这里需要加上unsafe关键字
    unsafe {
        hello();
    }

    let name = prompt("what's your name?")?;
    //  通过CString::new创建一个C语言兼容的字符串CString对象
    // 这个CString对象是安全的，它用于处理传统的c风格字符串(由单个空字节终止的非空字节序列)，
    // 这类字符串的主要用例是与类c代码进行互操作。
    let c_name = CString::new(name)?;

    // 通过unsafe块的方式调用greet函数
    unsafe {
        // 这里在c_name上面调用as_ptr方法，返回结果是指向该C字符串的内部指针，对应C语言的char*类型
        // 这个C字符串的内部指针作为greet函数的参数
        greet(c_name.as_ptr());
    }

    Ok(())
}
