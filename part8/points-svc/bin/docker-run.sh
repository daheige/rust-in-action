#!/usr/bin/env bash

# 启动之前，先删除原有单机版的pulsar服务
docker rm -f `docker ps -a | grep pulsar-server | awk '{print $1}'`

# pulsar in docker for macOS, Linux, and Windows:
docker run -idt \
--name pulsar-server \
-p 6650:6650 \
-p 8080:8080 \
--mount source=pulsardata,target=/pulsar/data \
--mount source=pulsarconf,target=/pulsar/conf \
apachepulsar/pulsar:3.1.3 \
bin/pulsar standalone
