# redis-rs集群操作

假设你想在Redis集群模式或vip ip模式执行上述操作，只需要将main.rs代码中的redis pool初始化改成如下代码即可。

```rust
// redis节点列表（假设redis集群部署在本机上，只是端口不同）
let nodes = vec![
    "redis://:@127.0.0.1:6381/0",
    "redis://:@127.0.0.1:6382/0",
    "redis://:@127.0.0.1:6383/0",
    "redis://:@127.0.0.1:6384/0",
    "redis://:@127.0.0.1:6385/0",
    "redis://:@127.0.0.1:6386/0",
];
// 初始化redis pool
let pool = RedisService::builder()
.with_cluster_nodes(nodes)
.cluster_pool();
如果你需要根据实际情况设置RedisService参数，只需需要调用with_开头的方法就可以，调用方式如下：
let pool = RedisService::builder()
.with_dsn(dsn)
.with_max_size(300)
.pool();
```
