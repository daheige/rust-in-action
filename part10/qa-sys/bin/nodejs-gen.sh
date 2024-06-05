#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)

protoExec=$(which "protoc")
if [ -z $protoExec ]; then
    echo 'Please install protoc3'
    exit 0
fi

jsProtoExec=$(which "grpc_tools_node_protoc")
if [ -z $jsProtoExec ]; then
    grpc_node_plugin=$(which "grpc_node_plugin")
    if [ -z $grpc_node_plugin ]; then
        echo 'Please install grpc_node_plugin or grpc_tools_node_protoc'
        echo "npm install -g grpc-tools"
        exit 0
    fi
fi

echo "\n\033[0;32mGenerating codes...\033[39;49;0m\n"
echo "generating nodejs stubs..."

nodejs_pb_dir=$root_dir/clients/nodejs/pb
mkdir -p $nodejs_pb_dir

cd $root_dir/proto

if [ ! -z $jsProtoExec ]; then
    # protoc generate js code with grpc_tools_node_protoc
    $jsProtoExec --js_out=import_style=commonjs,binary:$nodejs_pb_dir \
      --grpc_out=$nodejs_pb_dir --plugin=protoc-gen-grpc=`which grpc_tools_node_protoc_plugin` *.proto
else
  grpc_node_plugin=$(which "grpc_node_plugin")
  if [ -z $grpc_node_plugin ]; then
      echo 'Please install grpc_node_plugin'
      exit 0
  fi

  # protoc generate js code without grpc_tools_node_protoc,but use grpc_node_plugin
  $protoExec --js_out=import_style=commonjs,binary:$nodejs_pb_dir --plugin=protoc-gen-grpc=$grpc_node_plugin --grpc_out=$nodejs_pb_dir *.proto
fi

# replace
os=`uname -s`
if [ $os == "Darwin" ];then
     # mac os LC_CTYPE config
    export LC_CTYPE=C

    # mac os
    sed -i "" 's/var google_api_annotations_pb/\/\/ var google_api_annotations_pb/g' `grep google_api_annotations_pb -rl $nodejs_pb_dir`
    sed -i "" 's/let google_api_annotations_pb/\/\/ let google_api_annotations_pb/g' `grep google_api_annotations_pb -rl $nodejs_pb_dir`
    sed -i "" 's/goog.object.extend(proto, google_api_annotations_pb)/\/\/ this code deleted/g' `grep google_api_annotations_pb -rl $nodejs_pb_dir`
    # replace grpc to @grpc/grpc-js
    sed -i "" "s/require('grpc')/require('@grpc\/grpc-js')/g" `grep "require('grpc')" -rl $nodejs_pb_dir`
else
    sed -i 's/var google_api_annotations_pb/\/\/ var google_api_annotations_pb/g' `grep google_api_annotations_pb -rl $nodejs_pb_dir`
    sed -i 's/let google_api_annotations_pb/\/\/ let google_api_annotations_pb/g' `grep google_api_annotations_pb -rl $nodejs_pb_dir`
    sed -i 's/goog.object.extend(proto, google_api_annotations_pb)/\/\/ this code deleted/g' `grep google_api_annotations_pb -rl $nodejs_pb_dir`
    sed -i "s/require('grpc')/require('@grpc\/grpc-js')/g" `grep "require('grpc')" -rl $nodejs_pb_dir`
fi

echo "generating nodejs code success"

echo "\n\033[0;32mGenerate codes successfully!\033[39;49;0m\n"

exit 0
