use anyhow::Result as aResult;
use std::fs;

fn main() -> aResult<()> {
    // 读取文件内容
    let result = fs::read_to_string("test.md");
    // match模式匹配
    let content = match result {
        Ok(content) => content,
        Err(err) => return Err(err.into()), // 错误提前返回
    };

    println!("文件内容: {}", content);
    Ok(())
}
