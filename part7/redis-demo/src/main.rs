mod xredis; // 定义xredis模块
use redis::{AsyncCommands, Commands, RedisResult};
use std::time::Duration;
// 引入xredis模块中的 RedisService 结构体
use xredis::RedisService;

#[tokio::main]
// 对于错误返回使用的是anyhow的Result类型
// anyhow提供了一个Error类型，这个类型可以包含任何实现了std::error::Error的错误，
// 这意味着你可以使用anyhow::Error来包装几乎所有类型的错误，无需担心具体的错误类型。
// 这里的anyhow::Result其实是一个类型别名Result<T, E = Error>，所以可以直接使用
async fn main() -> anyhow::Result<()> {
    // 单机版redis或者vip ip形式的redis，创建redis连接连接池
    // dsn格式：redis://:[password]@[host]:[port]/[database]
    // 比如说：redis://:@127.0.0.1:6379/0
    let dsn = "redis://:@127.0.0.1:6379/0";
    let pool = RedisService::builder()
        .with_dsn(dsn)
        .with_max_size(300)
        .pool();

    // redis节点列表（假设redis集群部署在本机上，只是端口不同）
    // let nodes = vec![
    //     "redis://:@127.0.0.1:6381/0",
    //     "redis://:@127.0.0.1:6382/0",
    //     "redis://:@127.0.0.1:6383/0",
    //     "redis://:@127.0.0.1:6384/0",
    //     "redis://:@127.0.0.1:6385/0",
    //     "redis://:@127.0.0.1:6386/0",
    // ];
    // // 初始化redis pool
    // let pool = RedisService::builder()
    //     .with_cluster_nodes(nodes)
    //     .cluster_pool();

    let mut conn = pool.get().unwrap(); // 从连接池中获取一个连接

    // redis set操作
    // 大部分redis命令返回值是一个 redis::RedisResult<()>类型
    let res: RedisResult<String> = conn.set("username", "daheige");
    // 如果需要判断错误，可以使用is_err方法判断
    if res.is_err() {
        println!("redis set error:{}", res.err().unwrap().to_string());
    } else {
        println!("set success");
    }

    // redis get操作
    let name: String = conn.get("username")?; // 通过?简写的方式返回错误
    println!("username:{}", name);

    // 为单个conn session会话设置timeout为2s
    let mut conn2 = pool.get_timeout(Duration::from_secs(2)).unwrap();
    conn2.set("user2", "xiaoming")?;

    // redis mget操作
    // let res: RedisResult<Vec<String>> = conn.mget(&["username", "user2"]);
    // println!("res:{:?}", res.unwrap());
    let records: Vec<String> = conn.mget(&["username", "user2"])?; // 直接将结果解构到vec中
    println!("records:{:?}", records);

    // redis incr操作，假设要为一篇文章id=2，增加阅读数
    let num: i64 = conn.incr("post_id:1:read_count", 1)?;
    println!("post num:{}", num);

    // 通过redis cmd形式操作redis
    redis::cmd("set").arg(&["user3", "abc"]).query(&mut conn)?;
    let name: String = conn.get("user3")?;
    println!("user3:{}", name);

    // 执行redis set ex操作
    let res: RedisResult<()> = conn.set_ex("user", "zhangsan", 120);
    if res.is_err() {
        println!("set ex error:{}", res.err().unwrap())
    } else {
        println!("set ex success")
    }
    let name: String = conn.get("user")?;
    println!("user:{}", name);

    Ok(())
}
