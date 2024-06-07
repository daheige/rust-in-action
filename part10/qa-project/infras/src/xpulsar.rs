// 导入pulsar包
use pulsar::{Authentication, Error as PulsarError, Pulsar, TokioExecutor};

// pulsar基本配置信息
pub struct PulsarService<'a> {
    addr: &'a str,          // pulsar address
    token: Option<&'a str>, // token optional param
}

// 为PulsarConf实现pulsar客户端的创建
impl<'a> PulsarService<'a> {
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
