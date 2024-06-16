#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

image_name=qa-project-dev
cd $root_dir
docker build . -f Dockerfile-dev -t $image_name
