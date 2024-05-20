#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

sh $root_dir/bin/docker-build.sh
image_name=rs-debian
container_name=rs-local-dev

echo "docker run $container_name begin..."
docker rm -f `docker ps -a | grep $container_name | awk '{print $1}'`
docker run -itd --name $container_name $image_name
echo "docker run $container_name success!"
echo "you can run cmd: 'docker exec -it $container_name /bin/bash' enter $container_name"

exit 0
