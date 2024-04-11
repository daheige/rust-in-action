// 导入pulsar包
use pulsar::{
    producer, Authentication, DeserializeMessage, Error as PulsarError, Payload, Pulsar,
    SerializeMessage, TokioExecutor,
};
// 引入serde序列化包
use serde::{Deserialize, Serialize};

// pulsar基本配置信息
pub struct PulsarConf<'a> {
    addr: &'a str,          // pulsar address
    token: Option<&'a str>, // token optional param
}

// 为PulsarConf实现pulsar客户端的创建
impl<'a> PulsarConf<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self {
            addr: addr,
            token: None,
        }
    }

    pub fn with_token(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }

    // create pulsar client
    pub async fn client(&mut self) -> Result<Pulsar<TokioExecutor>, PulsarError> {
        let mut builder = Pulsar::builder(self.addr.to_string(), TokioExecutor);
        if let Some(token) = self.token {
            let authentication = Authentication {
                name: "token".to_string(),
                data: token.to_string().into_bytes(),
            };

            builder = builder.with_auth(authentication);
        }

        // 更多pulsar初始化参数配置
        // 可以看pulsar crate源码 impl<Exe: Executor> PulsarBuilder<Exe> 的with_开头的方法
        builder.build().await
    }
}

// 定义message消息格式
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub payload: String,
}

// 实现Message序列化
impl SerializeMessage for Message {
    // 实现消息序列化处理，返回producer::Message和PulsarError
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        // 将Message转换为Vec<u8>格式
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default() // 其他字段采用默认设置
        })
    }
}

// 实现Message反序列化
impl DeserializeMessage for Message {
    type Output = Result<Message, serde_json::Error>;
    // 实现message反序列化处理，也就是从pulsar Payload中解析出消息为Message
    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}
