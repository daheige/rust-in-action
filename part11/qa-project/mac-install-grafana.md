# MacOS下安装 grafana
参考文档：https://grafana.com/docs/grafana/latest/setup-grafana/installation/

具体安装步骤如下：
1）通过brew命令安装
```shell
brew install grafana
```
2）启动prometheus服务
```shell
brew services start grafana
```
默认安装好的 grafana 配置文件放在 /usr/local/etc/grafana/grafana.ini，常用的命令如下：
```shell
# 重启服务
brew services restart grafana
# 停止服务
brew services stop grafana
# 启动服务
brew services start grafana
```
