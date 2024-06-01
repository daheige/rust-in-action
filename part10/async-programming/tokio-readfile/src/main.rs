use std::io;
use tokio::fs; // 引入tokio fs模块

#[tokio::main]
async fn main() -> io::Result<()>{
    // 异步读取文件内容
    let content = fs::read_to_string("test.md").await?;

    println!("file content:\n{}",content);
    // 统计文件内容中每一行中出现rust字符串的行数
    let mut total = 0;
    for line in content.lines() {
        if line.contains("rust"){
            total +=1;
        }
    }

    println!("The total number of lines contains rust in the file content is {}", total);
    Ok(())
}
