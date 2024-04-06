#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

docker rm -f `docker ps -a | grep kafka | awk '{print $1}'`

cd $root_dir/kafka-cluster
docker-compose up -d
