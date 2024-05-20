# 使用Debian的最小镜像构建rust以及python3环境
FROM debian:bullseye-slim

LABEL authors="daheige"
# 安装指定版本的rust
ENV RUST_VERSION=1.77.2

RUN echo "deb http://mirrors.aliyun.com/debian bullseye main" >/etc/apt/sources.list && \
    echo "deb http://mirrors.aliyun.com/debian-security bullseye-security main" >>/etc/apt/sources.list && \
    echo "deb http://mirrors.aliyun.com/debian bullseye-updates main" >>/etc/apt/sources.list && \
    echo "export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static" >> ~/.bashrc && \
    echo "export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup" >> ~/.bashrc && \
    echo "export PATH=\$HOME/.cargo/bin:/usr/local/cargo/bin:\$PATH" >> ~/.bashrc && \
    apt-get clean && apt-get update && apt-get install -y \
    ca-certificates gcc vim bash curl libc6-dev wget git net-tools \
    python3-dev python3-pip && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y && \
    $HOME/.cargo/bin/rustup install $RUST_VERSION && \
    $HOME/.cargo/bin/rustup default $RUST_VERSION && \
    chmod -R a+w $HOME/.cargo && \
    $HOME/.cargo/bin/rustup default stable && \
    echo "#replace source" >> $HOME/.cargo/config && \
    echo "replace-with = 'ustc'" >> $HOME/.cargo/config && \
    echo "[source.crates-io]" >> $HOME/.cargo/config && \
    echo "registry = \"https://github.com/rust-lang/crates.io-index\"" >> $HOME/.cargo/config && \
    echo "[source.ustc]" >> $HOME/.cargo/config && \
    echo "registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"" >> $HOME/.cargo/config && \
    echo "[source.tuna]" >> $HOME/.cargo/config && \
    echo "registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"" >> $HOME/.cargo/config && \
    echo "[source.aliyun]" >> $HOME/.cargo/config && \
    echo "registry = \"https://code.aliyun.com/rustcc/crates.io-index\"" >> $HOME/.cargo/config && \
    echo "[net]" >> $HOME/.cargo/config && \
    echo "git-fetch-with-cli=true" >> $HOME/.cargo/config && \
    echo "[http]" >> $HOME/.cargo/config && \
    echo "check-revoke = false" >> $HOME/.cargo/config && \
    rm -rf /var/lib/apt/lists/*

# 设置环境变量
ENV LANG C.UTF-8

# 设置默认命令
CMD ["bash"]
