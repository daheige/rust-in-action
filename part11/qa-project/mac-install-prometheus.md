# MacOS下安装prometheus
参考文档：https://prometheus.io/docs/prometheus/latest/installation/

具体安装步骤如下：
1）通过brew命令安装
```shell
brew install prometheus
```
2）启动prometheus服务
```shell
brew services start prometheus
```
默认安装好的prometheus配置文件放在 `/usr/local/etc/prometheus.yml`，完整的配置如下：
```yaml
# 完整的prometheus配置
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: "prometheus"
    static_configs:
      - targets: ["localhost:9090"]
  # qa gateway 服务的 metrics 数据采集
  - job_name: "qa_gateway"
    static_configs:
      - targets: ["localhost:1338"]
  # qa-svc 服务的 metrics 数据采集
  - job_name: "qa_svc"
    static_configs:
      - targets: ["localhost:2338"]
```

# macos prometheus 常用的命令如下：
```shell
# 重启prometheus服务
brew services restart prometheus
# 停止prometheus服务
brew services stop prometheus
# 启动prometheus服务
brew services start prometheus
```
