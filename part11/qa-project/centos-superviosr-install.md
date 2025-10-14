# supervisor工具安装

以下内容是在Linux Centos系统上安装supervisor基本步骤：

1. 通过包管理器安装

```shell
sudo yum update && sudo yum install epel-release
sudo yum install -y supervisor
```

2. 创建配置文件路径和生成默认配置

```shell
sudo mkdir -p /etc/supervisor/conf.d/
sudo echo_supervisord_conf > /etc/supervisord.conf
```

这里需要注意的一点：supervisor默认配置文件在`/etc/supervisord.conf`中，你可以直接编辑该文件或者在`/etc/supervisor.d/`
目录中创建一个以`.ini`结尾的配置文件。

3. 启动supervisord守护进程和设置开机自动启动
   sudo systemctl start supervisord
   sudo systemctl enable supervisord
   以上内容就是Centos系统安装supervisor服务的基本操作。
   如果需要在其他操作系统上面安装supervisor工具，参考官方网站：http://supervisord.org/

除了上面提到的supervisorctl命令管理应用程序进程之外，你还可以通过浏览器访问http://ip:9001管理supervisor服务，
前提是你需要将`supervisord.conf`文件中的`[inet_http_server]`修改为如下内容：

```toml
[inet_http_server]
port = 0.0.0.0:9001 # 设置访问的ip和port
```

以下命令是supervisor常用命令，你可以根据实际情况执行不同的命令。

```shell
# 查看supervisor运行状态
supervisorctl status
# 关闭所有服务
supervisorctl shutdown
# 启动某个进程
supervisorctl start your-program
# 重启某个进程
supervisorctl restart your-program
# 停止某个进程
supervisorctl stop your-program
# 停止全部进程（备注：start、restart、stop操作不会载入新的配置文件）
supervisorctl stop all
# 停止原有所有进程并载入新的配置文件
supervisorctl reload
# 根据最新的配置文件，启动新配置或有改动的进程
supervisor update
# 查看supervisor命令帮助
supervisor help
```
