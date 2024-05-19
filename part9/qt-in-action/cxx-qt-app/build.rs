// 引入cxx_qt_build包用于构建之前的操作
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qml_module(QmlModule::<&str, &str> {
            uri: "cxx_qt.myapp", // qt qml文件对应的包名，这个包名必须和qml文件一致
            // Generate C++ from the `#[cxx_qt::bridge]` module
            // 通过桥接的方式生成对应的C++代码
            rust_files: &["src/cxxqt_object.rs"],
            // Generate C++ code from the .qrc file with the rcc tool
            // 指定qml/qml.qrc文件
            qrc_files: &["qml/qml.qrc"],
            ..Default::default()
        })
        .build();
}
