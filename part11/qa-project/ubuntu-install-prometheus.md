# ubuntu prometheus 部署
https://prometheus.io/docs/prometheus/latest/installation/

`Linux Ubuntu` 系统安装 `prometheus` 服务：
1）更新包索引
```shell
sudo apt-update
```

2）将 prometheus 官方仓库添加到 sources 中
```shell
echo "deb https://packages.prometheus.io/apt/ubuntu $(lsb_release -sc) main" | sudo tee /etc/apt/sources.list.d/prometheus-server.list
```

3）导入 prometheus 的 GPG 密钥和更新包索引
```shell
wget -qO- https://packages.prometheus.io/apt/doc/apt-key.gpg | sudo apt-key add - sudo apt-update
```
4）安装 prometheus 软件包
```shell
sudo apt-get install prometheus
```
当 prometheus 服务在安装完成后会自动启动，你可以通过以下命令查看运行状态：
```shell
sudo systemctl status prometheus
```

prometheus 服务默认配置放在`/etc/prometheus/prometheus.yml`中，Web UI 界面默认将运行在 9090 端口，常用的命令如下：
```shell
# 重启 prometheus 服务
sudo systemctl restart prometheus
# 停止 prometheus 服务
sudo systemctl stop prometheus
# 启动 prometheus 服务
sudo systemctl start prometheus
```

# qa项目使用和接入
在 prometheus 服务的配置文件 prometheus.yml 中添加如下配置：
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

接着，依次执行如下命令启动 qa-svc 和 gateway 服务后，再执行如下 curl 命令请求问题详情接口
```shell
curl --location 'localhost:8090/api/question/detail' --header 'Content-Type: application/json' --data '{"id":1,"username":"daheige"}'
```

在浏览器中访问 `http://localhost:9090`，并在查询框中输入数据指标：
`function_calls_duration_seconds_sum`（当然，你也可以搜索其他 metrics 数据指标），再点击 Execute 按钮后。

到这里，qa-svc 和 gateway 服务的 metrics 数据指标已正常被 prometheus 工具采集和存储。