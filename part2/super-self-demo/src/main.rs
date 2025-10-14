use std::fs;
use std::io::{self, Write}; // 这里使用self关键字引入io模块本身
use std::path::Path;

// 将指定的内容写入文件中，返回值是标准库的Result类型
fn write_file<P: AsRef<Path>>(path: P, content: String) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new().write(true).create(true).open(path)?;
    file.write(format!("{}", content).as_bytes())?;
    Ok(())
}

fn say_hello() {
    println!("called say_hello function");
}

pub mod my {
    // 这里使用super关键字，来调用上一层的say_hello函数
    pub fn hello() {
        super::say_hello();
    }

    pub fn foo() {
        println!("called my::foo function");
    }

    pub fn indirect_call() {
        println!("called my::indirect_call function");
        // self关键字表示当前的模块作用域，在这里指的是my模块
        self::cool::call();
        cool::call();
        self::foo();
        foo();
    }

    pub mod cool {
        pub fn call() {
            println!("called cool::call function");
        }
    }
}

fn main() {
    my::hello(); // 调用my模块中的hello函数
    my::cool::call(); // 调用my::cool模块中的call函数
    my::indirect_call(); // 调用my模块中的indirect_call函数

    // 调用write_file函数
    write_file("test.md", "hello,world!".to_string()).expect("failed to write file");
}
