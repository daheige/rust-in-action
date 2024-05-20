use serde::{Deserialize, Serialize};

// 文章实体对象定义
// sqlx::FromRow 标记特征用于行记录读取
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ArticleEntity {
    pub id: u64,
    pub title: String,
    pub read_count: u64,
    pub content: String,
    pub author: String,
    pub is_deleted: u16,
}
