# Rust 线上二进制 Debug 指南

> 面向「线上环境、发布版（release）二进制」的问题排查。
> 线上 Debug 的核心难点：**优化后的二进制符号被剥离、调用栈被内联、panic 信息不足、无法复现**。
> 本文按「准备 → 定位 → 分析 → 复盘」组织，兼顾 Linux 生产环境常见场景。

---

## 目录

1. [核心思路](#1-核心思路)
2. [发布前准备：让线上二进制可调试](#2-发布前准备让线上二进制可调试)
3. [第一步：拿到崩溃/异常现场](#3-第一步拿到崩溃异常现场)
4. [符号化：把地址翻译成函数名](#4-符号化把地址翻译成函数名)
5. [用 gdb / lldb 分析 core dump](#5-用-gdb--lldb-分析-core-dump)
6. [用 perf / bpftrace 做线上性能与火焰图](#6-用-perf--bpftrace-做线上性能与火焰图)
7. [异步运行时（tokio）的栈追踪](#7-异步运行时tokio的栈追踪)
8. [panic 与 backtrace 配置](#8-panic-与-backtrace-配置)
9. [内存问题：泄漏 / 越界 / 悬垂](#9-内存问题泄漏--越界--悬垂)
10. [死锁与卡死排查](#10-死锁与卡死排查)
11. [日志与可观测性](#11-日志与可观测性)
12. [常见问题速查表](#12-常见问题速查表)
13. [工具清单](#13-工具清单)
14. [发布 Checklist](#14-发布-checklist)

---

## 1. 核心思路

线上 Debug 的黄金三件套：

```
现场（crash/卡死/性能）  +  符号（symbols）  +  工具（gdb/perf 等）
```

- **没有符号 = 只能看内存地址**，几乎无法定位。
- **没有现场（core dump / 日志）= 无从下手**。
- 所以真正的关键在**发布前的可调试性设计**，而不是出事之后。

原则：
1. 先保现场，再谈分析（别急着重启，先 dump / 抓栈）。
2. 最小化改动：能在不重启的情况下抓信息就先抓。
3. 复现路径优先：能日志回放/压测复现的，优于纯静态分析。

---

## 2. 发布前准备：让线上二进制可调试

这是最重要的一节。发布版默认 `strip = true`（剥离符号），会让线上排查寸步难行。

### 2.1 Cargo.toml 推荐配置

```toml
[profile.release]
debug = 1          # 关键！生成 debug 信息（1=行号，2=完整）。可配合 split-debuginfo 外置。
debug-assertions = false
overflow-checks = false
lto = "thin"       # fat lto 更激进，会加大内联导致栈更难读，按需选择
codegen-units = 1  # 更好的优化，但编译更慢
panic = "abort"    # 注意：abort 无法 unwind，无法捕获 panic；若需捕获请用 "unwind"
strip = false      # 不要 strip！

# 可选：把 debug 信息拆到单独文件，减小二进制体积
# split-debuginfo = "packed"   # 配合 debug=2 使用（Linux 上支持有限，主要 macOS）
```

> **权衡**：`debug = 1` 会让二进制体积增大（可能 2~4 倍），且可能泄漏内部信息。
> 折中方案：**构建时保留带符号版本，发布时用 `objcopy --only-keep-debug` 把符号单独留档**，线上部署 stripped 版，排查时用留档符号。

### 2.2 分离符号的标准做法

```bash
# 1. 保留完整符号到一个独立文件
objcopy --only-keep-debug myapp myapp.debug

# 2. 剥离部署二进制
objcopy --strip-debug --strip-unneeded myapp myapp.stripped

# 3. 记录 debug link（关联符号文件）
objcopy --add-gnu-debuglink=myapp.debug myapp.stripped
```

部署 `myapp.stripped`，把 `myapp.debug` 存到符号服务器 / S3 / 制品库，**版本号对齐**。出问题时用对应版本符号。

### 2.3 构建元信息注入

```bash
# 把 git commit、build time 编译进二进制
export GIT_COMMIT=$(git rev-parse --short HEAD)
export BUILD_TIME=$(date -u +%Y-%m-%dT%H:%M:%SZ)
```

```rust
// build.rs
fn main() {
    let commit = std::env::var("GIT_COMMIT").unwrap_or_else(|_| "unknown".into());
    let time = std::env::var("BUILD_TIME").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=GIT_COMMIT={commit}");
    println!("cargo:rustc-env=BUILD_TIME={time}");
}
```

```rust
pub const VERSION_STR: &str =
    concat!(env!("CARGO_PKG_VERSION"), "+", env!("GIT_COMMIT"), " @ ", env!("BUILD_TIME"));
```

这样线上 binary 能自报版本，符号才能对上。

---

## 3. 第一步：拿到崩溃/异常现场

### 3.1 开启 core dump

```bash
# 系统层面（容器内需注意，见下）
ulimit -c unlimited
echo '/tmp/core.%e.%p.%t' | sudo tee /proc/sys/kernel/core_pattern
```

容器部署的坑：
- 容器内 `ulimit -c` 常为 0，需要在启动参数 / `docker run --ulimit core=-1` 设置。
- `core_pattern` 写管道（如 `|/usr/bin/apport`）时，容器内可能拿不到。
- 挂载 host 的 core 目录，或让 core 落到容器内可写卷。

### 3.2 不重启先抓现场（关键）

进程卡死但还活着时，**别重启**，先抓：

```bash
# 抓所有线程的 C 栈（不依赖符号也能看到地址）
gdb -p <PID> -batch -ex "thread apply all bt" > /tmp/bt.txt

# 记录内存布局（后续解析地址用）
gdb -p <PID> -batch -ex "info proc mappings" > /tmp/maps.txt

# 快速多次抓，观察卡在哪
for i in $(seq 1 5); do
  echo "=== $i ==="
  gdb -p <PID> -batch -ex "thread apply all bt" 2>/dev/null
  sleep 2
done
```

### 3.3 主动发起 core dump（不杀进程的可选方式）

```bash
# 用 gcore 在进程存活时生成 core，进程继续运行
gcore -o /tmp/core.myapp <PID>
```

> `gcore` 对进程是 STW（所有线程暂停）的，短暂停顿后恢复，适合抓「正在卡死」的现场。

---

## 4. 符号化：把地址翻译成函数名

无符号时的调用栈长这样：

```
#0  0x000055f2a1b3c4d0 in ?? ()
#1  0x000055f2a1b3e210 in ?? ()
```

### 4.1 用 addr2line / llvm-symbolizer 翻译

```bash
# addr2line：地址 -> 文件:行号
addr2line -f -C -e myapp.debug 0x1b3c4d0

# llvm-symbolizer：效果更好，支持内联展开
llvm-symbolizer --obj=myapp.debug 0x1b3c4d0

# 从崩溃栈批量符号化（用 PIE 基址 + 相对地址！）
```

**PIE 基址问题**：现代二进制默认 PIE，栈里是运行时地址，需要减去基址得到相对地址。
基址在 `/proc/<PID>/maps` 里取第一个可执行段的加载地址。

```bash
# 从 core 里取基址
gdb myapp.stripped core -batch -ex "info proc mappings" | head
```

### 4.2 用 rustfilt 还原 Rust 符号

Rust 符号是 mangle 过的（`_ZN4core3fmt...`）。用 `rustfilt` 还原可读形式：

```bash
cargo install rustfilt
nm -C myapp.debug | rustfilt | grep my_crate
# 或对栈里的符号逐个还原
echo "_ZN3app3foo17h1234567890abcdefE" | rustfilt
# => app::foo
```

### 4.3 批量「地址 -> 符号」脚本

```bash
#!/usr/bin/env bash
# demangle.sh <binary.debug> <base_addr>  从 stdin 读地址
BIN="$1"; BASE="$2"
while read -r addr; do
  rel=$(printf '0x%x' $((addr - BASE)))
  echo -n "$addr -> "
  llvm-symbolizer --obj="$BIN" "$rel" | tr '\n' ' '
  echo
done
```

---

## 5. 用 gdb / lldb 分析 core dump

### 5.1 加载 core + 符号

```bash
gdb ./myapp.debug ./core.12345
# 或先加载 stripped 再补符号
gdb ./myapp.stripped
(gdb) core-file ./core.12345
(gdb) symbol-file ./myapp.debug
```

### 5.2 常用命令

```
(gdb) bt                 # 当前线程调用栈
(gdb) bt full            # 带局部变量
(gdb) thread apply all bt   # 所有线程
(gdb) info threads       # 线程列表
(gdb) frame 3            # 切到第 3 帧
(gdb) info locals        # 当前帧局部变量
(gdb) info args          # 当前帧参数
(gdb) p *ptr             # 解引用
(gdb) x/16x 0xADDR       # 查看内存
(gdb) info registers     # 寄存器
(gdb) list               # 源码（需 debug info + 源码路径）
```

### 5.3 Rust 查看技巧

- `String` / `Vec` / `&str` 在 gdb 里显示不直观，可用 rust-gdb：

```bash
rust-gdb ./myapp.debug ./core.12345
```

rust-gdb 内置了 Rust 类型的 pretty-printer，能直接打印 `String`、`Vec`、`HashMap` 等。

- 查看枚举（如 `Result`、`Option`）需要看 discriminant：

```
(gdb) p my_result
(gdb) p *my_result.__0     # 大致的内部字段名
```

### 5.4 源码路径映射

如果编译机和线上路径不同，源码会找不到：

```
(gdb) set substitute-path /build/path /local/path
(gdb) directory /local/src
```

---

## 6. 用 perf / bpftrace 做线上性能与火焰图

线上通常不希望 STW，用采样型工具。

### 6.1 perf

```bash
# 采样系统栈（-g 记录调用图）
perf record -F 99 -g -p <PID> -- sleep 30
perf report

# 只采用户态
perf record -F 99 -g --call-graph dwarf -p <PID> -- sleep 30
```

**Rust 关键点**：
- 必须开 `debug = 1` 或启用 frame pointer，否则 `--call-graph fp` 栈不完整。
- `--call-graph dwarf` 更准但开销大（依赖 debug info）。
- 用 `perf script | rustfilt` 还原符号。

### 6.2 生成火焰图

```bash
git clone https://github.com/brendangregg/FlameGraph
perf record -F 99 -g -p <PID> -- sleep 30
perf script | rustfilt | \
  FlameGraph/stackcollapse-perf.pl | \
  FlameGraph/flamegraph.pl > flame.svg
```

### 6.3 bpftrace（低开销、在线）

```bash
# 统计用户态函数耗时
bpftrace -e 'uprobe:/path/to/myapp:myapp::hot_function { @[ustack] = count(); }'

# 观测某函数的调用计数
bpftrace -e 'uprobe:/path/to/myapp:myapp::process { @calls = count(); }'
```

> 需要符号表里有对应符号（`nm` 能查到）。release 且带 debug 的二进制通常可以。

### 6.4 在线诊断神器：process-inspector / pprof-rs

Rust 生态可选：
- [`pprof`](https://crates.io/crates/pprof) crate：进程内 CPU profiler，可暴露 `/debug/pprof` 接口在线采样。
- [`tokio-console`](https://crates.io/crates/tokio-console)：异步任务级可视化（见下一节）。

---

## 7. 异步运行时（tokio）的栈追踪

tokio 下 `bt` 常只看到 `epoll_wait` / `park`，看不到业务逻辑，因为任务是异步调度的。

### 7.1 用 tokio-console 看任务

```toml
[dependencies]
console-subscriber = "0.4"
tokio = { version = "1", features = ["full", "tracing"] }
```

```rust
fn main() {
    console_subscriber::init();       // 启动时开启
    // ... 运行 tokio
}
```

```bash
cargo install tokio-console
tokio-console http://127.0.0.1:6669   # 默认端口
```

可以看到：**每个任务的当前 poll 状态、耗时、waker、await 点**，能直接定位「哪个任务卡住了」。

### 7.2 抓某个线程在跑的异步栈

- 若任务长时间不 yield（CPU 卡在某个 poll 里），`perf` / `gdb` 的 native 栈仍有效，可在 `bt` 里看到该 poll 内部。
- 结合 `tokio-console` 的 task id 与 `tracing` span 定位业务。

### 7.3 tracing + 结构化日志

```rust
use tracing::{info, instrument};

#[instrument]   // 自动记录入参、耗时、nesting
async fn handle_order(order_id: u64) -> Result<(), Error> {
    info!("processing");
    // ...
    Ok(())
}
```

配合 `tracing-subscriber` + JSON 输出，便于线上检索和还原时序。

---

## 8. panic 与 backtrace 配置

### 8.1 让线上打印 backtrace

```bash
# 运行环境设置（推荐在服务启动脚本里 export）
export RUST_BACKTRACE=1          # 简略栈
export RUST_BACKTRACE=full       # 完整栈
export RUST_LIB_BACKTRACE=1      # 影响 backtrace crate
```

> `RUST_BACKTRACE` 在 `panic=abort` 下效果有限（仍会打印，但无法 catch）。
> 需要完整符号行号，仍要求二进制带 debug info（见第 2 节）。

### 8.2 自定义 panic hook

```rust
std::panic::set_hook(Box::new(|info| {
    let backtrace = std::backtrace::Backtrace::force_capture();
    eprintln!("PANIC: {info}\n{backtrace}");
    // 上报到监控系统（sentry / rollbar / 自建）
}));
```

配合 [`sentry`](https://crates.io/crates/sentry) 或 `sentry-tracing` 可自动上报带符号栈。

### 8.3 backtrace 符号化

用 [`backtrace-symbolize`] / 手动 `addr2line`：
Sentry 收到栈后需要上传**对应的 debug 符号文件**（`sentry-cli debug-files upload`），才能显示行号。

```bash
sentry-cli debug-files upload myapp.debug
```

---

## 9. 内存问题：泄漏 / 越界 / 悬垂

### 9.1 内存泄漏排查

```bash
# 观察 RSS 趋势
cat /proc/<PID>/status | grep VmRSS

# 用 heaptrack 追踪分配点
heaptrack ./myapp
heaptrack_gui heaptrack.myapp.*.gz
```

Rust 常见泄漏来源：
- `Rc`/`Arc` 循环引用（需 `Weak` 打破）。
- 全局 `static` 不断 push 的容器（`lazy_static`/`OnceLock` 缓存无上限）。
- `spawn` 的任务/线程未回收。
- `mem::forget` / `Box::leak`。
- 异步 channel 消费慢于生产。

### 9.2 越界 / UB / 悬垂（unsafe 相关）

release 关闭了边界检查的 panic 展示，越界/UB 可能表现为随机崩溃。用：

```bash
# 编译时开启 sanitizer（需要在测试/预发环境用同配置二进制）
RUSTFLAGS="-Zsanitizer=address" cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu
```

- AddressSanitizer：内存越界、悬垂。
- ThreadSanitizer：数据竞争。
- MemorySanitizer：未初始化内存。

> 线上一般不开 sanitizer（开销大），但**预发用相同业务路径跑 ASan** 是复现 unsafe bug 的有效手段。

### 9.3 用 valgrind（快速验证）

```bash
valgrind --leak-check=full --track-origins=yes ./myapp
```

---

## 10. 死锁与卡死排查

### 10.1 定位死锁现场

```bash
# 抓所有线程栈
gdb -p <PID> -batch -ex "thread apply all bt" -ex detach

# 或直接生成 core
gcore -o /tmp/core <PID>
```

找**互相等待**的模式：
- 线程 A 持锁 1 等锁 2，线程 B 持锁 2 等锁 1。
- `Mutex::lock` / `RwLock` 内部 ff 帧。

### 10.2 用 parking_lot 的死锁检测

```toml
parking_lot = { version = "0.12", features = ["deadlock_detection"] }
```

```rust
// 线上触发检测（比如监听信号 / 定时）
std::thread::spawn(|| loop {
    std::thread::sleep(Duration::from_secs(30));
    let deadlocks = parking_lot::deadlock::check_deadlock();
    if !deadlocks.is_empty() {
        eprintln!("发现 {n} 处死锁", n = deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            eprintln!("死锁组 #{i}");
            for t in threads { eprintln!("{:?}", t.backtrace()); }
        }
    }
});
```

### 10.3 异步死锁

tokio 里「同步锁跨 await 持有」或「单线程 runtime 里阻塞」会导致任务永不推进。
- 检查是否在 `async` 里用了 `std::sync::Mutex` 并跨 `.await` 持锁。
- 单线程 runtime（`new_current_thread`）里跑阻塞任务会卡住整个 runtime。
- 用 `tokio-console` 看是否有任务长时间 `poll` 不返回。

---

## 11. 日志与可观测性

线上 Debug 的前提是「信息足够」。建议：

| 手段 | 工具 | 用途 |
|------|------|------|
| 结构化日志 | `tracing` + `tracing-subscriber`(JSON) | 时序还原、检索 |
| 指标 | `metrics` + Prometheus | 趋势、突刺、GC/内存 |
| 追踪 | `tracing` + OpenTelemetry | 跨服务链路 |
| 错误上报 | `sentry` | 崩溃聚合 + 符号化栈 |
| 动态日志级别 | `tracing_subscriber::reload` | 不重启调高某模块日志 |

**动态开启详细日志**（线上救命）：

```rust
use tracing_subscriber::{reload, EnvFilter};
// 保存 reload handle，通过管理接口 / 信号动态调整 filter
let (filter, handle) = reload::Layer::new(EnvFilter::new("info"));
// 线上一键: handle.modify(|f| *f = EnvFilter::new("my_crate=debug"))
```

---

## 12. 常见问题速查表

| 现象 | 可能原因 | 排查手段 |
|------|----------|----------|
| 进程崩溃无栈 | `panic=abort` + 无 core | 开 core dump + `RUST_BACKTRACE` |
| 栈全是 `??` | 二进制被 strip | 用对应 debug 符号 + `addr2line` |
| 栈看不到业务函数 | LTO 内联 | `debug=1` + `llvm-symbolizer` 展开内联 |
| CPU 打满 | 死循环 / 算法问题 | `perf top` + 火焰图 |
| RSS 持续增长 | 内存泄漏 | heaptrack / 观察 VmRSS |
| 请求卡住不返回 | 死锁 / await 未唤醒 | gdb bt / tokio-console |
| 随机崩溃 | UB / unsafe 越界 | 预发跑 ASan |
| 性能慢慢变差 | 锁竞争 / 分配压力 | perf + jemalloc 统计 |
| 异步任务不推进 | 阻塞 runtime | tokio-console + 查阻塞点 |

---

## 13. 工具清单

| 工具 | 安装 | 用途 |
|------|------|------|
| gdb / rust-gdb | 系统包管理 | core dump、栈 |
| lldb | 系统 | macOS / 替代 |
| addr2line / llvm-symbolizer | binutils / llvm | 地址符号化 |
| rustfilt | `cargo install rustfilt` | 还原 mangle 符号 |
| perf | linux-tools | CPU 采样 |
| FlameGraph | git clone | 火焰图 |
| bpftrace | 系统 | 在线内核/用户态观测 |
| heaptrack | 系统 | 内存分配追踪 |
| valgrind | 系统 | 内存/线程检查 |
| tokio-console | `cargo install tokio-console` | 异步任务可视化 |
| pprof (crate) | Cargo 依赖 | 进程内 CPU profiling |
| sentry-cli | 官方脚本 | 符号上传 / 崩溃聚合 |
| cargo-bloat | `cargo install cargo-bloat` | 二进制体积分析 |
| cargo-flamegraph | `cargo install flamegraph` | 快速火焰图 |

---

## 14. 发布 Checklist

发布前逐条确认，能省掉 80% 的线上排查痛苦：

- [ ] `[profile.release]` 设了 `debug = 1`（或至少外置符号）。
- [ ] 每次发布的二进制都**归档了对应的 debug 符号文件**，版本可查。
- [ ] 二进制内置 git commit + build time（`build.rs` 注入）。
- [ ] 线上开启 core dump（`ulimit -c unlimited` + `core_pattern` 可写）。
- [ ] 服务启动脚本 `export RUST_BACKTRACE=1`。
- [ ] 设置了 panic hook，崩溃自动上报（sentry 等）。
- [ ] 结构化日志 + 可动态调整日志级别。
- [ ] 有基础指标（内存/RSS、CPU、QPS、延迟）。
- [ ] 明确「出事第一步」SOP：**先抓栈/gcore，再考虑重启**。
- [ ] 容器场景确认 core dump 能落盘、符号能取到。

---

## 附：一次典型线上排查流程（模板）

```
1. 收到告警（崩溃 / 延迟上升 / OOM）
2. 确认版本：从 /version 接口或 binary --version 拿到 commit hash
3. 保现场：
   - 崩溃 → 找 core dump 文件
   - 卡死 → gcore 或 gdb -p 抓栈
   - 性能 → perf record 采样 30s
4. 取对应版本符号文件（debug link / 制品库）
5. 符号化：
   gdb myapp.debug core → bt full
   perf script | rustfilt | flamegraph
   tokio-console 看任务
6. 定位根因 → 写复现用例 → 修复 → 回归（预发可用 ASan/TSan）
7. 复盘：是否可观测性不足？是否发布配置遗漏？更新 Checklist
```

---

> 关键一句话：**线上 Debug 的能力，取决于发布前是否埋好了「符号 + 现场 + 日志」三件事。**
