# vote_message.rs模块剖析

```rust
// domain/entity/vote_message.rs 文件
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
        // 序列化处理，将VoteMessage转换为Vec<u8>格式
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
```

- 在上述代码中，首先引入了pulsar库和serde库的相关模块。
- 然后定义了VoteMessage结构体，在其上方使用derive派生宏的方式使得VoteMessage具备了debug、序列化和反序列化、Default（字段默认值）等功能。
- 接着，为VoteMessage实现了pulsar库的DeserializeMessage、SerializeMessage
  trait用于pulsar消息序列化处理，也就是可以将消息VoteMessage序列化为pulsar::producer::
  Message，以及将pulsar消息反序列化为VoteMessage。
- 从repository/vote.rs文件的代码可知，consumer方法的第二个参数target_type是实体类型，第三个参数exit:Arc<RwLock<bool>>
  ，它是一个引用计数bool类型的读写锁，用于消费者退出标识（当程序接收到退出信号量时，这个消费操作就会退出）。
