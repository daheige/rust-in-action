// 引入pulsar包涉及到的基本模块
use pulsar::{producer, DeserializeMessage, Error as PulsarError, Payload, SerializeMessage};
use serde::{Deserialize, Serialize};

// 积分消息Message定义
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PointsMessage {
    pub openid: String,             // 用户唯一标识
    pub points: u64,                // 积分数
    pub action: String,             // 积分操作，add表示增加，sub表示扣除
    pub reason: String,             // 积分操作理由
    pub created_at: Option<String>, // 积分创建时间
}

// 为PointsMessage实现pulsar SerializeMessage序列化
impl SerializeMessage for PointsMessage {
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

// 为PointsMessage实现pulsar SerializeMessage反序列化
impl DeserializeMessage for PointsMessage {
    type Output = Result<PointsMessage, serde_json::Error>;
    // 从pulsar Payload中解析出消息为PointsMessage
    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}
