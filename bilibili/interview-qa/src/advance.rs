use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::time;

// 生命全局变量static修饰
static mut APP_DEBUG: bool = true;

// AtomicBool 在 x86 上就是一条指令，性能上没有任何可感知差异。
// 除非你在写操作系统底层、FFI 边界这类必须和 static mut 打交道的代码，否则都应该选 AtomicBool。
static APP_DEBUG2: AtomicBool = AtomicBool::new(true);

// cargo run --bin advance-qa
#[tokio::main]
async fn main() {
    println!("hello world");
    // =====闭包=======
    // 在 Rust 中，闭包（closures）是一种可以捕获其创建环境中的变量的匿名函数。它们允许你定义一个临时的一次性函数，
    // 可以在任何地方使用，并且能够访问外部作用域内的数据。闭包有三种捕获机制，
    // 它们对应于三个不同的 Fn 属性：Fn, FnMut, 和 FnOnce
    let x = 5;
    // Fn trait 类型的闭包
    let add_x = |y| y + x; // 这个闭包捕获x不可变借用

    let value = add_x(5); // 这里自动推导y是i32类型
    println!("The value of x is: {}", value);

    // FnMut trait类型的闭包
    let mut x = 5;
    let mut change_value = || {
        x += 1;
        x
    };
    // 调用闭包函数
    println!("The value is: {}", change_value());

    // FnOnce trait
    let s = String::from("hello");
    let take = || {
        println!("{}", s);
        drop(s); // 这里主动释放s
    };
    // println!("s:{}", s); // ^ value borrowed here after move
    take();

    // await 关键字：await 关键字用于暂停异步函数的执行，直到给定的 Future 对象完成。
    // 这样就可以处理异步操作的结果而不阻塞其他任务。
    // 通过await来驱动异步任务执行
    let v = my_task().await;
    println!("v:{}", v);

    exec_task().await;
    task2().await;

    mutex_task();
    arc_task();

    channel_task().await;
    select_task().await;

    unsafe {
        APP_DEBUG = false;
        // addr_of! 产生 *const bool 裸指针，不创建引用
        // println!("APP_DEBUG: {}", *std::ptr::addr_of!(APP_DEBUG));

        // 读的时候用 read_volatile，语义上更明确，每次都真正去内存读，不优化缓存
        println!(
            "APP_DEBUG: {}",
            std::ptr::read_volatile(std::ptr::addr_of!(APP_DEBUG))
        );
    }

    APP_DEBUG2.store(false, Ordering::SeqCst);
    println!("APP_DEBUG2: {}", APP_DEBUG2.load(Ordering::SeqCst));
}

fn mutex_task() {
    // 创建一个包含整数的 Mutex
    let mutex = Mutex::new(0);

    // 获取 Mutex 的可变引用，并增加其内部值
    {
        let mut num = mutex.lock().unwrap();
        // 解引用方式获取值，并修改
        *num += 2;
    }

    // 通过 Mutex 再次读取并打印内部值
    {
        let num = mutex.lock().unwrap();
        println!("The value is: {}", num);
    }
}

fn arc_task() {
    // 创建一个包含整数的 Arc 和 Mutex
    // 原子引用+mutex组合使用，可以在多个线程中实现数据读写
    let data = Arc::new(Mutex::new(0));

    // 创建两个 Arc 的克隆并将它们传递给新线程
    let thread_data1 = Arc::clone(&data);
    let thread_data2 = Arc::clone(&data);

    let t1 = thread::spawn(move || {
        // 获取 Mutex 的可变引用，并在第一个线程中增加其内部值
        let mut num = thread_data1.lock().unwrap();
        *num += 1;
    });

    let t2 = thread::spawn(move || {
        // 获取 Mutex 的可变引用，并在第二个线程中增加其内部值
        let mut num = thread_data2.lock().unwrap();
        *num += 2;
    });

    // 等待线程执行完毕
    t1.join().unwrap();
    t2.join().unwrap();

    // 通过 Mutex 再次读取并打印内部值
    {
        let num = data.lock().unwrap();
        println!("The final value is: {}", num);
    }
}

fn increment_counter(counter: &Arc<Mutex<i32>>) {
    let mut num = counter.lock().unwrap();
    *num += 1;
}

fn data_race() {
    let counter = Arc::new(Mutex::new(0));
    let thread_data = counter.clone();

    let value = counter.clone();
    let t1 = thread::spawn(move || increment_counter(&value));
    // let t1 = thread::spawn(move || increment_counter(&thread_data)); // 无法编译
    let t2 = thread::spawn(move || increment_counter(&thread_data));

    t1.join().unwrap();
    t2.join().unwrap();

    println!("The final value is: {}", *counter.lock().unwrap());
}

async fn channel_task() {
    // 创建一个通道，其缓冲区大小为 10
    // 这里使用 tokio::sync::mpsc 多生产者/单消费者模式
    let (tx, mut rx) = mpsc::channel(10);

    // 在后台启动一个新的异步任务来监听通道
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Received: {}", msg);
        }
    });

    // 发送一些消息到通道
    for i in 0..10 {
        tx.send(i).await.unwrap();
    }

    // 延迟一段时间以确保所有消息都被打印出来
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn select_task() {
    let timeout = time::sleep(Duration::from_millis(200));
    let mut interval = tokio::time::interval(Duration::from_millis(100));

    // tokio::select! 宏会同时监控这两个异步操作，并在其中一个准备好时立即执行相应的代码块。
    tokio::select! {
        _ = timeout => {
            println!("The timeout occurred first!");
        },
        _ = interval.tick() => {
            println!("The interval ticked first!");
        },
    }
}

async fn query_cache() {
    println!("mock query_cache");
    // 模拟超时
    time::sleep(Duration::from_secs(1)).await;
}

async fn exec_task() {
    // 单生产者/单消费者
    let (tx, rx) = oneshot::channel();

    // 启动异步任务执行
    tokio::spawn(async move {
        let _ = tx.send(query_cache().await);
    });

    tokio::select! {
        result = rx => {
            println!("缓存/计算完成: {:?}", result);
        }
        _ = time::sleep(Duration::from_secs(2)) => {
            println!("等不及了，先返回降级结果");
        }
    }
}

async fn task2() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    // 执行异步任务1
    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    // 执行异步任务2
    tokio::spawn(async {
        let _ = tx2.send("two");
        time::sleep(Duration::from_secs(1)).await;
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
        _ = time::sleep(Duration::from_secs(2)) => {
            println!("等不及了，先返回降级结果");
        }
    }
}

async fn my_task() -> i32 {
    12
}
