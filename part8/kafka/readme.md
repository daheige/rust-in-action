# kafka安装
1. 新建一个目录 dockerconfig，执行命令如下：
   sudo mkdir ~/dockerconfig

2. 在~/dockerconfig 目录，新建一个 docker-compose.yaml 文件
```yaml
# kafka in docker
services:
  zookeeper:
    image: wurstmeister/zookeeper
    restart: always
    container_name: my-kafka-zk
    ports:
      - 2181:2181
  kafka:
    image: wurstmeister/kafka
    restart: always
    container_name: my-kafka
    depends_on:
      - zookeeper
    ports:
      - 9092:9092
    environment:
      KAFKA_ADVERTISED_HOST_NAME: kafka
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181/kafka
      KAFKA_LISTENERS: PLAINTEXT://:9092
      # 这里需要修改为自己本机IP地址
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://192.168.1.26:9092
      # broker id
      KAFKA_BROKER_ID: 1
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
```

3. 查看kafka运行情况
docker ps | grep kafka

执行 docker exec -it my-kafka /bin/bash 命令进入 Kafka 服务终端窗口中

进入kafka安装目录
cd /opt/kafka

Kafka 安装目录中有 bin、config、libs、logs 等目录。其中在bin 目录下面有各种 kafka 命令操作的 shell 脚本，通过这些脚本可以辅助我们更好地操作和管理 kafka。

# 创建 topic
./bin/kafka-topics.sh --create --topic test --bootstrap-server localhost:9092
当终端输出“Created topic test”就表示 topic 创建成功。
./bin/kafka-topics.sh --create --topic my-topic --bootstrap-server localhost:9092

# 发送消息
```shell
./bin/kafka-console-producer.sh --topic test --bootstrap-server localhost:9092
```

# 消费消息
```shell
docker exec -it my-kafka /bin/bash
cd /opt/kafka
./bin/kafka-console-consumer.sh --topic test --from-beginning --bootstrap-server localhost:9092
```
上述命令需要在另一个终端中运行
