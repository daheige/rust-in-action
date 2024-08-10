use tokio::runtime::Runtime;

fn main() {
    // 创建tokio runtime运行时实例
    let rt = Runtime::new().expect("failed to new tokio runtime");

    // 执行future，阻塞当前线程直到完成
    rt.block_on(async {
        println!("hello,world!");
    });

    // 在tokio运行时中生成一个future
    // spawn方法返回值是一个JoinHandle<T>，它位于tokio::runtime::task::join模块中
    // JoinHandle<T>和标准库中std::thread::spawn返回值几乎相同
    // 当spawn被调用时，提供的future将立即在后台运行，即使你没有等待返回的JoinHandle
    let handler = rt.spawn(async {
        println!("rt spawn task");
    });
    // 等待异步任务执行完毕
    rt.block_on(handler).expect("failed to exec async task");
}

mod tests {
    // tokio spawn单元测试
    #[tokio::test]
    async fn test_spawn(){
        let _ = tokio::spawn(async{
            println!("hello,world!");
        }).await;
    }

    // tokio::spawn代码等价于下面的代码
    #[test]
    fn test_runtime(){
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                println!("hello,world!");
            });
    }
}