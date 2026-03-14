# tokio 运行时
tokio 是一个开源的异步运行库，它提供了一些基础的异步编程模块，例如：异
步 io、net、fs、task、sync、signal 等，这些模块简化了异步编程的复杂性，使得
开发人员可以快速构建可靠且高效的应用程序。

由于 tokio 是基于 Rust 的 async/await语言特性实现的，
因此 tokio 运行时本身也是可以拓展的。tokio 不仅适用于网络编程，
还可以用于其他类型的异步 IO 操作，例如：文件异步读写、数据库操作等，同时它
还提供了一种统一的编程模型，使得开发人员可以更加专注于业务逻辑的实现，而无
需关心底层的异步实现细节。

`tokio-demo`
```rust
use tokio::task;
async fn say_hello(name: &str){
    println!("hello,{}",name);
}

// 通过#[tokio::main]属性宏将异步函数标记为由所选运行时执行。
// 这个宏可以帮助用户设置运行时，而不需要用户直接使用Runtime或Builder。
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

```
#`[tokio::main]`标记的 main 函数代码会变成 fn main 格式的代码块。转换后
的 main 函数中，它将使用 `tokio::runtime::Runtime::new` 创建了一个运行时 rt 对象，然
后在 rt 对象上调用 block_on 方法，将驱动异步 async 代码执行（在 Rust 底层 async 返
回结果是一个 Future 类型，因此需要一个异步执行器来推动 Future 执行）。

在上述示例 main 函数中，首先调用 say_hello 函数会返回一个 Future 类型。然后，
使用.await 关键字等待 Future 执行完毕。tokio 库提供的 task 模块用于执行异步任务。
这些任务类似于操作系统线程，但它们不是由操作系统调度器管理，而是通过 tokio
运行时调度管理。这种模式的另一个名称叫作：绿色线程。如果你熟悉 Go 语言的协
程（GMP 调度）或 Erlang 语言的进程调度，那么你可以将 tokio 的 task 模块与它们类
比。

tokio::task::spawn 函数定义如下：
```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let fut_size = std::mem::size_of::<F>();
        if fut_size > BOX_FUTURE_THRESHOLD {
            spawn_inner(Box::pin(future), SpawnMeta::new_unnamed(fut_size))
        } else {
            spawn_inner(future, SpawnMeta::new_unnamed(fut_size))
        }
    }
```
`task::spawn` 函数的参数是一个 Future，它几乎和标
准库标准库 `std::thread::spawn` 函数签名相似，函数返回值是一个 `JoinHandle<T>`类型。
因此在上述示例 main 函数中将一个 `async` 语句块作为 `task::spawn` 的参数，函数返回
值 `handler` 是一个 `JoinHandle<()>`类型，在 handler 上使用`.await` 让这个异步任务开始运
行起来。

一个使用 tokio 库执行多个异步操作的示例:
`tokio-readfile`

```rust
use std::io;
use tokio::fs; // 引入tokio fs模块

// 通过#`[tokio::main]`标记的 main 函数代码会变成 fn main 格式的代码块。
#[tokio::main]
async fn main() -> io::Result<()> {
    // 异步读取文件内容
    let content = fs::read_to_string("test.md").await?;

    println!("file content:\n{}", content);
    // 统计文件中出现rust字符串的行数
    let mut total = 0;
    for line in content.lines() {
        if line.contains("rust") {
            total += 1;
        }
    }

    println!(
        "The total number of lines contains rust in the file content is {}",
        total
    );
    Ok(())
}
```
通过 `tokio::fs` 模块异步读取了 test.md 文件的内容，并判断每一行内容是否存在 rust 字符串。如果存在，sum 计数就会加 1。

tokio 作为一个强大且灵活的异步运行库（tokio 运行时调度机制见附录 B），它的目标是通过提供一种简洁而强大的抽象层，使得异步编程更加容易和直观。
更多 `tokio` 用法，你可以参考官方文档:https://tokio.rs/tokio/tutorial。
