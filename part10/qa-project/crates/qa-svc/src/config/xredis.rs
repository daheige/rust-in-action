use infras::RedisService;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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

pub fn pool(redis_conf: &RedisConf) -> Result<Pool<redis::Client>, String> {
    if redis_conf.dsn.is_empty() {
        return Err("dsn is empty".to_string());
    }

    let pool = RedisService::builder()
        .with_dsn(redis_conf.dsn.as_str())
        .with_max_size(redis_conf.max_size)
        .with_max_lifetime(Duration::from_secs(redis_conf.max_lifetime))
        .with_idle_timeout(Duration::from_secs(redis_conf.idle_timeout))
        .with_min_idle(redis_conf.min_idle)
        .with_connect_timeout(Duration::from_secs(redis_conf.connection_timeout))
        .pool();

    Ok(pool)
}

pub fn cluster_pool(redis_conf: &RedisConf) -> Result<Pool<redis::cluster::ClusterClient>, String> {
    if redis_conf.cluster_nodes.is_none() {
        return Err("cluster nodes is empty".to_string());
    }

    let nodes = redis_conf.cluster_nodes.clone().unwrap();
    let cluster_nodes = nodes.iter().map(|s| s.as_str()).collect();
    let pool = RedisService::builder()
        .with_cluster_nodes(cluster_nodes)
        .with_max_size(redis_conf.max_size)
        .with_max_lifetime(Duration::from_secs(redis_conf.max_lifetime))
        .with_idle_timeout(Duration::from_secs(redis_conf.idle_timeout))
        .with_min_idle(redis_conf.min_idle)
        .with_connect_timeout(Duration::from_secs(redis_conf.connection_timeout))
        .cluster_pool();
    Ok(pool)
}
