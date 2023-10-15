fn say_hello() {
    println!("hello");
}

pub mod mod1 {
    // 这里使用super关键字，来调用上一层的say_hello函数
    pub fn say_hello() {
        super::say_hello();
    }

    pub mod mod2 {
        fn say_hello() {
            println!("hello,world");
        }

        // 这里使用self关键字，来调用当前模块mod2的say_hello函数
        pub fn call() {
            println!("call mod1::mod2::say_hello");
            self::say_hello();
        }
    }
}

fn main() {
    mod1::say_hello(); // 调用mo1模块中的say_hello函数
    mod1::mod2::call(); // 调用mod2模块中的call函数，它通过self关键字调用到了mod2的say_hello函数
}
