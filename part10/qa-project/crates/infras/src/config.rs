// yaml配置文件读取封装
use serde_yaml::{self, Value};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// ConfigTrait define config trait
pub trait ConfigTrait {
    fn load<P: AsRef<Path>>(path: P) -> Self;
    fn sections(&self) -> Value;
    fn content(&self) -> &str;
}

// Config impl config trait
pub struct Config {
    sections: String,
}

impl ConfigTrait for Config {
    fn load<P: AsRef<Path>>(path: P) -> Self {
        let mut c = Self {
            sections: String::new(),
        };

        // 读取配置文件内容
        File::open(path)
            .expect("open config file")
            .read_to_string(&mut c.sections)
            .expect("failed to read config file");
        c
    }

    fn sections(&self) -> Value {
        let val = serde_yaml::from_str(&self.sections).unwrap();
        val
    }

    fn content(&self) -> &str {
        self.sections.as_str()
    }
}
