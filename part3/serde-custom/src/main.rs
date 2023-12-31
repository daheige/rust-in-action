// 引入serde库
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// 用户信息
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i64,
    name: String,
    lang: String,

    #[serde(skip)]
    is_married: bool,

    #[serde(
        serialize_with = "serialize_hobbies",
        deserialize_with = "deserialize_hobbies"
    )]
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

fn serialize_hobbies<S>(hobbies: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(hobbies.join(",").as_str())
}

// 返回Vec<String>和对应的错误error
fn deserialize_hobbies<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let v: Vec<&str> = s.split(",").collect();
    let mut arr = Vec::new();
    for val in v {
        arr.push(val.to_string())
    }
    Ok(arr)
}

fn main() {
    let p = Person {
        id: 1,
        name: "daheige".to_string(),
        lang: "rust".to_string(),
        is_married: true,
        hobbies: vec!["reading".to_string(), "music".to_string()],
        address: Address {
            city: "shenzhen".to_string(),
            street: "guangming".to_string(),
            post_code: 518000,
        },
    };
    let s = serde_json::to_string(&p).unwrap();
    println!("json encode person encode to str: {}", s);

    let p: Person = serde_json::from_str(&s).unwrap();
    println!("json decode person:{:?}", p);
    println!("person id:{},name:{} hobbies:{:?}", p.id, p.name, p.hobbies);
}
