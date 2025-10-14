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

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    city: String,
    street: String,
    post_code: String,
}

fn main() {
    let p = Person {
        id: 2,
        name: "daheige".to_string(),
        lang: "rust".to_string(),
        is_married: true,
        hobbies: vec!["reading".to_string(), "music".to_string()],
        address: Address {
            city: "shenzhen".to_string(),
            street: "guangming".to_string(),
            post_code: "518000".to_string(),
        },
    };

    println!("p:{:?}", p);

    // 将数据结构转换为JSON字符串
    let s = serde_json::to_string(&p).unwrap();
    // 或者使用下面的方式
    // let s = serde_json::to_string(&p).expect("failed to encode Person to json");
    println!("person encode to str: {}", s);

    // 将JSON字符串转换为Person结构体对象
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
            "post_code":"518000"
          }
        }
    "##;
    // from_str函数返回值是一个Result<T>类型，在这里T是Person结构体
    let res: serde_json::Result<Person> = serde_json::from_str(&s);
    match res {
        Ok(p) => {
            println!("person:{:?}", p);
            println!("person id:{},name:{} hobbies:{:?}", p.id, p.name, p.hobbies);
        }
        Err(err) => println!("failed to decode s to Person,err:{}", err),
    }

    // 或者使用下面的方式，直接反序列化为Person结构体对象
    // let p: Person = serde_json::from_str(&s).expect("failed to decode s to Person");
    // println!("person:{:?}", p);
    // println!("person id:{},name:{} hobbies:{:?}", p.id, p.name, p.hobbies);
}
