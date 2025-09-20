use chrono::Local;
use std::env;

// 环境变量获取并输出
fn main() {
    // 输出当前时间
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("current_time:{}", current_time);

    println!("output rustup updates the environment variable");
    // 获取环境变量RUSTUP_DIST_SERVER
    let dist_server = env::var("RUSTUP_DIST_SERVER").unwrap_or("".to_string());
    println!("RUSTUP_DIST_SERVER:{}", dist_server);

    // 获取环境变量RUSTUP_UPDATE_ROOT
    let update_root = env::var("RUSTUP_UPDATE_ROOT").unwrap_or("".to_string());
    println!("RUSTUP_UPDATE_ROOT:{}", update_root);
}
