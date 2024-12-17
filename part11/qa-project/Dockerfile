FROM qa-project-dev:v1.0 as builder

LABEL authors="heige"

# 设置环境变量LANG
ENV LANG C.UTF-8

WORKDIR /app

COPY . .

# 编译构建rust应用程序
RUN cd /app && cargo build --release

# 将上面构建好的二进制文件复制到容器中运行
FROM debian:bullseye-slim

WORKDIR /app

# 配置文件目录
ENV QA_CONFIG_DIR=/app/config
VOLUME ${QA_CONFIG_DIR}

# 设置gRPC微服务和metrics服务运行端口
EXPOSE 50051
EXPOSE 2338

# 设置deb镜像源，这里我使用aliyun的镜像
RUN echo "deb http://mirrors.aliyun.com/debian bullseye main" > /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian-security bullseye-security main" >> /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian bullseye-updates main" >> /etc/apt/sources.list &&  \
    apt-get update && apt-get install -y ca-certificates vim bash curl net-tools \
    apt-transport-https && update-ca-certificates && apt-get clean &&  \
    rm -rf /var/lib/apt/lists/* && mkdir -p /app/config

# 将构建阶段的二进制文件复制到工作目录中
COPY --from=builder /app/target/release/qa-svc /app/qa-svc

CMD ["/app/qa-svc"]
