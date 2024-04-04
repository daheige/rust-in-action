#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

docker rm -f $(docker ps | grep kafka | awk '{print $1}')

rm -rf $root_dir/{kafka,zookeeper}

mkdir $root_dir/{kafka,zookeeper}
chmod -R 777 $root_dir/kafka
chmod -R 777 $root_dir/zookeeper

cd $root_dir
docker-compose up -d
