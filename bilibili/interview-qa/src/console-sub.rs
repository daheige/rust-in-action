use std::{thread::sleep, time::Duration};

// 运行方式 RUSTFLAGS="--cfg tokio_unstable" cargo run --bin console-sub --features tokio-console
// feature 控制代码，RUSTFLAGS 控制 tokio，release 构建天然关闭观测
#[tokio::main]
async fn main() {
    // 只有在开启了 tokio-console 特性才生效
    #[cfg(feature = "tokio-console")]
    console_subscriber::init();

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
