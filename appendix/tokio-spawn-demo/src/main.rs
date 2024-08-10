// 通过tokio::main方式启动async异步任务
// #[tokio::main]
// async fn main(){
//     tokio::spawn(async{
//         println!("hello,world!");
//     }).await;
// }

// 上述代码等价于下面的代码：
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("hello,world!");
        });
}

mod tests {
    // tokio spawn单元测试
    #[tokio::test]
    async fn test_spawn() {
        let _ = tokio::spawn(async {
            println!("hello,world!");
        }).await;
    }
}
