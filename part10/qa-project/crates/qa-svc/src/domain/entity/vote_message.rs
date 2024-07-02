use chrono::NaiveDateTime;
// 引入pulsar包涉及到的基本模块
use pulsar::{producer, DeserializeMessage, Error as PulsarError, Payload, SerializeMessage};
use serde::{Deserialize, Serialize};

// VoteMessage entity vote message
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VoteMessage {
    pub target_id: u64,      // 实体id
    pub target_type: String, // 实体类型
    pub created_by: String,  // 创建者
    pub action: String,      // 执行动作：点赞up，取消点赞cancel
}

// 为 VoteMessage 实现pulsar SerializeMessage序列化
impl SerializeMessage for VoteMessage {
    // 实现消息序列化处理，返回producer::Message和PulsarError
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        // 将PointsMessage转换为Vec<u8>格式
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,              // pulsar Message主体payload
            ..Default::default()  // pulsar Message 其他字段采用默认设置
        })
    }
}

// 为 VoteMessage 实现pulsar SerializeMessage反序列化
impl DeserializeMessage for VoteMessage {
    type Output = Result<VoteMessage, serde_json::Error>;
    // 从pulsar Payload中解析出消息为PointsMessage
    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}
