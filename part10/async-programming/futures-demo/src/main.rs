// 引入futures库的block_on
use futures::executor::block_on;

async fn hello_world() {
    println!("hello,world!");
}

async fn greet(name: &str) {
    println!("hello,{}!", name);
}

async fn hello_cat() {
    println!("hello,kitty");
}

async fn say() {
    // 在async fn函数中使用.await并不会阻塞当前线程，而是异步地等待Future完成。
    hello_cat().await;
}

fn main() {
    println!("exec async task...");
    // futures block_on阻塞当前线程，直到提供的Future运行完成
    block_on(hello_world());

    // 这里与block_on不同的是，say函数中的.await并不会阻塞当前的线程，
    // 而是异步等待hello_cat函数返回的Future执行完成，
    // 在等待的过程中，该线程还可以继续执行其它的Future，最终实现了并发处理的效果。
    let f = say();
    block_on(f);

    block_on(greet("futures"));
    block_on(greet("rust async programming"));
}
