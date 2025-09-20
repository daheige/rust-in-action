use rand::{Rng, rng}; // 引入第三方库rand中的模块
use std::sync::{Arc, Mutex}; // 引入Arc和Mutex
use std::thread;

fn main() {
    // 创建一个Arc<Mutex<i32>>类型，初始值为0
    let counter = Arc::new(Mutex::new(0));

    // 存放spawn函数的返回结果JoinHandle<()>
    let mut handlers = Vec::new();
    // 创建多个线程，每个线程都会增加计数器的值
    for i in 0..5 {
        let counter = counter.clone(); // 原子引用计数，这里是克隆counter
        let handler = thread::spawn(move || {
            // 获取互斥锁，以便安全地访问和修改计数器的值
            // 获取相加之前的数据
            let mut num = counter.lock().unwrap();
            let counter = *num;
            println!(
                "current thread index:{} counter before adding is:{}",
                i, counter
            );

            // 对Arc<Mutex<i32>>类型中的计数器，随机增加1-10的数字
            let mut rng = rng(); // 创建随机数实例对象
            let rnd =rng.random_range(1..=10);
            println!("current thread index:{} gen random number:{}", i, rnd);
            *num += rnd;

            // 获取相加之后的数据
            let counter = *num;
            println!(
                "current thread index:{} counter after adding is:{}",
                i, counter
            );
        });

        // 将当前线程返回的结果JoinHandle<()>追加到handlers向量中
        handlers.push(handler);
    }

    // 在主线程中等待所有线程完成
    for handler in handlers {
        handler.join().unwrap();
    }

    // 打印最终的计数器值，由于在多个线程中随机增加计数器的值
    // 因此这个count的值可能每次运行都不一样
    let count = counter.lock().unwrap();
    println!("final counter value is {}", *count);
}
