// 自定义错误类型CustomError
// 这里错误信息String放入一个元组结构体中
#[derive(Debug)]
pub struct CustomError(pub String);
