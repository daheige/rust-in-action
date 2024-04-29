// sqlx pool 连接池管理
use sqlx;
use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

// mysql config for mysql
#[derive(Debug, Default)]
pub struct MysqlService<'a> {
    dsn: &'a str,           // dsn连接句柄信息
    max_connections: u32,   // 最大连接数，默认为500
    min_connections: u32,   // 最小连接数，默认为10
    max_lifetime: Duration, // 连接池默认生命周期，默认为1800s
    // 空闲连接生命周期超时，默认为600s
    idle_timeout: Duration,
    // 连接超时时间，默认为10s
    connect_timeout: Duration,
}

impl<'a> MysqlService<'a> {
    pub fn builder(dsn: &'a str) -> Self {
        if dsn.is_empty() {
            panic!("mysql dsn is empty");
        }

        Self {
            dsn,
            max_connections: 500,
            min_connections: 10,
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(600),
            connect_timeout: Duration::from_secs(10),
        }
    }

    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    pub fn with_min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    pub fn with_max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = lifetime;
        self
    }

    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    // 初始化mysql连接池pool
    pub async fn pool(&self) -> Result<sqlx::MySqlPool, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(self.max_connections) // 最大连接数
            .min_connections(self.min_connections) // 最小连接数
            .max_lifetime(self.max_lifetime) // 最大生命周期
            .idle_timeout(self.idle_timeout) // 空闲连接的生命周期
            .acquire_timeout(self.connect_timeout) // 连接超时
            .connect(&self.dsn)
            .await?;
        Ok(pool)
    }
}
