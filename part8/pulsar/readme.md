# Pulsar docker 安装
```shell
# 启动之前删除原有单机版的pulsar服务
container_name=pulsar-server
container=$(docker ps -a | grep $container_name | awk '{print $1}')
if [ ${#container} -gt 0 ]; then
    docker rm -f $container_name
fi

# pulsar in docker for macOS, Linux, and Windows:
docker run -idt \
  --name pulsar-server \
  -p 6650:6650 \
  -p 8080:8080 \
  --mount source=pulsardata,target=/pulsar/data \
  --mount source=pulsarconf,target=/pulsar/conf \
  apachepulsar/pulsar:3.1.3 \
  bin/pulsar standalone
```

执行 `docker exec -it pulsar-server /bin/bash` 命令进入容器中，查看pulsar目录
```shell
ls
cd bin
```
在 pulsar/bin 目录中为 Pulsar 服务提供了常用命令操作和相关工具

# Pulsar 消息队列基本操作：
- 创建 topic
```shell
bin/pulsar-admin topics create persistent://public/default/my-topic
```
- 查看 topic list
```shell
bin/pulsar-admin topics list public/default
```
- 发送消息
```shell
bin/pulsar-client produce my-topic --messages 'Hello rust!'
```

- 消费消息
```shell
bin/pulsar-client consume my-topic -s 'my-subscription' -p Earliest -n 0
```
- 删除 topic
```shell
bin/pulsar-admin topics delete persistent://public/default/my-topic
```
- 查看 topic 状态
```shell
bin/pulsar-admin topics stats persistent://public/default/my-topic
```

在 Pulsar docker 容器中执行上述命令。当然，这些命令也同样适用于集群版的
pulsar。更多 pulsar 命令操作，你可以直接参考 Pulsar CLI Tools Docs 官方在线文档：
https://pulsar.apache.org/reference/#/next/cli
