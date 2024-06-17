FROM rust:1.78.0-bullseye

LABEL authors="heige"

# 设置环境变量
ENV LANG C.UTF-8

# 设置rust相关的环境变量
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
    RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

WORKDIR /app

COPY . .

RUN echo "deb http://mirrors.aliyun.com/debian bullseye main" > /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian-security bullseye-security main" >> /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian bullseye-updates main" >> /etc/apt/sources.list &&  \
    apt-get update && apt-get install -y ca-certificates gcc cmake make libtool vim bash curl libc6-dev wget  \
    git net-tools protobuf-compiler apt-transport-https &&  \
    echo "#replace source" >> /usr/local/cargo/config.toml &&  \
    echo "replace-with = 'ustc'" >> /usr/local/cargo/config.toml &&  \
    echo "[source.crates-io]" >> /usr/local/cargo/config.toml &&  \
    echo "registry = \"https://github.com/rust-lang/crates.io-index\"" >> /usr/local/cargo/config.toml &&  \
    echo "[source.ustc]" >> /usr/local/cargo/config.toml &&  \
    echo "registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"" >> /usr/local/cargo/config.toml &&  \
    echo "[source.tuna]" >> /usr/local/cargo/config.toml &&  \
    echo "registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"" >> /usr/local/cargo/config.toml &&  \
    echo "[source.aliyun]" >> /usr/local/cargo/config.toml &&  \
    echo "registry = \"https://code.aliyun.com/rustcc/crates.io-index\"" >> /usr/local/cargo/config.toml &&  \
    echo "[net]" >> /usr/local/cargo/config.toml &&  \
    echo "git-fetch-with-cli=true" >> /usr/local/cargo/config.toml &&  \
    echo "[http]" >> /usr/local/cargo/config.toml &&  \
    echo "check-revoke = false" >> /usr/local/cargo/config.toml && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

# 将上面构建好的二进制文件复制到容器中 todo
