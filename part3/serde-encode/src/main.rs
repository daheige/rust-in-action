// 引入serde库
use serde::{Deserialize, Serialize};

// 用户信息
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i64,
    name: String,
    lang: String,
    is_married: bool,
    hobbies: Vec<String>,
    address: Address,
}

// 地址信息
#[derive(Serialize, Deserialize, Debug)]
struct Address {
    city: String,
    street: String,
    post_code: i32,
}

fn main() {
    let p2 = Person{
        id: 2,
        name: "daheige".to_string(),
        lang: "rust".to_string(),
        is_married: true,
        hobbies: vec!["reading".to_string(),"music".to_string()],
        address: Address {
            city: "shenzhen".to_string(),
            street: "guangming".to_string(),
            post_code: 518000,
        },
    };
    let s = serde_json::to_string(&p2).unwrap();
    println!("person encode to str: {}",s);
}
