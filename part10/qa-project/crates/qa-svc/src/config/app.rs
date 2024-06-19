use crate::config::{mysql, xpulsar};
use infras::{Config, ConfigTrait};
use once_cell::sync::Lazy;
use pulsar::{Pulsar, TokioExecutor};
use serde::{Deserialize, Serialize};
use std::path::Path;

// 定义传递给axum handlers的app_state，这里是通过引用计数的方式共享变量
// Sharing state with handlers
#[derive(Clone)]
pub struct AppState {
    pub mysql_pool: sqlx::MySqlPool,
    pub pulsar_client: Pulsar<TokioExecutor>,
}

// AppConfig 项目配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub mysql_conf: mysql::MysqlConf,
    pub pulsar_conf: xpulsar::PulsarConf,
    pub app_port: u16,           // grpc 微服务端口
    pub metrics_port: u16,       // prometheus metrics port
    pub graceful_wait_time: u64, // 平滑退出等待时间，单位s
    pub app_debug: bool,
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
