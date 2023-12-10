// 声明config模块，用于配置文件读写
mod config;

use config::Config;
use config::ConfigTrait;

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct AppConfig {
    app_debug: bool,
    app_name: String,
    app_port: i32,
}

fn main() {
    let mut conf = Config::new("app.json");
    conf.load().expect("load config failed");

    // 输出读取到的json文件内容
    let content = conf.content();
    println!("content:{:?}", content);

    // 输出serde_json::Value类型
    let sections = conf.sections();
    println!("sections:{:?}", sections);

    // 将读取到的内容content反序列化到AppConfig结构体
    let app_config: AppConfig = serde_json::from_str(content).expect("deserialize content failed");
    println!(
        "app_debug:{} app_name:{} app_port:{}",
        app_config.app_debug, app_config.app_name, app_config.app_port
    );

    // 将读到的内容写入app2.json文件中
    conf.write("app2.json").expect("write file failed");
}
