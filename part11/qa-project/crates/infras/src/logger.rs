// 引入了chrono::Local模块，主要用于日志记录时间格式的自定义
use chrono::Local;
// 引入Write trait，主要用于env_logger自定义日志写入格式
use std::io::Write;

pub struct Logger {
    is_custom: Option<bool>, // 日志初始化是否自定义
}

impl Logger {
    pub fn new() -> Self {
        Self { is_custom: None }
    }

    pub fn with_custom(mut self) -> Self {
        self.is_custom = Some(true);
        self
    }

    // 日志初始化
    // 其中日志level优先级从高到低：error > warn > info > debug > trace
    pub fn init(&self) {
        if self.is_custom.is_none() {
            // 如果你不关注日志时区的话，可以直接使用eng_logger默认方式初始化
            env_logger::init();
            return;
        }

        // env_logger env settings
        let env_config =
            env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
        let mut builder = env_logger::Builder::from_env(env_config);
        builder
            .format(|buf, record| {
                let level = record.level();
                // 通过default_styled_level方法设置不同level日志颜色标识
                // let level = buf.default_level_style(level);
                writeln!(
                    buf,
                    "{} {} [{}:{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"), // 时间格式
                    level,                                    // 日志级别
                    record.module_path().unwrap_or("unnamed"), // 模块名
                    record.line().unwrap_or(0),               // 行号
                    &record.args()                            // 日志message body内容
                )
            })
            .init();
    }
}
