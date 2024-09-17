# ubuntu22.04 系统上安装kafka服务

在安装Kafka之前，请先确保你的操作系统上面已安装好Java环境。
安装步骤如下所示：

1. 安装Java运行环境，使用以下命令安装OpenJDK:

```shell
sudo apt update && sudo apt install openjdk-11-jdk
```

如果你的系统已经安装好JDK,可以直接跳过该步骤。

2. 从kafka官方网站https://kafka.apache.org/downloads，选择合适的版本下载并解压到本地目录，执行命令如下：

```shell
cd /usr/local/src/
sudo wget https://archive.apache.org/dist/kafka/2.8.2/kafka_2.12-2.8.2.tgz
sudo tar -zxvf kafka_2.12-2.8.2.tgz
```

3. 修改配置文件server.properties，并添加对应的配置内容，执行命令如下：

```shell
cd kafka_2.12-2.8.2/config
sudo mkdir -p /usr/local/logs/kafka
sudo chmod -R 777 /usr/local/logs/kafka
sudo vim server.properties # 配置如下所示：
broker.id=0
port=9092 #kafka运行的端口号
host.name=192.168.0.5 #根据实际情况，修改为自己的本机IP地址
log.dirs=/usr/local/logs/kafka #日志存放路径，上面创建的目录
#zookeeper地址和端口，单机配置部署设置为localhost:2181
zookeeper.connect=localhost:2181
```

4. 启动zookeeper（kafka依赖于zookeeper，所以需要先启动它），执行命令如下：

```shell
cd /usr/local/src/kafka_2.12-2.8.2
bin/zookeeper-server-start.sh config/zookeeper.properties
```

5. 新开一个命令终端窗口，启动kakfa，执行命令如下：

```shell
cd /usr/local/src/kafka_2.12-2.8.2
bin/kafka-server-start.sh config/server.properties
```

当你看到终端中提示：INFO Verifying properties，就说明Kafka安装完成。
