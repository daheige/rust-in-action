mod service {
    // 通过pub关键字公开user模块
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

    // 通过pub关键字公开feed模块
    pub mod feed {
        // 通过pub关键字公开show函数
        pub fn show() {}
    }
}

fn main() {
    // 使用模块中的函数
    service::user::say_hello("xiaoming".to_string());
    service::user::eat();
    service::feed::show();
}
