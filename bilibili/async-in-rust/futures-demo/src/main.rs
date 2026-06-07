// 引入futures库的block_on
use futures::executor::block_on;

async fn hello_world() {
    println!("hello,world!");
}

async fn greet(name: &str) {
    println!("hello,{}!", name);
}

async fn hello_cat() {
    println!("hello, kitty!");
}

// 在async fn中调用Future不使用.await表达式
// hello_cat函数不会执行
// async fn hello() {
//     println!("call hello_cat without .await keywords");
//     hello_cat();
// }

// 在async fn中调用Future使用.await表达式
async fn say() {
    println!("call hello_cat with .await keywords");
    // 在async fn函数中使用.await可以等待任务执行完毕，并不会阻塞当前线程
    hello_cat().await;
    println!("hello,world");
}

fn main() {
    // println!("exec async task...");
    // let future = hello_world(); // 返回结果是一个Future对象
    // block_on(future); // block_on阻塞当前线程，直到提供的Future运行完成
    //
    // let f = greet("rust async programming");
    // block_on(f);

    let f = say();
    block_on(f); // 使用执行器推进Future执行
}
