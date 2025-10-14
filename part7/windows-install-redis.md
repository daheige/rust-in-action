# Windows安装redis服务

- Redis服务在Windows下安装不被Redis官方所支持，但你可以通过WSL2（Linux的Windows子系统）在Windows系统（需要Windows10或Windows
  11及其更高的版本）上面安装Linux系统。
- Microsoft提供了安装WSL2的详细说明，具体可参考Windows官方的说明操作，并注意它安装的默认Linux发行版。

假设你已经使用WSL2方式安装了Linux
Ubuntu系统，那么Redis服务安装步骤如下：
1）设置Redis gpg。

```shell
curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg
```

2）设置packages.redis.io/deb。

```shell
echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs)
main" | sudo tee /etc/apt/sources.list.d/redis.list
```

3）更新Ubuntu package并安装Redis。

```shell
sudo apt-get update
sudo apt-get install redis
# 当我们安装好redis后，就可以通过如下命令启动redis
sudo service redis-server start
```

此时你可以通过redis-cli交互窗口，执行ping命令来测试Redis服务器是否正常运行

```shell
redis-cli
ping
```
