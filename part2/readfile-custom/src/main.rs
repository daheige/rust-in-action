// 导入标准库中的env和fs模块
use std::env;
use std::fs;

mod custom_error;
use custom_error::CustomError;

fn main() {
    let res = read_file();

    // 模式匹配
    match res {
        Err(err) => println!("{:?}", err.0),
        Ok(content) => println!("{}", content),
    }
}

// 从终端中获取文件名，并读取文件内容
fn read_file() -> Result<String, CustomError> {
    println!("read file");

    // 获取终端输入的第二个参数，也就是指定读取的文件
    let file_path = env::args().skip(1).next().unwrap(); // 跳过第一个参数，得到文件名
    println!("file_path:{}", file_path);

    // 读取内容到字符串中
    // 如果发生错误，就将错误放入CustomError类型中
    let content = fs::read_to_string(file_path)
        .map_err(|err| CustomError(format!("read file err:{}", err)))?;

    Ok(content)
}
