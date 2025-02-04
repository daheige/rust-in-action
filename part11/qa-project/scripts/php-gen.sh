#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

protoExec=$(which "protoc")
if [ -z $protoExec ]; then
    echo "Please look readme.md to install protoc"
    exit 0
fi

# grpc_php_plugin check.
grpc_php_plugin=$(which "grpc_php_plugin")
if [ -z $grpc_php_plugin ]; then
    echo 'Please install grpc_php_plugin!'
    echo "Please look clients/php/centos-php-grpc.md or mac-php-grpc.md to install grpc_php_plugin tool"
    exit 0
fi

# protos dir
protos_dir=$root_dir/proto

#you can change this dir
php_client_dir=$root_dir/clients/php

mkdir -p $php_client_dir

#delete old pb code
rm -rf $php_client_dir/App

echo "\n\033[0;32mGenerating codes...\033[39;49;0m\n"

echo "generating php stubs..."

#generate php code
cd $protos_dir

for file in $protos_dir/*.proto; do
    echo "generating php stubs from: $file"
    $protoExec --proto_path=$protos_dir --php_out=$root_dir/clients/php/ --grpc_out=$root_dir/clients/php/ --plugin=protoc-gen-grpc=$grpc_php_plugin $file
    echo "\t\033[0;32m[DONE]\033[39;49;0m\n"
done

exit 0;
