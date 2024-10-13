use std::fs;
use std::path::Path;

fn main() {
    // 读取当前目录下的test.md文件
    let path = "./test.md";
    let content = read_file(path).expect("failed to read file");
    // 输出文件内容到终端中
    println!("{}", content);
}

/// 读取文件的内容，函数接受一个实现了`AsRef<Path>`trait的参数path。
/// `AsRef<Path>`是一个trait，它定义了一个方法as_ref，该方法返回一个Path引用，
/// 它可以被实现为各种类型，包括&Path本身，以及其他可以转换为Path引用的类型，比如String、&str等。
fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    // 将内容放在content变量中，如果读取过程中发生错误就返回Error
    let content = fs::read_to_string(path)?;
    Ok(content)
}
