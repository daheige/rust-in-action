FROM qa-project-dev:v1.0 as builder

LABEL authors="heige"

# 设置环境变量
ENV LANG C.UTF-8

WORKDIR /app

COPY . .

RUN cargo build --release

# 将上面构建好的二进制文件复制到容器中运行
FROM debian:bullseye-slim

WORKDIR /app

ENV QA_CONFIG_DIR=/app/config

VOLUME ${QA_CONFIG_DIR}

# grpc服务端口和metrics端口
EXPOSE 50051
EXPOSE 2338

COPY --from=builder /app/target/release/qa-svc /app/qa-svc

RUN apt-get update && apt-get install -y ca-certificates vim bash curl net-tools \
    apt-transport-https && update-ca-certificates && apt-get clean &&  \
    rm -rf /var/lib/apt/lists/* && mkdir /app/config

CMD ["/app/qa-svc"]
