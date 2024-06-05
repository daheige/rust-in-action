#!/bin/bash

# go gRPC tools
go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway@latest
go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2@latest
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

# install go validator optional
# go get github.com/go-playground/validator/v10@latest

#This will place four binaries in your $GOBIN;
#    protoc-gen-grpc-gateway
#    protoc-gen-openapiv2
#    protoc-gen-go
#    protoc-gen-go-grpc

# google api link:https://github.com/googleapis/googleapis

# protoc inject tag
go install github.com/favadi/protoc-go-inject-tag@latest
