# MacOS安装MySQL：

在MacOS下安装MySQL服务相对来说，简单一些，只需要执行以下几步即可。

1. 使用brew命令下载并安装mysql8.0。

```shell
brew install cmake
brew install mysql@8.0
```

2. 设置密码并刷新权限。

```shell
brew services start mysql
mysql -uroot -p
# 回车进入终端，接着执行如下命令：
alter user 'root'@'localhost' identified with mysql_native_password by 'root123456';
flush priviledges;
exit;
```

3. 重启MySQL服务，并使用密码重新登录MySQL。

```shell
brew services restart mysql
mysql -uroot -p
#回车后，输入密码即可。
```

当我们在MacOS上面安装好MySQL后，可以通过如下命令启动、停止、重启MySQL服务。

# mysql服务相关命令

```shell
brew services start mysql # 启动
brew services stop mysql # 停止
brew services restart mysql # 重启
```
