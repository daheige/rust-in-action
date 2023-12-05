use serde_json::Value;
use std::collections::HashMap;

fn main() {
    // 定义hash map类型
    let mut m = HashMap::new();
    m.insert("id", 1);
    m.insert("post_code", 518000);
    let s = serde_json::to_string(&m).unwrap();
    println!("json str:{}", s);

    // 将s字符串反序列化为serde_json value对象类型
    let value: Value = serde_json::from_str(&s).unwrap();
    println!("id: {},post_code:{}", value["id"], value["post_code"]);
    println!(
        "call as_i64 method for id: {},post_code:{}",
        value["id"].as_i64().unwrap(),
        value["post_code"].as_i64().unwrap(),
    );
}
