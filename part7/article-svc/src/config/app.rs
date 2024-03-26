use crate::config::{mysql, xredis};
use crate::infras::{Config, ConfigTrait, MysqlService, RedisService};
use once_cell::sync::Lazy;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// 定义传递给axum handlers的app_state，这里是通过引用计数的方式共享变量
// Sharing state with handlers
#[derive(Clone)]
pub struct AppState {
    pub mysql_pool: sqlx::MySqlPool,
    pub redis_pool: Pool<redis::Client>,
}

// AppConfig 项目配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub redis_conf: xredis::RedisConf,
    pub mysql_conf: mysql::MysqlConf,
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

// redis pool 初始化
pub static REDIS_POOL: Lazy<Pool<redis::Client>> = Lazy::new(|| {
    let redis_conf = &APP_CONFIG.redis_conf;
    let pool = RedisService::builder()
        .with_dsn(redis_conf.dsn.as_str())
        .with_max_size(redis_conf.max_size)
        .with_max_lifetime(Duration::from_secs(redis_conf.max_lifetime))
        .with_idle_timeout(Duration::from_secs(redis_conf.idle_timeout))
        .with_min_idle(redis_conf.min_idle)
        .with_connect_timeout(Duration::from_secs(redis_conf.connection_timeout))
        .pool();
    pool
});
