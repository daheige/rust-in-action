# protoc工具安装
protoc工具的安装因操作系统不同，其安装方式不一样，你可以根据实际情况选择其中的一种方式即可。
下面的内容是如何在Linux Centos、Ubuntu以及MacOS系统上面安装protoc工具的步骤。
## Centos安装protoc工具：
1）下载protobuf包
```shell
cd /usr/local/src/
sudo wget https://github.com/protocolbuffers/protobuf/archive/v3.15.8.tar.gz
```
2）安装相关依赖
```shell
sudo yum install gcc-c++ cmake libtool
```
3）解压protobuf包，并使用make编译安装
```shell
sudo mv v3.15.8.tar.gz protobuf-3.15.8.tar.gz
sudo tar zxvf protobuf-3.15.8.tar.gz
sudo mkdir /usr/local/protobuf
cd protobuf-3.15.8
./autogen.sh
sudo ./configure --prefix=/usr/local/protobuf
sudo make && sudo make install
```
4）查看安装protoc版本
```shell
cd /usr/local/protobuf/bin
./protoc --version
```
执行上面命令后，会输出：libprotoc 3.15.8
5）建立protoc软链接
```shell
sudo ln -s /usr/local/protobuf/bin/protoc /usr/bin/protoc
sudo chmod +x /usr/bin/protoc
```

## Ubuntu系统安装protoc工具
1）安装相关依赖
```shell
sudo apt-get install gcc cmake make libtool
```
2）安装protobuf工具包
```shell
sudo apt-get install libprotobuf-dev protocbuf-compiler
```
3）查看安装版本
```shell
protoc --version
```

## MacOS系统安装protoc工具
1）安装相关依赖
```shell
brew install automake
brew install libtool
```
2）安装protobuf
```shell
brew install protobuf
```
3）查看protoc版本
```shell
protoc --version
```
