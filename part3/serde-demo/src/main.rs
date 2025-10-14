// 引入serde_json库
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    // 声明一个hash map类型变量m
    let mut m = HashMap::new();
    // 插入数据
    m.insert("id", 1);
    m.insert("post_code", 518000);

    // 使用serde_json库将m转换为JSON字符串
    let s = serde_json::to_string(&m).unwrap();
    println!("json str:{}", s);

    // 将s字符串，也就是JSON字符串反序列化为serde_json value对象类型
    let value: Value = serde_json::from_str(&s).unwrap();
    println!("id: {},post_code:{}", value["id"], value["post_code"]);
    println!(
        "call as_i64 method for id: {},post_code:{}",
        value["id"].as_i64().unwrap(),
        value["post_code"].as_i64().unwrap(),
    );
}
