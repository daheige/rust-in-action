// 导入infras模块的PulsarService结构体
use infras::PulsarService;
use serde::{Deserialize, Serialize};
// 导入pulsar包
use pulsar::{Error as PulsarError, Pulsar, TokioExecutor};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PulsarConf {
    pub addr: String,  // pulsar连接句柄信息
    pub token: String, // auth token信息
}

// 创建pulsar client
pub async fn client(conf: &PulsarConf) -> Result<Pulsar<TokioExecutor>, PulsarError> {
    let mut p = PulsarService::new(&conf.addr);
    if !conf.token.is_empty() {
        p = p.with_token(&conf.token);
    }

    let client = p.client().await;
    client
}
