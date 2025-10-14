# pulsar message自定义

```rust
上述示例中的pulsar client和Message消息结构体定义放在src/xpulsar.rs模块中，其代码如下所示。
// 引入pulsar相关的包
use pulsar::{
    producer, Authentication, DeserializeMessage, Error as PulsarError, Payload, Pulsar,
    SerializeMessage, TokioExecutor,
};
// 引入serde序列化包
use serde::{Deserialize, Serialize};

// pulsar基本配置信息
pub struct PulsarConf<'a> {
    addr: &'a str,
    // pulsar address
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

        // 更多pulsar初始化参数配置，
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
```

- 在这个xpulsar模块中，首先引入了pulsar::producer、pulsar::Authentication、pulsar::Error等模块。然后，引入了serde::
  {Deserialize, Serialize}用于消息Message序列化处理。
- 接着，定义了PulsarConf结构体，在结构体中有addr和token两个字段，类型分别是&str和Option
  &str。这里需要注意的一点：PulsarConf结构体包含了引用类型的变量，需要显式标注生命周期'a，保证引用的有效性在程序运行过程中一直有效。
- 随后，在这个PulsarConf结构体上通过impl实现了new方法用于创建实例对象，client方法主要用于pulsar客户端对象的创建。
- with_token方法用于设置pulsar连接的token（如果需要设置的token的话，链式调用with_token即可）。
