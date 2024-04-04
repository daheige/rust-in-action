# kafka in docker
    You need to change KAFKA_ADVERTISED_LISTENERS to the ip address of your own host,
    then run the following command to start kafka.

    sh bin/docker-run.sh
    ocker Compose is now in the Docker CLI, try `docker compose up`

    Creating dockerconfig_zookeeper_1 ... done
    Creating dockerconfig_kafka_1     ... done
    heige@daheige dockerconfig % docker ps | grep dockerconfig
    29887116ee70   wurstmeister/kafka       "start-kafka.sh"         3 minutes ago    Up 3 minutes    0.0.0.0:9092->9092/tcp                               dockerconfig_kafka_1
    dc84f1aa18eb   wurstmeister/zookeeper   "/bin/sh -c '/usr/sb…"   3 minutes ago    Up 3 minutes    22/tcp, 2888/tcp, 3888/tcp, 0.0.0.0:2181->2181/tcp   dockerconfig_zookeeper_1

# kafka常用命令基本操作
```shell
# 创建topic
./bin/kafka-topics.sh --create --topic test --bootstrap-server localhost:9092
当终端输出“Created topic test”就表示topic创建成功。
# 发送消息
./bin/kafka-console-producer.sh --topic test --bootstrap-server localhost:9092
# 消费消息
./bin/kafka-console-consumer.sh --topic test --from-beginning --bootstrap-server localhost:9092
# 查看和删除topic操作
./bin/kafka-topics.sh --list --bootstrap-server localhost:9092
./bin/kafka-topics.sh --delete --bootstrap-server localhost:9092 --topic test
```
更多有关kafka知识，请参考官方文档：https://kafka.apache.org/documentation/。

