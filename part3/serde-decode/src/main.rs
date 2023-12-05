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
    let s = r##"
        {
          "id": 1,
          "name": "daheige",
          "lang": "rust",
          "is_married":true,
          "hobbies":["reading","music"],
          "address":{
            "city":"shenzhen",
            "street":"guangming",
            "post_code":518000
          }
        }
    "##;
    let p : Person = serde_json::from_str(&s).unwrap();
    println!("person:{:?}",p);
    println!("person id:{},name:{} hobbies:{:?}",p.id,p.name,p.hobbies);
}

