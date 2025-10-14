# logger日志模块核心逻辑和剖析

在qa-project/crates/infras/src目录中新增logger.rs文件，并添加如下代码：

```rust
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
```

- 在上述代码中，首先，引入了chrono::Local模块和std::io::Write trait（特征）。其中chrono::Local主要用于日志记录时间格式的自定义，Write trait主要用于env_logger库自定义日志写入格式。
- 然后，定义了Logger结构体，is_custom字段，是一个Option bool类型，用于env_logger记录日志是否自定义格式。
- 接着，在impl块中为Logger添加了new、with_custom和init三个方法。在init方法中，首先判断is_custom字段是否有值，如果没有设置，就直接使用env_logger::init()初始化日志配置，否则就调用env_logger::Builder上的from_env传入env_config参数，创建一个env_logger实例对象builder。
- 然后，调用builder.format方法自定义env_logger日志格式。format方法接收一个闭包函数，第一个参数是buf，其类型是Formatter可变引用；第二个参数record，其类型是log::Record不可变引用。在闭包中通过record.level方法获取日志level，如果需要对日志内容染色处理，你可以在buf上面调用deafult_level_style方法开启日志染色功能。
- 接着，使用writeln!宏自定义日志内容格式，第一个参数是buf，第二个参数是日志format占位符，其他参数依次是日志时间格式、日志级别、模块名、行号以及日志message body内容。
- 最后，当初始化日志格式后，在builder上面调用init方法完成了整个日志自定义功能。

