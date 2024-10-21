// 定义config模块
mod config;
use config::{load_config, write_config}; // 引入config模块中的函数

fn main() {
    let path = "app.json";
    let app_config = load_config(path).expect("failed to read json config");
    // 输出读取到的内容
    println!(
        "app_debug:{} app_name:{} app_port:{}",
        app_config.app_debug, app_config.app_name, app_config.app_port
    );

    // 将app_config序列化为json字符串，并写入文件中
    let s = serde_json::to_string(&app_config).expect("failed to serialize app_config");
    let res = write_config("app2.json", s).expect("failed to write file");
    println!("res:{}", res);
}
