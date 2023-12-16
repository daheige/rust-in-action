// io以及net模块相关的包
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::sync::{mpsc, mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let address = "localhost:8080";
    println!("server run on: {}", address);
    println!("server pid: {}", process::id());

    let pool = ThreadPool::new(8);
    // 通过TcpListener::bind方法，创建一个tcp TcpListener 连接实例，并绑定到对应的本地端口上
    let listener = TcpListener::bind(address).unwrap();
    // 监听tcp连接，下面的for可以循环处理每个连接产生的流
    // listener这里只处理8个请求，就平滑退出(使用的是pool Drop trait实现的)
    for stream in listener.incoming().take(8) {
        // 这里的stream表示客户端和服务端直接打开的连接，称作为流
        let stream = stream.unwrap(); // 调用unwrap方法获得流信息

        // 通过execute创建一个线程，让每个请求都使用单独的线程处理
        pool.execute(move || {
            println!("start handler request...");
            handler_connection(stream);
            println!("finish handler request");
        });
    }
}

// 处理客户端请求
fn handler_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // 通过读取stream流到buffer变量中
    stream.read(&mut buffer).unwrap();

    let long_page = b"GET /long HTTP/1.1\r\n";

    // 响应的body内容
    let mut content = r##"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="utf-8">
            <title>web-server</title>
          </head>
          <body>
            <h1>Hello,web-server</h1>
            <p>this is a demo</p>
          </body>
        </html>
    "##;

    // 判断请求路径是否是/long
    if buffer.starts_with(long_page) {
        println!("sleep 3s...");
        // 停顿3s
        thread::sleep(Duration::from_secs(3));
        content = r##"
            <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="utf-8">
            <title>web-server-long</title>
          </head>
          <body>
            <h1>web-server-thread</h1>
            <p>This is a long page</p>
          </body>
        </html>
        "##
    }

    // 设置http请求行，响应状态码200
    // 将content加入到将要写入流的成功返回的body中
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content,
    );

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    max_workers: usize,
    sender: mpsc::Sender<MessageEntry>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

// MessageEntry 定义枚举存储job或者中断信号，两者只能存放一个
enum MessageEntry {
    Task(Job),
    Terminate, // 退出信号
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        // 创建异步消息通道sender, receiver
        let (sender, receiver) = channel();

        // 引用计数的方式创建跨多线程安全的receiver
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        // 创建多个worker
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Self {
            workers,
            sender,
            max_workers: size,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 将满足具有Send特型，且具有static静态生命周期的FnOnce特型的闭包f,用Box包装后，变成了固定大小的job
        // FnOnce只能执行一次，也就是说闭包类型的所有权传递到了函数execute中
        // Send特型表明可以在线程之间转移所有权
        // ‘static这个闭包类型必须具有静态生命周期，也就是在f发送到通道中，在整个通道中闭包一直是有效的。
        let job = Box::new(f);
        self.sender.send(MessageEntry::Task(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    // 为ThreadPool实现Drop特征，用于等待每个线程执行完毕
    // 模拟每个线程平滑退出操作
    fn drop(&mut self) {
        println!("send terminate msg to workers");
        // 发送中断信号
        for _ in 0..self.max_workers {
            self.sender.send(MessageEntry::Terminate).unwrap();
        }

        // 等待所有的worker执行完毕
        println!("wait all workers thread finish...");
        for worker in &mut self.workers {
            println!("shutdown worker id:{}", worker.id);
            // 通过if let 解构得到thread线程
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    // 创建worker实例
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<MessageEntry>>>) -> Self {
        // 通过线程的方式从receiver接收者中获取通道中的任务，也就是execute发送的job闭包类型
        let thread = thread::spawn(move || {
            loop {
                let entry = receiver.lock().unwrap().recv().unwrap();
                // 判断entry是需要执行的job还是中断信号
                match entry {
                    MessageEntry::Task(job) => {
                        println!("worker {} got a job,executing...", id);
                        job();
                    }
                    MessageEntry::Terminate => {
                        // 如果是退出信号就退出当前循环，让其线程不再从receiver获取job执行
                        println!("worker {} will terminate...", id);
                        break;
                    }
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
