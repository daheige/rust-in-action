pub mod index;

use serde::{Deserialize, Serialize};

// 返回结果定义
#[derive(Deserialize, Serialize, Debug)]
pub struct Reply<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 空结构体定义
// empty object,like {}
#[derive(Deserialize, Serialize, Debug)]
pub struct EmptyObject {}

// empty array,like:[]
type EmptyArray = Vec<EmptyObject>;
