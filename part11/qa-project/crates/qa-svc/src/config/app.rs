use crate::config::{mysql, xpulsar, xredis};
use infras::{Config, ConfigTrait};
use once_cell::sync::Lazy;
use pulsar::{Pulsar, TokioExecutor};
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::path::Path;

// 定义项目中使用的mysql pool 和pulsar client
#[derive(Clone)]
pub struct AppState {
    pub mysql_pool: sqlx::MySqlPool,
    pub pulsar_client: Pulsar<TokioExecutor>,
    pub redis_pool: Pool<redis::Client>,
}

#[derive(Clone)]
pub struct ReadCountJobAppState {
    pub mysql_pool: sqlx::MySqlPool,
    pub redis_pool: Pool<redis::Client>,
}

#[derive(Clone)]
pub struct VoteJobAppState {
    pub mysql_pool: sqlx::MySqlPool,
    pub pulsar_client: Pulsar<TokioExecutor>,
}

// AppConfig 项目配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub mysql_conf: mysql::MysqlConf,
    pub pulsar_conf: xpulsar::PulsarConf,
    pub redis_conf: xredis::RedisConf,
    pub app_port: u16,           // grpc 微服务端口
    pub metrics_port: u16,       // prometheus metrics port
    pub graceful_wait_time: u64, // 平滑退出等待时间，单位s
    pub app_debug: bool,
    pub aes_key: String,
    pub aes_iv: String,
}

// config read and init app config
pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let config_dir = std::env::var("QA_CONFIG_DIR").unwrap_or("./".to_string());
    let filename = Path::new(config_dir.as_str()).join("app.yaml");
    println!("filename:{:?}", filename);
    let c = Config::load(filename);

    // read config to struct
    let conf: AppConfig = serde_yaml::from_str(c.content()).unwrap();
    // 开发过程中，可以取消下面的注释
    if conf.app_debug {
        println!("conf:{:?}", conf);
    }

    conf
});
