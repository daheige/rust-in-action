use tokio::task;
async fn say_hello(name: &str){
    println!("hello,{}",name);
}

#[tokio::main]
async fn main() {
    // 使用.await关键字来等待Future执行完成
    say_hello("world").await;

    // 通过task模块提供的spawn执行异步任务
    // spawn函数返回值是一个JoinHandle<T>，它位于tokio::runtime::task::join模块中
    // JoinHandle<T>和标准库中std::thread::spawn返回值几乎相同
    let handler = task::spawn(async {
        for i in 1..5{
            println!("current index:{}",i);
        }
    });

    // 通过.await等待异步任务执行完毕
    handler.await.unwrap();
}

// #[tokio::main]会将async fn main函数转换为如下代码：
// fn main(){
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async{
//         // 省略其他代码...
//     });
// }