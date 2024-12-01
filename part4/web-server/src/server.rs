// io以及net模块相关的包
use ctrlc;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let address = "localhost:8080";
    println!("server run on: {}", address);

    // 平滑退出在独立的线程中执行
    let shutdown_handler = thread::spawn(|| {
        graceful_shutdown();
    });

    let handler = thread::spawn(move || {
        // 通过TcpListener::bind方法，创建一个tcp TcpListener 连接实例，并绑定到对应的本地端口上
        let listener = TcpListener::bind(address).unwrap();
        // 监听tcp连接
        // 下面的for可以循环处理每个连接产生的流
        for stream in listener.incoming() {
            // 这里的stream表示客户端和服务端直接打开的连接，称作为流
            let stream = stream.unwrap(); // 调用unwrap方法获得流信息

            // 通过spawn创建一个线程，让每个请求都使用单独的线程处理
            thread::spawn(|| {
                println!("start handler request...");
                handler_connection(stream);
                println!("finish handler request");
            });
        }
    });

    handler.join().unwrap();

    // 等平滑退出函数执行完毕
    shutdown_handler.join().unwrap();
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

    // 设置http请求行，响应状态码200，
    // 并将content加入到将要写入流的成功返回的body中
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content,
    );

    // 如果请求地址非/favicon.ico开头，就输出对应的请求日志
    if !buffer.starts_with(b"GET /favicon.ico HTTP/1.1\r\n") {
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// 使用ctrlc包实现服务平滑退出
fn graceful_shutdown() {
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");

    // do somethings...

    println!("Got it! Exiting...");
    thread::sleep(Duration::from_secs(5));
    process::exit(0);
}
