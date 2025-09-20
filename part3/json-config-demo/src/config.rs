use serde::{Deserialize, Serialize}; // 引入serde库
use std::path::Path;
use std::{fs, io};

// 读取文件内容
fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
    println!("content:{}", s);
    Ok(s)
}

// 将内容写入文件中，返回值是标准库中的Result
pub fn write_config<P: AsRef<Path>>(path: P, content: String) -> Result<String, io::Error> {
    fs::write(path, content)?;
    Ok("write success".to_string())
}

// 定义配置结构体
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    // 调用函数设置默认值
    #[serde(default = "default_app_debug")]
    pub app_debug: bool,

    pub app_name: String,

    #[serde(default = "default_app_env")]
    pub app_env: String,

    // 调用函数设置默认值
    #[serde(default = "default_app_port")]
    pub app_port: u16,

    #[serde(default)]
    pub token: String,
}

fn default_app_env() -> String {
    "development".to_string()
}

fn default_app_debug() -> bool {
    false
}

fn default_app_port() -> u16 {
    1338
}

// 从JSON配置文件中加载配置内容到AppConfig结构体对象中
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<AppConfig, io::Error> {
    // 读取文件
    let content = read_file(path)?;

    // 解析JSON字符串内容到AppConfig结构体对象app_config中
    let app_config: AppConfig =
        serde_json::from_str(&content).expect("failed to deserialize content");
    println!("config:{:#?}", app_config);
    Ok(app_config)
}
