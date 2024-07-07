// 定义redis和mysql服务连接池管理的模块
mod aes;
mod config;
mod logger;
mod metrics;
mod shutdown;
mod xmysql;
mod xpulsar;
mod xredis;
pub mod sql_utils;

// 使用use对模块重新导出
pub use aes::{AesCBCCrypto, AesKeySize};
pub use config::{Config, ConfigTrait};
pub use logger::Logger;
pub use metrics::prometheus_init;
pub use shutdown::{graceful_shutdown, job_graceful_shutdown};
pub use xmysql::MysqlService;
pub use xpulsar::PulsarService;
pub use xredis::RedisService;

#[test]
fn it_works() {
    println!("ok");
    use aes::{AesCBCCrypto, AesKeySize};
    let key = "YiBX0z9WnJjsS5aNXmi0AeT1yTPZZJYa";
    let iv = "3ZQEpwP9DbK4h1Z0";

    let aes_crypto = AesCBCCrypto::new(key, iv, AesKeySize::Size256);
    let data = "Hello, world!";
    let encrypted_data = aes_crypto.encrypt(data).unwrap();
    println!("encrypted_data:{}", encrypted_data);

    let decrypted_data = aes_crypto.decrypt(&encrypted_data).unwrap();
    println!("decrypted_data:{}", decrypted_data);

    assert_eq!(data, decrypted_data.as_str());
}
