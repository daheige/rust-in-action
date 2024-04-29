use pulsar::{producer, DeserializeMessage, Error as PulsarError, Payload, SerializeMessage};
use serde::{Deserialize, Serialize};

// 积分消息message
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PointsMessage {
    pub openid: String,
    pub points: u64,
    pub action: String,
    pub reason: String,
    pub created_at: String,
}

// 为PointsMessage实现pulsar SerializeMessage序列化
impl SerializeMessage for PointsMessage {
    // 实现消息序列化处理，返回producer::Message和PulsarError
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        // 将Message转换为Vec<u8>格式
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
    // 从pulsar Payload中解析出消息为Message
    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}
