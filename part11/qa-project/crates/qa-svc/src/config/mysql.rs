// 导入infras模块的MysqlService结构体
use infras::MysqlService;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// mysql配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MysqlConf {
    pub dsn: String,          // dsn连接句柄信息
    pub max_connections: u32, // 最大连接数
    pub min_connections: u32, // 最小连接数
    pub max_lifetime: u64,    // 连接池默认生命周期，单位s
    // 空闲连接生命周期超时，单位s
    pub idle_timeout: u64,
    // 连接超时时间，单位s
    pub connect_timeout: u64,
}

// 创建mysql pool 连接池
pub async fn pool(conf: &MysqlConf) -> Result<sqlx::MySqlPool, sqlx::Error> {
    let pool = MysqlService::builder(conf.dsn.as_str())
        .with_max_connections(conf.max_connections) // 最大连接数
        .with_min_connections(conf.min_connections) // 最小连接数
        .with_max_lifetime(Duration::from_secs(conf.max_lifetime)) // 最大生命周期
        .with_idle_timeout(Duration::from_secs(conf.idle_timeout)) // 空闲连接的生命周期
        .with_connect_timeout(Duration::from_secs(conf.connect_timeout)) // 连接超时
        .pool()
        .await;
    pool
}
