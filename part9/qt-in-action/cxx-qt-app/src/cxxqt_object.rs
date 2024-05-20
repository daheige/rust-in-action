use cxx_qt_lib::QString;
use rand::Rng;

// 定义CXX-Qt bridge module部分，用来为qml文件生成指定的数据类型
// #[cxx_qt::bridge]与#[cxx::bridge]相同，你可以在其中使用cxx的所有功能。
// 此外，这个#[cxx_qt::bridge]给了你更多的特性，
// 允许你从Rust中创建qobject或声明现有的qobject以便从Rust代码中可以访问。
// 比如下面通过type别名定义的Hello结构体和RandObj结构体，就可以通过impl实现ffi外部函数接口
#[cxx_qt::bridge]
pub mod qobject {
    // 引入c++ cxx_qt_lib 包中的rust QString
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        /// 在cxx-qt来说，QString是未知的。
        /// 幸运的是cxx_qt_lib crate已经为我们包装了许多Qt类型
        /// 因此这里直接通过include!宏导入对应的头文件，然后通过type别名的方式设置为QString
        /// 这个QString类型就可以在rust代码中使用
        type QString = cxx_qt_lib::QString;
    }

    // 定义ffi数据类型
    // QObject对应的数据类型定义
    unsafe extern "RustQt" {
        // Hello 类型定义
        #[qobject]
        #[qml_element]
        #[qproperty(QString, plain)] // qml文件中的Hello.plain字段类型为QString
        type Hello = super::HelloRust; // 这个qml文件的中类型名字不能和rust数据类型名字一样

        // RandObj 类型定义
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(i32, number2)]
        type RandObj = super::RandObjRust;
    }

    // 定义ffi外部函数
    unsafe extern "RustQt"{
        // say_hello函数返回值是字符串类型QString
        #[qinvokable]
        fn say_hello(self: &Hello) -> QString;

        // gen_number函数返回值是i32类型，对于i32, cxx-qt库对应的就是32位的数字
        #[qinvokable]
        fn gen_number(self:&RandObj,m:i32,n:i32) -> i32;
    }
}

// 定义rust结构体对象HelloRust
#[derive(Default)]
pub struct HelloRust {
    // plain字符串类型，必须要使用cxx-qt-lib/qstring.h中的QString类型
    plain: QString
}

#[derive(Default)]
pub struct RandObjRust {
    number:i32,
    number2:i32
}

// 为 Hello 实现say_hello方法
impl qobject::Hello {
    pub fn say_hello(&self) -> QString {
        println!("call say_hello from rust");
        QString::from("hello,world")
    }
}

impl qobject::RandObj {
    // 生成指定返回内的随机数
    fn gen_number(&self,m:i32,n:i32) -> i32 {
        println!("call gen_number from rust");
        // 这个gen_range方法生成的数字是一个半开区间，也就说[1,101)不包含101，它是1-100之间的数字
        let rnd : i32 = rand::thread_rng().gen_range(m..n); // 随机生成一个数字
        rnd
    }
}
