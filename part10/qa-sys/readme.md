# qa-sys
    综合型项目：QA问答系统
# grpc client support
- rust grpc微服务使用的crates: tokio,tonic,tonic-build,prost代码生成
- grpc客户端支持go,nodejs,rust等不同语言调用服务端程序
- 支持http gateway模式（http json请求到网关层后，转换为pb message，然后发起grpc service调用）

# linux install protoc
1、下载protoc
```shell
cd /usr/local/src
sudo wget https://github.com/protocolbuffers/protobuf/archive/v3.15.8.tar.gz
```
2、安装protoc
```shell
# 解压文件
sudo mv v3.15.8.tar.gz protobuf-3.15.8.tar.gz
sudo tar zxvf protobuf-3.15.8.tar.gz
cd protobuf-3.15.8
# centos系统
sudo yum install gcc-c++ cmake libtool

# 对于ubuntu系统需要安装的依赖：
# sudo apt install gcc cmake make libtool

sudo mkdir /usr/local/protobuf
```

这里需要注意的一点: 在新版的 PB 源码中，是不包含 .configure 文件的，需要生成。此时先执行 sudo ./autogen.sh 脚本，说明如下:
```
# Run this script to generate the configure script and other files that will
# be included in the distribution. These files are not checked in because they
# are automatically generated.
```

此时生成了 .configure 文件，可以开始编译了
```shell
sudo ./configure --prefix=/usr/local/protobuf
sudo make && make install
```

安装完成后,查看版本:
```shell
cd /usr/local/protobuf/bin
./protoc --version
```
输出结果：libprotoc 3.15.8

建立软链接
```shell
sudo ln -s /usr/local/protobuf/bin/protoc /usr/bin/protoc
sudo chmod +x /usr/bin/protoc
```

# mac install protoc
```shell
brew install automake
brew install libtool
brew install protobuf
```
