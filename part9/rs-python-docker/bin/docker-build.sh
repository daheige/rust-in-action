#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

image_name=rs-debian
rs_image=`docker images | grep $image_name | awk '{print $1}'`
if [ -z $rs_image ]; then
  cd $root_dir
  docker build . -t $image_name
fi

exit 0
