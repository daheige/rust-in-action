use std::process;
use crate::config::APP_CONFIG;

// 定义grpc 代码生成的模块
mod rust_grpc;

// 定义配置模块和基础设施模块
mod config;
mod infras;
fn main() {
    env_logger::init(); // 初始化操作日志配置

    println!("app_debug:{:?}", APP_CONFIG.app_debug);
    println!("current process pid:{}", process::id());

    println!("Hello, qa-sys!");
}
