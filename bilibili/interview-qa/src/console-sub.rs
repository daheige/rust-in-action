use std::{thread::sleep, time::Duration};

// 运行方式 RUSTFLAGS="--cfg tokio_unstable" cargo run --bin console-sub
#[tokio::main]
async fn main() {
    console_subscriber::init(); // 启动时开启
    println!("hello,world");
    print_number().await;

    let mut i = 0;
    loop {
        i += 1;
        println!("current index1:{}", i);
        sleep(Duration::from_millis(100));
    }
}

async fn print_number() {
    tokio::spawn(async {
        let mut i = 0;
        loop {
            i += 1;
            println!("current index:{}", i);
            sleep(Duration::from_millis(100));
        }
    });
}
