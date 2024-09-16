#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

protoExec=$(which "protoc")
if [ -z $protoExec ]; then
    echo 'Please install protoc!'
    echo "Please look readme.md to install protoc"
    exit 0
fi

protos_dir=$root_dir/proto
pb_dir=$root_dir/clients/go/pb

mkdir -p $pb_dir

#delete old pb code.
rm -rf $pb_dir/*.go

echo "\n\033[0;32mGenerating codes...\033[39;49;0m\n"

echo "generating golang stubs..."
cd $protos_dir

# go grpc code
protoc -I $protos_dir \
    --go_out $pb_dir --go_opt paths=source_relative \
    --go-grpc_out $pb_dir --go-grpc_opt paths=source_relative \
    $protos_dir/*.proto

# http gw code
protoc -I $protos_dir --grpc-gateway_out $pb_dir \
    --grpc-gateway_opt logtostderr=true \
    --grpc-gateway_opt paths=source_relative \
    $protos_dir/*.proto

echo "generating golang code success"
echo "\n\033[0;32mGenerate codes successfully!\033[39;49;0m\n"
exit 0
