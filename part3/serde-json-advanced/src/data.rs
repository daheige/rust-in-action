// 引入serde和serde_repr库中的相关模块
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub name: String,
    pub hash: String,

    // 通过skip属性跳过password字段序列化和反序列化处理
    #[serde(skip)]
    pub password: String,

    // 通过rename为字段重命名
    #[serde(rename = "version")]
    pub current_version: String,

    // 反序列化时，如果没有提供weight字段，默认为0
    #[serde(default)]
    pub weight: i32,

    // 反序列化时，如果没有提供path字段，默认为字段调用default_path函数
    #[serde(default = "default_path")]
    pub path: String,
}

fn default_path() -> String {
    "./".to_string()
}

// 将枚举类型序列化为字符串格式
#[derive(Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
}

// 定一个分页参数的结构体
#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub nick: String,

    // 通过skip属性跳过password字段序列化和反序列化处理
    #[serde(skip)]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub users: Vec<User>,

    // 通过flatten属性将结构体字段扁平化平铺
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Day {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 0,
}

// 通过rename_all = "camelCase"将字段序列化为驼峰命名风格
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
