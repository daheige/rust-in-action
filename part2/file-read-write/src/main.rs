use anyhow::Result as aResult;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// 使用第三库anyhow::Result作为main函数返回值
// anyhow::Result定义是pub type Result<T, E = Error> = core::result::Result<T, E>;
fn main() -> aResult<()> {
    // 读取文件内容
    let content = fs::read_to_string("test.md")?;
    println!("file content: \n{}", content);
    let res = write_file("test2.md", content)?;
    println!("{}", res);
    Ok(())
}

// 将内容写入文件中，返回值是标注库的Result
fn write_file<P: AsRef<Path>>(path: P, content: String) -> Result<String, io::Error> {
    let mut file = fs::OpenOptions::new().write(true).create(true).open(path)?;
    file.write(content.as_bytes())?;
    Ok("write file success".to_string())
}
