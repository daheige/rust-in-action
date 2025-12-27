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
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_LISTENERS: PLAINTEXT://0.0.0.0:9092
      # KAFKA_CREATE_TOPICS: "my-topic"
      # 这里需要修改为自己本机IP地址
      #KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://192.168.1.168:9092
      #或者使用下面的方式也可以
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
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

# rs-broker
- 为了更好、更快使用kafka发送消息和消费消息，我将其封装为`rs-broker`包，已发布到crates.io上面。
- rs-broker: https://github.com/rs-god/rs-broker 具体使用方式见：https://github.com/daheige/rs-broker-demo

# rust rdkafka

下面是使用rust rdkafka组件库实现消息发送和消费

- rust rdkafka官网: https://crates.io/crates/rdkafka
- rdkafka crate: https://github.com/fede1024/rust-rdkafka 更多用法看官方examples
- librdkafka官网：https://github.com/confluentinc/librdkafka the Apache Kafka C/C++ library
- rust kafka实现消息发送和消费，这里使用的是rdkafka，相比rskafka和kafka crate，稳定性和兼容性更好。
- rdkafka这个库是基于c语言编写的，性能高。
- 如果不需要kafka更多配置，可以直接使用kafka = "0.10.0" 这个crate: https://crates.io/crates/kafka
- 基于rdkafka封装的broker见: https://github.com/rs-god/rs-broker
