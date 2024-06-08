// yaml配置文件读取封装
use serde_yaml::{self, Value};
use std::fs::File;
use std::io::{Error, Read};

// ConfigTrait define config trait
pub trait ConfigTrait {
    fn new(path: &str) -> Self;
    fn load(&mut self) -> Result<(), Error>;
    fn sections(&self) -> Value;
    fn content(&self) -> &str;
}

// Config impl config trait
pub struct Config {
    filepath: String,
    sections: String,
}

impl ConfigTrait for Config {
    fn new(filepath: &str) -> Self {
        let s = Config {
            filepath: filepath.to_string(),
            sections: String::new(),
        };
        s
    }

    fn load(&mut self) -> Result<(), Error> {
        File::open(&self.filepath)
            .expect(format!("load file:{} failed", &self.filepath).as_str())
            .read_to_string(&mut self.sections)?;
        Ok(())
    }

    fn sections(&self) -> Value {
        let val = serde_yaml::from_str(&self.sections).unwrap();
        val
    }

    fn content(&self) -> &str {
        self.sections.as_str()
    }
}
