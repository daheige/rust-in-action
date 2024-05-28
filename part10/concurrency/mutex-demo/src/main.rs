use std::sync::Mutex;

fn main() {
    // 创建一个互斥锁来保护共享数据
    let mutex = Mutex::new(0);
    // 在一个闭包中获取互斥器的锁
    let f = || {
        let mut count = mutex.lock().unwrap();
        *count += 1; // 离开作用域时，锁自动释放
    };
    f();

    // 在另一个闭包中获取互斥器的锁
    let f2 = || {
        let mut count = mutex.lock().unwrap();
        *count += 2;
    };
    f2();

    // 在主线程中获取互斥锁，并打印共享数据
    let count = mutex.lock().unwrap();
    println!("Shared data count: {}", *count);
}
