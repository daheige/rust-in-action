// 实体阅读数对应的基本数据信息
#[derive(Debug, Default)]
pub struct EntityReadCountData {
    pub target_id: u64,      // 实体id
    pub target_type: String, // 实体类型
    pub count: u64,          // 增量阅读数
}
