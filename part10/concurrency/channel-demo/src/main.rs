use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个channel通道
    // 其中sender表示发送者，receiver表示接收者
    let (sender, receiver) = mpsc::channel();

    // 创建一个新线程
    let handler = thread::spawn(move || {
        // 往通道中发送5条消息，消息内容是String类型，格式是: hello,index 1
        for i in 1..5 {
            let msg = format!("hello,index:{}", i);
            println!("Sent message: {}", msg);
            // 发送消息到通道中
            sender.send(msg).unwrap();
        }
    });

    handler.join().unwrap(); // 等待子线程执行完毕

    // 在主线程中接收消息，
    // 当发送通道被关闭后，接收端就会立即接收数据。
    // 由于receiver实现了Iterator，因此这里可以使用迭代器的方式，
    // 接收所有可用的消息，直到channel被关闭。
    // 这种方式简化了接收端的代码，特别是当需要处理所有消息，
    // 而不必关心接收的具体时机时。
    for received in receiver {
        println!("Received message: {}", received);
    }
}
