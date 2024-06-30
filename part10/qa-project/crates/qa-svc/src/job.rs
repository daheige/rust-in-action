use std::process;
use chrono::Local;

mod config;

fn main() {
    // 初始化日志,这里采用自定义日志输出
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .format(|buf, record| {
            let file = record.file().unwrap_or("??"); // 文件名
            let line = record.line().unwrap_or(0); // 行号
            writeln!(
                buf,
                "{} [{}] - file:{}#{} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // 本地时间格式
                record.level(),
                file,
                line,
                record.args()
            )
        })
        .init();
    println!("current process pid:{}", process::id());
    println!("hello,qa-job");
}
