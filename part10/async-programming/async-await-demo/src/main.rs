// 引入futures包提供的block_on执行器
use futures::executor::block_on;
async fn hello_cat() {
    println!("hello, kitty!");
}

// 在async fn中调用Future不使用.await表达式
async fn hello() {
    println!("call hello_cat without .await keywords");
    hello_cat();
}

// 在async fn中调用Future使用.await表达式
async fn say() {
    println!("call hello_cat with .await keywords");
    // 在async fn函数中使用.await可以等待任务执行完毕，并不会阻塞当前线程
    hello_cat().await;
    println!("hello,world");
}

fn main() {
    // // 在hello函数中不使用.await表达式
    // hello();

    // 使用block_on阻塞的方式运行Future
    // block_on(hello_cat());

    let f = say();
    block_on(f); // 使用执行器推进Future执行
}
