use r2d2::Pool;
use redis::{self, cluster::ClusterClient};
use std::time::Duration;

// redis connection 配置信息
// 'a是生命周期标记，RedisService中 dsn是一个String引用类型
#[derive(Default, Debug)]
pub struct RedisService<'a> {
    // dsn格式：redis://:[password]@[host]:[port]/[database]
    // 比如说：redis://:@127.0.0.1:6379/0
    dsn: &'a str,                        // redis dsn信息，用于连接redis
    max_size: u32,                       // 最大连接个数
    min_idle: u32,                       // 最小空闲数
    max_lifetime: Duration,              // 过期时间
    idle_timeout: Duration,              // 连接池最大生存期
    connection_timeout: Duration,        // 连接超时时间
    cluster_nodes: Option<Vec<&'a str>>, // 可选参数，集群模式节点列表
}

impl<'a> RedisService<'a> {
    // builder模式创建RedisService
    pub fn builder() -> Self {
        Self {
            max_size: 200,
            min_idle: 3,
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(300),
            connection_timeout: Duration::from_secs(10),
            ..Default::default()
        }
    }

    pub fn with_dsn(mut self, dsn: &'a str) -> Self {
        self.dsn = dsn;
        self
    }

    pub fn with_cluster_nodes(mut self, nodes: Vec<&'a str>) -> Self {
        self.cluster_nodes = Some(nodes);
        self
    }

    pub fn with_max_size(mut self, max: u32) -> Self {
        self.max_size = max;
        self
    }

    pub fn with_min_idle(mut self, min: u32) -> Self {
        self.min_idle = min;
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
        self.connection_timeout = timeout;
        self
    }

    // create redis client
    pub fn client(&self) -> redis::RedisResult<redis::Client> {
        if self.dsn.is_empty() {
            return Err(redis::RedisError::from((
                redis::ErrorKind::InvalidClientConfig,
                "redis dsn is empty",
            )));
        }

        let client = redis::Client::open(self.dsn);
        client
    }

    // create redis cluster client
    // 这种适合集群模式的redis节点连接redis
    pub fn cluster_client(&self) -> redis::RedisResult<ClusterClient> {
        if self.cluster_nodes.is_none() {
            return Err(redis::RedisError::from((
                redis::ErrorKind::InvalidClientConfig,
                "redis cluster nodes is empty",
            )));
        }

        // 这里需要对cluster_nodes克隆得到一个新的Option<Vec<String>>
        let nodes = self.cluster_nodes.clone().unwrap();
        let client = ClusterClient::new(nodes);
        client
    }

    // create redis client pool
    pub fn pool(&self) -> Pool<redis::Client> {
        let client = self.client().expect("create redis client failed");
        let pool = self.init_pool(client);
        pool
    }

    // redis cluster pool
    pub fn cluster_pool(&self) -> Pool<ClusterClient> {
        let client = self
            .cluster_client()
            .expect("create redis cluster client failed");
        let pool = self.init_pool(client);
        pool
    }

    // 由于redis client和redis cluster client创建pool只是build的client不一样
    // 所以这里可以采用泛型方法创建pool,这个泛型参数T满足ManageConnection trait特征就可以
    fn init_pool<T: r2d2::ManageConnection>(&self, client: T) -> Pool<T> {
        let pool = Pool::builder()
            .max_size(self.max_size)
            .max_lifetime(Some(self.max_lifetime))
            .idle_timeout(Some(self.idle_timeout))
            .min_idle(Some(self.min_idle))
            .connection_timeout(self.connection_timeout)
            .build(client)
            .expect("init redis pool failed");
        // 返回redis pool
        pool
    }
}
