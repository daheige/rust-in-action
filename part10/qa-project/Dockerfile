FROM qa-project-dev:v1.0 as builder

LABEL authors="heige"

# 设置rust镜像源
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 设置环境变量
ENV LANG C.UTF-8

WORKDIR /app

COPY . .

RUN cd /app && cargo build --release

# 将上面构建好的二进制文件复制到容器中运行
FROM debian:bullseye-slim

WORKDIR /app

ENV QA_CONFIG_DIR=/app/config

VOLUME ${QA_CONFIG_DIR}

# grpc服务端口和metrics端口
EXPOSE 50051
EXPOSE 2338

COPY --from=builder /app/target/release/qa-svc /app/qa-svc

# 设置deb镜像源，这里我使用aliyun的镜像
RUN echo "deb http://mirrors.aliyun.com/debian bullseye main" > /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian-security bullseye-security main" >> /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian bullseye-updates main" >> /etc/apt/sources.list &&  \
    apt-get update && apt-get install -y ca-certificates vim bash curl net-tools \
    apt-transport-https && update-ca-certificates && apt-get clean &&  \
    rm -rf /var/lib/apt/lists/* && mkdir /app/config

CMD ["/app/qa-svc"]
