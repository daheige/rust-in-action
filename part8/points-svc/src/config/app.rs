use crate::config::{mysql, xpulsar};
use crate::infras::{Config, ConfigTrait};
use once_cell::sync::Lazy;
use pulsar::{Pulsar, TokioExecutor};
use serde::{Deserialize, Serialize};

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
    pub app_port: u32,
    pub app_debug: bool,
    pub graceful_wait_time: u64,
}

// config read and init app config
pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let mut c = Config::new("app.yaml");
    c.load().expect("read file failed");

    // read config to struct
    let conf: AppConfig = serde_yaml::from_str(c.content()).unwrap();
    // 开发过程中，可以取消下面的注释
    // if conf.app_debug {
    //     println!("conf:{:?}", c.content());
    // }

    conf
});
