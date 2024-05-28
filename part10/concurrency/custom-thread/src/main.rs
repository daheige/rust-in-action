use std::thread;

fn main() {
    // 设置线程栈大小为1MB并设置线程的名字
    let stack_size = 1*1024 * 1024; // 1MB
    let builder = thread::Builder::new().stack_size(stack_size).name("my_thread".to_string());

    println!("在自定义的线程中打印1-100的数字");
    let handler = builder.spawn(|| {
        for i in 1..101 {
            print!("{} ",i);
        }
    }).unwrap();

    // 等待线程执行完毕
    handler.join().unwrap();
}