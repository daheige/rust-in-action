# async/await基础
为了简化 Rust 异步编程的复杂性，Rust 1.39 版本开始引入了 async/await 语法，让开发人员可以使用同步的方式去编写更加直观、易读、易维护的异步代码。

`async-demo`
```rust
async fn hello_cat() {
    println!("hello, kitty!");
}

async fn say() {
    hello_cat();
}

fn main() {
    say();
}
```
当我们执行 cargo run 命令运行该示例时，发现"hello, kitty!"并没有打印出来，同
时编译器给出了提示：“warning: unused implementer of `futures::Future` that must be used”
hello_cat 函数调用后返回值是一个 Future，
该 Future 没有执行，因此不会输出任何内容。那我们该怎么才可以让 Future 执行呢？
答案是需要一个执行器 executor 才可以推进 Future 执行。

`async-await-demo`
```rust
// 引入futures包提供的block_on执行器
use futures::executor::block_on;
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
    // 在hello函数中不使用.await表达式
    // hello();

    // 使用block_on阻塞的方式运行Future
    // block_on(hello_cat());

    let f = say();
    block_on(f); // 使用执行器推进Future执行
}
```
该示例和 10.3.1 章节中的示例使用 block_on 函数以阻塞的方式运行 async
异步函数有所不同。在该示例中，async fn 异步函数中是通过.await 关键字驱动执行的。
也就是说，.await 关键字并不会阻塞当前线程，而是异步推进当前 Future 执行，直到
程序执行完毕。在异步函数执行期间，当前线程还可以继续执行其他代码，从而提升
了并发程序的性能和执行效率。

使用 `async` 标记的异步函数块会被 Rust 编译器转换为 `Future` 特征的状态
机。同时，`await` 表达式也会获取 `Future` 的所有权，并对其执行 `poll` 轮询操作，驱动
`Future` 执行，直到 `Future` 的状态变成 `Poll::Ready(T)`已完成为止。如果 `Future` 执行完毕，
其最终值就是 `await` 表达式的值，否则就会返回 `Poll::Pending` 给调用者。如果在执行
过程中发生了阻塞，它就会让出控制权，允许线程执行其他代码。
