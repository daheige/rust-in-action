mod outer_mod {
    pub mod my_mod {
        // 此函数仅仅在模块内部可见，在模块内部私有，只能在模块内中调用
        pub(self) fn private_function() {
            println!("called my_mod::private_function");
        }

        // 此函数仅对当前包（crate）以及同包中的其他模块内可见
        pub(crate) fn crate_visible_fn() {
            println!("called my_mod::crate_visible_fn");
        }

        // 对my_mod模块可见，可以被外部以及当前模块中调用
        pub fn public_function() {
            println!("called my_mod::public_function");

            // 可以在同一模块内调用private_function函数
            private_function();

            // 调用当前包中的crate_visible_fn函数
            crate_visible_fn();
        }

        // 此函数仅仅在outer_mod中可见
        pub(super) fn super_mod_visible_fn() {
            println!("called my_mod::super_mod_visible_fn");
            // 在同一模块内调用private_function函数
            private_function();
        }
    }

    pub fn foo() {
        println!("call outer_mod::foo");
        // 在outer_mod中调用super_mod_visible_fn函数
        my_mod::super_mod_visible_fn();

        // 下面代码无法运行，因为不能调用my_mod中的私有函数private_function
        // my_mod::private_function();
    }
}

// 引入my_mod模块
use outer_mod::my_mod;
fn main() {
    my_mod::public_function(); // 可以调用成功
    outer_mod::foo(); // 调用foo函数

    // 下面的代码将编译错误，无法运行，
    // 因为super_mod_visible_fn仅仅在my_mod上一级父模块outer_mod中可见
    // my_mod::super_mod_visible_fn();

    // 下面的代码将编译错误，无法运行，
    // 因为private_function函数仅仅在定义的模块内部可见
    // my_mod::private_function()
}
