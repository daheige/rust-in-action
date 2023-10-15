// 通过mod关键字声明user模块
mod user;

// 将feed模块放入文件夹feed中
mod feed;

// 定义auth模块
mod auth;
use auth::post; // 引入auth中的post模块

fn main() {
    // 调用user模块的函数
    user::say_hello("xiaoming".to_string());
    user::eat();

    // 调用feed模块的函数
    feed::show();
    feed::edit("hello".to_string());
    feed::summary();

    // 调用post模块中的函数
    post::show("rust lang".to_string());
}
