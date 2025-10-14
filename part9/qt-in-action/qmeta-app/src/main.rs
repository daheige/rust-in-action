use chrono::Local;
use cstr::cstr;
use log::info;
use qmetaobject::prelude::*;
use std::io::Write;

// 配置资源文件qml
// as前面的qml是文件目录，as后面的是虚拟路径
qrc!(qml_resource,
    "qml" as "qml" {
        "main.qml",
    },
);

// 定义模块 my_object
mod my_object;

fn main() {
    // 日志初始化
    // 其中日志level优先级从高到低：error > warn > info > debug > trace
    // 如果想在启动时改变日志级别，可以通过指定环境变量启动应用
    // 启动方式：RUST_LOG=debug cargo run
    // 过滤日志级别，该应用程序默认是debug级别
    let env_config = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");
    let mut builder = env_logger::Builder::from_env(env_config);
    builder
        .format(|buf, record| {
            // 通过default_level_style方法设置不同level日志颜色标识
            // let level = record.level();
            let level = buf.default_level_style(record.level());
            writeln!(
                buf,
                "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // 时间格式
                level,                                    // 日志级别
                record.module_path().unwrap_or("unnamed"), // 模块名
                record.line().unwrap_or(0),               // 行号
                &record.args()                            // 格式化参数
            )
        })
        .init();

    // 如果你不关注日志时区的话，可以直接使用env_logger::init初始化日志配置
    // env_logger::init();

    // qmetaobject日志初始化
    // 如果注释掉下面qmetaobject日志初始化，那qmetaobject日志就会以qml作为前缀输出到终端
    qmetaobject::log::init_qt_to_rust();

    info!("qml resource init...");
    // 注册资源qt qml资源
    qml_resource();

    info!("register qml type...");
    // 注册自定义类型Hello和Rot
    qml_register_type::<my_object::Hello>(cstr!("qmetaobject.myapp"), 1, 0, cstr!("Hello"));
    qml_register_type::<my_object::Rot>(cstr!("qmetaobject.myapp"), 1, 0, cstr!("Rot"));

    // 创建qml engine
    let mut engine = QmlEngine::new();

    info!("load qml file...");
    // 加载qml文件内容
    engine.load_file(QString::from("qrc:/qml/main.qml"));

    info!("run app...");

    // 启动app
    engine.exec();
}
