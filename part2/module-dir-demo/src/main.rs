mod auth;
mod feed;

use auth::post; // 引入auth中的post模块

fn main() {
    // 调用feed模块的函数
    feed::show();
    feed::edit("hello".to_string());
    feed::summary();

    // 调用post模块中的函数
    post::show("rust lang".to_string());
}
