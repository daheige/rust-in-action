# MacOS下安装prometheus

1）通过brew命令安装
brew install prometheus
2）启动prometheus服务
brew services start prometheus
默认安装好的prometheus配置文件放在/etc/local/etc/prometheus.yml，常用的命令如下：

```shell
# 重启prometheus服务
brew services restart prometheus
# 停止prometheus服务
brew services stop prometheus
# 启动prometheus服务
brew services start prometheus
```
