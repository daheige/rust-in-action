// 定义user模块
mod user;

fn main() {
    // 调用user模块中的函数
    user::say_hello("daheige".to_string());
    user::eat();
    user::walk();
    user::water();
}
