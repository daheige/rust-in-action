# Rust 线上二进制 Debug 指南

> 核心思路：**平时埋好钩子，出事时能拿到足够信息**。
> 按「事前准备 → 事中排查 → 常用工具」梳理。

---

## 1. 事前准备（编译时就要做对）

```toml
# Cargo.toml —— 保留 debug 符号，同时优化体积
[profile.release]
debug = true          # 生成符号表，core dump / perf 用得上
strip = "debuginfo"   # 或者不 strip，二选一看磁盘/包大小策略
lto = true            # 优化别关
codegen-units = 1
```

- **保留符号 + 归档对应版本的二进制/symbol 文件**。线上崩溃栈只有地址（如 `0x55xxxx`），没有符号文件就无法定位。
- 建议发版时把二进制存到 S3/制品仓库，并记录 commit hash / 构建 ID，崩溃时按版本拉取对应符号。

## 2. 运行时定位：先拿 backtrace

```bash
# 让 panic / abort 时打印调用栈（很多发行版默认已开）
RUST_BACKTRACE=full ./your-app
```

- 加 `panic = "abort"` + 全局 panic hook（用 `human-panic` 或自己接 `std::panic::set_hook`），把 panic 栈写到日志/磁盘，便于事后分析。
- 结构性问题优先用 `tracing`/`slog` + `tracing-opentelemetry`，把 request_id、关键状态打进日志，比事后抓现场靠谱得多。

## 3. 事中：现场抓取

### 挂 gdb 看栈（Live 调试，影响小）

```bash
# attach 到运行进程
gdb -p <pid>
(gdb) bt                     # 看完整调用栈
(gdb) thread apply all bt    # 所有线程
(gdb) info locals            # 局部变量（需要 debug symbols）
```

### 抓 core dump（崩溃后分析，黄金证据）

```bash
ulimit -c unlimited
cat /proc/sys/kernel/core_pattern
```

```bash
# 崩溃后用 gdb + 对应版本的二进制 + symbols 分析
gdb ./your-app core.<pid>
(gdb) bt full
```

Rust 类型在 gdb 里比较难看，推荐：

- `rustup component add rust-gdb`，使用 **`rust-gdb`**
- 或使用 **[CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)** 在本地复现分析

## 4. 线上性能 / 卡死排查

| 场景 | 工具 |
|---|---|
| CPU 高 | `perf top -p <pid>`、`cargo flamegraph` |
| 内存泄漏/膨胀 | `heaptrack`、jemalloc 的 `prof` 开启 malloc 采样 |
| 线程卡死/死锁 | `gdb thread apply all bt` 看哪些线程停在 lock 上 |
| 异步任务卡住 | tokio-console（需加 `--cfg tokio_unstable` 编译） |
| tokio 任务管理 | `tokio::task::JoinHandle::abort` + `tracing` 埋点 |

## 5. 生产环境的稳妥姿势

- **不要把 gdb/strace 直接跑在关键服务上太久**，优先抓取快照（core/线程栈/heaptrack profile）然后离线分析。
- **远程诊断 crate**：考虑接入 Sentry / Backtrace.io（`sentry-rust` + `panic` 集成）自动收集崩溃栈 + breadcrumbs。
- 如果线上允许热修逻辑，可用 `dlopen` 动态加载 `.so` 做降级/旁路；常规建议还是靠日志 + 配置开关回滚。

---

## 一句话总结

**release 构建保留 symbols 并归档、崩溃留 core + backtrace、平时用 tracing 把上下文打全** —— 这三件事做到，线上 90% 的问题都能离线还原。

> 若调试对象是 OpenObserve 等 Rust 可观测性平台：思路完全一样，先利用其内置的 tracing/metrics 日志和指标面，再结合上述工具抓现场。
