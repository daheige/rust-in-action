// 定义redis和mysql服务连接池管理的模块
mod config;
mod xmysql;
mod xredis;

// 使用use对模块重新导出
pub use config::{Config, ConfigTrait};
pub use xmysql::MysqlService;
pub use xredis::RedisService;
