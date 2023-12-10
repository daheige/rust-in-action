// 引入serde_json和fs,io相关的包对应的类型
use serde_json::{self, Value};
use std::fs::{self, File};
use std::io::{Error, Read, Write};

// 定义配置文件读取的 trait
pub trait ConfigTrait {
    fn new(path: &str) -> Self;
    fn load(&mut self) -> Result<(), Error>;
    fn sections(&self) -> Value;
    fn content(&self) -> &str;
    fn write(&self, path: &str) -> Result<(), Error>;
}

// Config 定义
pub struct Config {
    config_file: String,
    sections: String,
}

// 实现 ConfigTrait
impl ConfigTrait for Config {
    // 创建一个ConfigTrait 实例
    fn new(path: &str) -> Self {
        let s = Config {
            config_file: path.to_string(),
            sections: String::new(),
        };
        s
    }

    // 从配置文件中加载json字符串，并将其读取到sections字段中
    fn load(&mut self) -> Result<(), Error> {
        File::open(&self.config_file)?.read_to_string(&mut self.sections)?;
        Ok(())
    }

    // 将sections字段的内容转化为serde_json::Value类型
    fn sections(&self) -> Value {
        let val = serde_json::from_str(&self.sections).unwrap();
        val
    }

    // 返回读取到的json字符串内容
    fn content(&self) -> &str {
        self.sections.as_str()
    }

    // 将读取的内容写入指定的文件中
    fn write(&self, path: &str) -> Result<(), Error> {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?
            .write(self.content().as_bytes())?;
        Ok(())
    }
}
