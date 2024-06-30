use chrono::NaiveDateTime;

// VoteMessage entity vote message
pub struct VoteMessage {
    pub target_id: u64,            // 实体id
    pub target_type: String,       // 实体类型
    pub created_by: String,        // 创建者
    pub action: String,            // 执行动作：点赞up，取消点赞cancel
    pub created_at: NaiveDateTime, // 创建时间
}
