use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个通道，返回值是sender生产者和receiver消费者
    let (sender, receiver) = mpsc::channel();

    // Sender<T>是一种满足Copy特征，这意味着它可以克隆发送方多次，
    // 将消息发送到同一通道，但只支持一个接收方。
    let sender1 = sender.clone(); // 通过clone方法显式克隆一个生产者
    thread::spawn(move || {
        let s = vec!["hello".to_string(), "rust".to_string()];
        for val in s {
            println!("sender1 sent msg:{}", val);
            sender1.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let s = String::from("hello,world");
        println!("sender sent msg:{}", s);
        sender.send(s).unwrap();
    });

    // 接收消息
    // 这里把接收端当作迭代器来使用，这样就不需要显式调用recv方法接收消息
    for msg in receiver {
        println!("Received msg: {}", msg);
    }
}
