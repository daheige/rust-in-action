// 定义redis和mysql服务连接池管理的模块
mod config;
mod logger;
mod metrics;
mod shutdown;
mod xmysql;
mod xpulsar;
mod xredis;

// 使用use对模块重新导出
pub use config::{Config, ConfigTrait};
pub use logger::Logger;
pub use metrics::prometheus_init;
pub use shutdown::{graceful_shutdown,job_graceful_shutdown};
pub use xmysql::MysqlService;
pub use xpulsar::PulsarService;
pub use xredis::RedisService;

#[test]
fn it_works() {
    println!("ok");
}
