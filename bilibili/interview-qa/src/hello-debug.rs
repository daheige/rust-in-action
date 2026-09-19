use std::thread;
use std::thread::sleep;
use std::time::Duration;

// 当前二进制文件的git提交信息和构建时间
pub const VERSION_STR: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "+",
    env!("GIT_COMMIT"),
    " @ ",
    env!("BUILD_TIME")
);

// 如果没有设置hook，可以通过下面的命令查看对应的panic堆栈信息
// RUST_BACKTRACE=full ./target/release/hello-debug
fn main() {
    println!("Hello, world!");
    println!("current version: {}", VERSION_STR);
    // current version: 0.1.0+798902c @ 2026-09-19T03:01:52Z
    // 设置全局panic钩子，打印日志
    std::panic::set_hook(Box::new(|info| {
        let info = info.location().unwrap();
        println!("panic info:{:?}", info.to_string());
        println!(
            "location file: {} line: {} column:{}",
            info.file(),
            info.line(),
            info.column()
        );
    }));

    print_number();

    let mut i = 0;
    loop {
        i += 1;
        println!("current index1:{}", i);
        sleep(Duration::from_millis(100));
    }
}

fn print_number() {
    thread::spawn(|| {
        let mut i = 0;
        loop {
            i += 1;
            if i == 100 {
                panic!("current index:{} exec panic", i);
            } else {
                println!("current index:{}", i);
            }

            sleep(Duration::from_millis(100));
        }
    });
}
