// 定义redis和mysql服务连接池管理的模块
mod config;
mod xmysql;
mod xpulsar;

// 使用use对模块重新导出
pub use config::{Config, ConfigTrait};
pub use xmysql::MysqlService;
pub use xpulsar::PulsarService;

#[test]
fn it_works(){
    println!("ok");
}