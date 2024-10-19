// 定义service模块
mod service {
    pub mod user {
        pub fn say_hello(name: String) {
            println!("hello,{}", name);
        }
        pub fn eat() {
            println!("eat something");
        }
    }

    pub mod feed {
        pub fn show() {
            println!("feed show");
        }
    }
}

fn main() {
    // 使用模块中的函数
    service::user::say_hello("xiaoming".to_string());
    service::user::eat();
    service::feed::show();
}
