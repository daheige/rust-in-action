// 定义data模块，并引入data模块的数据结构
mod data;
use data::*;

fn main() {
    // 反序列化操作
    let resource_str = r##"{"name":"node1","hash":"xxx","password":"abc","version":"v1"}"##;
    let r: Resource = serde_json::from_str(&resource_str).unwrap();
    println!("resource:{:?}", r);

    // 枚举序列化为数字
    let days = vec![
        Day::Monday,
        Day::Tuesday,
        Day::Wednesday,
        Day::Thursday,
        Day::Friday,
        Day::Saturday,
        Day::Sunday,
    ];
    let res = serde_json::to_string(&days).unwrap();
    println!("days json encode:{}", res);

    // 将枚举序列化为字符串格式
    let colors = vec![Color::Red, Color::Green, Color::Blue];
    let res = serde_json::to_string(&colors).unwrap();
    println!("colors json encode:{}", res);

    // 序列化时，pagination字段平铺
    let users = Users {
        users: vec![User {
            username: "daheige".to_string(),
            nick: "hg".to_string(),
            password: "abc".to_string(),
        }],
        pagination: Pagination {
            limit: 10,
            offset: 0,
            total: 1,
        },
    };
    let res = serde_json::to_string(&users).unwrap();
    println!("users json encode:{}", res);

    // 将字段序列化为驼峰命名风格
    let p = Person {
        first_name: "Graydon".to_string(),
        last_name: "Hoare".to_string(),
    };
    let res = serde_json::to_string(&p).unwrap();
    println!("person json encode:{}", res);

    // 将字符串反序列化为Person对象
    let s = r##"{"firstName":"Graydon","lastName":"Hoare"}"##;
    let p: Person = serde_json::from_str(&s).unwrap();
    println!("person:{:?}", p);
}
