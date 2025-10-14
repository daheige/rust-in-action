# Linux Ubuntu22.04安装pulsar服务

由于Pulsar消息队列是使用Java语言开发的，在安装之前请确保你的系统上已经具备Java 8或更高版本的环境。

1) 首先安装java运行环境，使用以下命令安装OpenJDK:
   sudo apt update && sudo apt install openjdk-11-jdk
   如果你的系统已经安装好JDK,可以直接跳过该步骤。
2) 下载pulsar包并解压到本地目录
   cd /usr/local/opt/
   sudo wget https://archive.apache.org/dist/pulsar/pulsar-3.1.3/apache-pulsar-3.1.3-bin.tar.gz
   sudo mkdir -p /usr/local/pulsar
   sudo tar -xvfz apache-pulsar-3.1.3-bin.tar.gz -C /usr/local/pulsar
3) 进入pulsar目录，启动pulsar服务（我这里使用的是单节点模式启动pulsar）
   cd /usr/local/pulsar
   ./bin/pulsar standalone
   当终端输出“Successfully started storage”提示，就表明pulsar服务安装成功。
   如果想让pulsar服务在后台运行，执行./bin/pulsar-daemon start standalone即可。
