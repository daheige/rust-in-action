FROM rust:1.78.0-bullseye

LABEL authors="heige"

# 设置环境变量
ENV LANG C.UTF-8

WORKDIR /app

COPY . .

RUN echo "deb http://mirrors.aliyun.com/debian bullseye main" > /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian-security bullseye-security main" >> /etc/apt/sources.list &&  \
    echo "deb http://mirrors.aliyun.com/debian bullseye-updates main" >> /etc/apt/sources.list &&  \
    echo "export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static" >> ~/.bashrc &&  \
    echo "export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup" >> ~/.bashrc &&  \
    echo "export PATH=/usr/local/cargo/bin:\$PATH" >> ~/.bashrc &&  \
    apt-get update && apt-get install -y ca-certificates gcc cmake make libtool vim bash  \
    curl libc6-dev wget git net-tools protobuf-compiler &&  \
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
    rm -rf /var/lib/apt/lists/*

RUN cd /app && cargo build --release

# 设置默认命令
CMD ["bash"]
