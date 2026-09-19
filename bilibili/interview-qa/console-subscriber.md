# console-subscriber 使用指南（实战验证版）

> `console-subscriber` 是 Tokio 官方的任务追踪/观测库（`tokio-console` 的服务端），用于实时查看异步任务、资源占用、调度延迟等。仓库：<https://github.com/tokio-rs/console>
>
> 本文配置已经过实战验证，可直接照搬。

---

## 1. 前置条件

- Rust **1.80+**（支持 `--check-cfg`；1.77–1.79 需 nightly）
- 运行观测功能时，Tokio 必须带 `tokio_unstable` 编译，否则会 panic：

  ```text
  task tracing requires Tokio to be built with RUSTFLAGS="--cfg tokio_unstable"!
  ```

## 2. 依赖配置（Cargo.toml）

```toml
[dependencies]
tokio = { version = "1.53.1", features = ["full"] }

# 用于抓取异步任务运行状态（仅本地调试，线上勿开）
console-subscriber = { version = "0.5", optional = true }

[features]
tokio-console = ["dep:console-subscriber"]
```

要点：

- `optional = true`：console-subscriber 默认不参与编译
- `tokio-console = ["dep:console-subscriber"]`：feature 名自取，用于启用上面的 optional 依赖
- **不要**在 Cargo.toml 里写 `[build] rustflags`——会报 `unused manifest key: build`，它不属于 manifest

## 3. 开启 tokio_unstable（二选一）

### 方式一：命令行指定（推荐，零配置文件）

```bash
RUSTFLAGS="--cfg tokio_unstable" cargo run --bin console-sub --features tokio-console
```

### 方式二：`.cargo/config.toml`（团队统一）

项目根目录创建 `.cargo/config.toml`（注意是**带点的隐藏目录**，与 Cargo.toml 同级）：

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

> ⚠️ 改了 rustflags 必须 `cargo clean` 一次，让 tokio 重新编译，否则可能继续报 panic。

## 4. 实战代码（src/bin/console-sub.rs）

```rust
use std::{thread::sleep, time::Duration};

// 运行方式：
//   RUSTFLAGS="--cfg tokio_unstable" cargo run --bin console-sub --features tokio-console
#[tokio::main]
async fn main() {
    // 只有在开启了 tokio-console 特性才生效
    #[cfg(feature = "tokio-console")]
    console_subscriber::init();

    println!("hello,world");
    print_number().await;

    let mut i = 0;
    loop {
        i += 1;
        println!("current index1:{}", i);
        sleep(Duration::from_millis(100));
    }
}

async fn print_number() {
    tokio::spawn(async {
        let mut i = 0;
        loop {
            i += 1;
            println!("current index:{}", i);
            sleep(Duration::from_millis(100));
        }
    });
}
```

关键写法：

- `#[cfg(feature = "tokio-console")] console_subscriber::init();` —— 条件编译，不带 feature 时该语句直接不存在，裸 `cargo run` 也能编译通过
- `console_subscriber::init()` 必须在 main 第一行（tokio runtime 启动前）

## 5. 运行方式

```bash
# 方式一：本地调试（开启观测，6669 端口起 gRPC server）
RUSTFLAGS="--cfg tokio_unstable" cargo run --bin console-sub --features tokio-console

# 方式二：直接运行（观测不启用，console-subscriber 不参与编译）
cargo run --bin console-sub
```

另开终端看观测界面：

```bash
cargo install tokio-console   # 只需装一次
tokio-console                 # 默认连接 127.0.0.1:6669
# 或指定地址
tokio-console http://127.0.0.1:6669
```

## 6. 配置项（环境变量）

| 环境变量 | 说明 | 默认值 |
|---|---|---|
| `TOKIO_CONSOLE_BIND` | 监听地址 | `127.0.0.1:6669` |
| `TOKIO_CONSOLE_RETENTION` | 任务数据保留时长 | `6s` |
| `TOKIO_CONSOLE_PUBLISH_INTERVAL` | 数据发布间隔 | `1s` |

```bash
TOKIO_CONSOLE_BIND=127.0.0.1:6669 RUSTFLAGS="--cfg tokio_unstable" cargo run --bin console-sub --features tokio-console
```

代码内自定义：

```rust
console_subscriber::ConsoleLayer::builder()
    .server_addr(([127, 0, 0, 1], 6669))
    .spawn();
```

## 7. 常见坑排查（全部来自实战）

| 现象 | 原因 | 解决 |
|---|---|---|
| `task tracing requires Tokio to be built with RUSTFLAGS...` | tokio 没用 unstable flag 编译 | 加 `RUSTFLAGS="--cfg tokio_unstable"` 后 `cargo clean` 重编 |
| 改了 flag 还是报同样的 panic | 旧编译缓存 | `cargo clean` 一次 |
| `warning: unused manifest key: build` | 把 `[build] rustflags` 写进了 Cargo.toml | 移到项目根目录 `.cargo/config.toml` |
| `cannot find crate console_subscriber`（E0433） | optional 依赖没启用，代码却无条件调用 | 运行时加 `--features tokio-console`；或给 init() 加 `#[cfg(feature = "tokio-console")]` |
| tokio-console 连接后无数据 | 应用没初始化 subscriber | `console_subscriber::init()` 放 main 第一行 |
| 端口被占用 | 6669 冲突 | `TOKIO_CONSOLE_BIND` 换端口 |
| tokio-console 中任务一直 running | async 里用了 `std::thread::sleep` 阻塞工作线程 | 改用 `tokio::time::sleep(...).await` |
| spawn 的任务没被等待 | JoinHandle 未 await 即丢弃（即发即弃） | 需要等待就保存 handle 并 `.await`；明确是有意即发即弃 |

## 8. tokio-console 客户端按键

| 按键 | 功能 |
|---|---|
| `t` | 任务列表视图 |
| `r` | 资源视图（mutex、semaphore 等） |
| `a` | 异步操作视图 |
| `←/→` 或 `hjkl` | 切换/移动 |
| `Enter` | 任务详情（调用栈、span、waker） |
| `q` | 退出 |

---

## ⚠️ 重要：线上/生产环境不要开启

`console-subscriber` + `tokio_unstable` **仅限本地开发调试**：

1. **性能开销**：每个 task 的创建、poll、waker、span 都会被记录
2. **内存增长**：任务历史数据驻留内存
3. **信息泄露**：6669 gRPC 端口暴露任务详情、调用栈；误绑 `0.0.0.0` 即安全漏洞
4. **稳定性**：unstable API，Tokio 升级可能破坏行为

本实战配置天然满足该要求：**release 构建不带 `--features tokio-console`**，console-subscriber 完全不参与编译，零开销零风险：

```bash
cargo build --release   # 线上发布，观测自动关闭
```

## 9. 快速检查清单

- [ ] Cargo.toml：`optional = true` + `[features] tokio-console = ["dep:console-subscriber"]`
- [ ] Cargo.toml 中**没有** `[build]` 段
- [ ] `.cargo/config.toml` 已配置 rustflags（或用命令行 RUSTFLAGS）
- [ ] 改 flag 后已 `cargo clean`
- [ ] 代码：`#[cfg(feature = "tokio-console")] console_subscriber::init();` 在 main 第一行
- [ ] 本地运行带 `--features tokio-console`，客户端 `tokio-console` 能连上
- [ ] 线上 release 构建不带该 feature
