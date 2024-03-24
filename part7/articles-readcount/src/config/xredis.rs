use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisConf {
    // dsn格式：redis://:[password]@[host]:[port]/[database]
    // 比如说：redis://:@127.0.0.1:6379/0
    pub dsn: String,                        // redis dsn信息，用于连接redis
    pub max_size: u32,                      // 最大连接个数
    pub min_idle: u32,                      // 最小空闲数
    pub max_lifetime: u64,                  // 过期时间
    pub idle_timeout: u64,                  // 连接池最大生存期
    pub connection_timeout: u64,            // 连接超时时间
    pub cluster_nodes: Option<Vec<String>>, // 可选参数，集群模式节点列表
}
