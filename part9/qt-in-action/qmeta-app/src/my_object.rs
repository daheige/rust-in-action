use log::info;

// 导入qmetaobject相关的包
use qmetaobject::prelude::*;
use qttypes::QString; // 这是与Qt C++兼容的字符串类型

// 为Hello实现Default trait和QObject trait
#[derive(Default, QObject)]
pub struct Hello {
    // 使用qt_base_class宏指定基类为QObject
    base: qt_base_class!(trait QObject),
    // 使用qt_method宏包裹say_hello方法
    say_hello: qt_method!(fn(&self) -> ()),
}

// 为 Hello 实现say_hello方法
impl Hello {
    pub fn say_hello(&self) {
        println!("Hello world!");
    }
}

#[derive(Default, QObject)]
pub struct Rot {
    // 使用qt_base_class宏指定基类为QObject
    base: qt_base_class!(trait QObject),

    // 属性用 qt_property!包裹起来
    plain: qt_property!(QString; NOTIFY plain_changed),
    // Declare a signal
    plain_changed: qt_signal!(),

    secret: qt_property!(QString;NOTIFY secret_changed),
    // Declare a signal
    secret_changed: qt_signal!(),

    // 使用qt_method宏包裹md5方法
    md5: qt_method!(fn(&self, plain: String) -> QString),
}

impl Rot {
    // 实现md5加密，并将加密后的字符串返回
    pub fn md5(&self, plain: String) -> QString {
        if plain.is_empty() {
            return QString::from("plain is empty");
        }

        // 生成md5字符串
        let digest = md5::compute(&plain);
        let md5_str = format!("{:x}", digest);

        // 在终端中输出加密之前的字符串和加密后的字符串内容
        info!("plain:{} md5 string:{}", plain, md5_str);
        let result = format!("{}", md5_str);
        QString::from(result.as_str())
    }
}
