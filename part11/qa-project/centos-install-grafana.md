# centos安装grafana

1）添加grafana的yum源

```shell
sudo cat <<EOF > /etc/yum.repos.d/grafana.repo
[grafana]
name=grafana
baseurl=https://packages.grafana.com/oss/rpm
repo_gpgcheck=1
enabled=1
gpgcheck=1
gpgkey=https://packages.grafana.com/gpg.key
sslverify=1
sslcacert=/etc/pki/tls/certs/ca-bundle.crt
EOF
```

2）安装grafana软件

```shell
sudo yum install grafana -y
```

3）启动grafana服务并设置开机启动

```shell
sudo systemctl start grafana-server
sudo systemctl enable grafana-server
```
