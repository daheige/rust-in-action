// 定义cxx qt object module
mod cxxqt_object;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

// 这个main函数启动时候，就像c++中启动QML应用程序一样
fn main() {
    println!("cxx-qt-app run...");
    // 创建qt gui app应用
    let mut app = QGuiApplication::new();
    // 创建qt gui app qml engine
    let mut engine = QQmlApplicationEngine::new();

    // 将qml文件加载到app中
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qml/main.qml"));
    }

    // 启动gui app应用
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
