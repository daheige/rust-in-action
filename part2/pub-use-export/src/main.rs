mod service {
    pub mod user {
        // 通过pub关键字公开say_hello函数
        pub fn say_hello(name: String) {
            println!("hello,{}", name);
        }

        // 通过pub关键字公开eat函数
        pub fn eat() {
            println!("eat something");
        }
    }

    pub mod feed {
        // 通过pub关键字公开show函数
        pub fn show() {}
    }
}

// 通过pub use重新导出feed模块和user模块
pub use crate::service::feed;
pub use crate::service::user;

fn main() {
    // 使用模块中的函数
    user::say_hello("xiaoming".to_string());
    user::eat();
    feed::show();
}
