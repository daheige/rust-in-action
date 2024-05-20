// build.rs
// 将src/foo.c编译为静态库libfoo.a文件
// 对于更复杂的构建需求，可能需要cc::Build为指定的include的路径和额外的编译器flag标志
fn main() {
    // 首先通过define方法来定义c语言的宏变量，分别是APP_NAME和VERSION
    // 然后通过file方法指定需要编译的c源码文件
    // 最后调用compile方法，指定编译输出的文件，也就说在debug/build中会生成类似cc-demo-xxx目录
    // 在该目录下会有一个编译之后的静态库libfoo.a文件
    cc::Build::new()
        .define(
            "APP_NAME",
            // 获取rust当前项目的名字
            format!("\"{}\"", env!("CARGO_PKG_NAME")).as_str(),
        )
        .define(
            "VERSION",
            // 获取rust当前项目的版本号
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", "\"YES\"") //  定义WELCOME宏
        .file("c_code/foo.c")
        .compile("foo");
}
