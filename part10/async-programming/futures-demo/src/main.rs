// 引入futures库的block_on
use futures::executor::block_on;

async fn hello_world() {
    println!("hello,world!");
}

async fn greet(name: &str) {
    println!("hello,{}!", name);
}

fn main() {
    println!("exec async task...");
    let future = hello_world(); // 返回结果是一个Future对象
    block_on(future); // block_on阻塞当前线程，直到提供的Future运行完成

    let f = greet("rust async programming");
    block_on(f);
}
