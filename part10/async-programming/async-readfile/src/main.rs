// 引入futures中的block_on模块
use futures::executor::block_on;
use std::fs;

// 异步读取文件内容
async fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

async fn read(path: &str) {
    let contents = read_file(path).await.unwrap();
    println!("read contents:\n{}", contents);
}

fn main() {
    let path = "test.md";
    let f = read(path);
    block_on(f);
}
