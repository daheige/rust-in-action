# rust开发环境配置
DEV_IMAGE_NAME := qa-project-dev
VERSION := v1.0

# qa-project grpc镜像名字配置
RPC_IMAGE_NAME := qa-project-qa-rpc
RPC_SERVICE := qa-svc

# qa-project gateway配置
GATEWAY_IMAGE_NAME := qa-project-gateway
GATEWAY_SERVICE := qa-gateway

# rust qa-project开发环境镜像构建
rust-dev:
	docker build . -f Dockerfile-dev -t ${DEV_IMAGE_NAME}:${VERSION}

# qa-svc 镜像构建
rpc-build:
	docker build . -f Dockerfile -t ${RPC_IMAGE_NAME}:${VERSION}

# qa-svc 容器运行
rpc-start:
	docker run --name=${RPC_SERVICE} -p 50051:50051 -p 2338:2338 \
	-v ./config:/app/config -itd ${RPC_IMAGE_NAME}:${VERSION}

# 删除qa-svc容器
rpc-stop:
	docker stop ${RPC_SERVICE}
	docker rm ${RPC_SERVICE}

# 重启qa-svc
rpc-restart: rpc-stop rpc-start

# gateway 镜像构建
gateway-build:
	docker build . -f Dockerfile-gateway -t ${GATEWAY_IMAGE_NAME}:${VERSION}

# gateway 容器运行
gateway-start:
	docker run --name=${GATEWAY_SERVICE} -p 8090:8090 -p 1338:1338 \
	-v ./config:/app/config -itd ${GATEWAY_IMAGE_NAME}:${VERSION}

# 删除gateway容器
gateway-stop:
	docker stop ${GATEWAY_SERVICE}
	docker rm ${GATEWAY_SERVICE}

# 重启gateway
gateway-restart: gateway-stop gateway-start
