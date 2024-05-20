pub mod index;

use serde::{Deserialize, Serialize};

// 返回结果定义
#[derive(Deserialize, Serialize, Debug)]
pub struct Reply<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 空结构体定义 empty object,like {}
#[derive(Deserialize, Serialize, Debug)]
pub struct EmptyObject {}

// 空数组定义 empty array,like:[]
type EmptyArray = Vec<EmptyObject>;

// 定义分页请求参数
#[derive(Debug, PartialEq, Deserialize)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub page: Option<u64>,
}
