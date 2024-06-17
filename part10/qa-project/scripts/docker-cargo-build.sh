#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

image_name=qa-project
version=v1.0
cd $root_dir
docker build . -t $image_name:$version
