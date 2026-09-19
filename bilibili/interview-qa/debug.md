# rust debug实战

1. 在编译时，保留debug符号，同时优化二进制文件大小

```toml
# Cargo.toml —— 保留 debug 符号，同时优化体积
# 事前准备（编译时就要做对）
[profile.release]
debug = true          # 生成符号表，core dump / perf 用得上
strip = "debuginfo"   # 或者不 strip，二选一看磁盘/包大小策略
lto = true            # 优化别关
codegen-units = 1
panic = "abort"
```

2. 运行定位，可以设置panic hook钩子，也可以通过 RUST_BACKTRACE 获取运行堆栈信息。

下面以堆栈backtrace为例：

- 在执行 `cargo build --release` 构建编译
- 我们可以执行`RUST_BACKTRACE=full ./target/release/hello-debug`
- 在`Cargo.toml` 加 `panic = "abort"`，以及在代码中设置全局 panic hook（用 `human-panic` 或自己接 `std::panic::set_hook`
  ），把 panic 栈写到日志/磁盘，便于事后分析。
- 同时，也推荐结构性问题优先用 `tracing`/`log` + `tracing-opentelemetry`，把 request_id、关键状态打进日志，比事后抓现场靠谱得多。

运行过程中抛出的堆栈信息如下：

```ini
current index:87
current index:88
current index:89
current index:90
current index:91
current index:92
current index:93
current index:94
current index:95
current index:96
current index:97
current index:98
current index:99
current index:100

thread 'main' (5832787) panicked at src/hello-debug.rs:26:13:
current index:100 exec panic
stack backtrace:
0:        0x10e762560 - <<std[db781add53f00ba4]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as
core[e634693f4a6b938f]::fmt::Display>::fmt
1:        0x10e755757 - core[e634693f4a6b938f]::fmt::write
2:        0x10e76170b - <std[db781add53f00ba4]::sys::stdio::unix::Stderr as std[db781add53f00ba4]::io::Write>::write_fmt
3:        0x10e761395 - std[db781add53f00ba4]::panicking::panic_with_hook
4:        0x10e787a71 - std[db781add53f00ba4]::panicking::panic_handler::{closure#0}
5:        0x10e787a29 - std[db781add53f00ba4]::sys::backtrace::__rust_end_short_backtrace::<std[db781add53f00ba4]::
panicking::panic_handler::{closure#0}, !>
6:        0x10e787f34 - __rustc[100742bb89c490cb]::rust_begin_unwind
7:        0x10e78bf0b - core[e634693f4a6b938f]::panicking::panic_fmt
8:        0x10e7548fe - hello_debug[6440582796326c41]::main
9:        0x10e7547d6 - std[db781add53f00ba4]::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
10:        0x10e754c20 - _main
11:     0x7ff80e961530 - <unknown>
[1]    19294 abort RUST_BACKTRACE=full ./target/release/hello-debug
```

## 事中：现场抓取

1. 通过 ps 先查看进程id

```shell
ps aux | grep hello-debug
```

输出结果为：

```ini
heige  20287   0.0  0.0 34130692    700 s006  Ss+  10:24上午   0:00.14 target/debug/hello-debug
```

第二列就是进程id，此时为 20287

2. 通过 top 看进程运行情况

```shell
top -l 1 -pid 20287
# 或者下面的方式
top -l 0 -s 1 -pid 20287
```

参数说明：

- -l 1：采样 1 次 就退出（相当于 Linux 的 -b -n 1）
- -pid 20287：只显示指定进程（相当于 Linux 的 -p）
- -l 0 表示无限采样，-s 1 是刷新间隔 1 秒。

简单总结：Linux 用 `top -b -n 1 -p`，Mac 用 `top -l 1 -pid`，脚本场景 Mac 上优先 `ps -p`

## 设置panic 钩子记录日志

```rust
// 设置全局panic钩子，打印日志
    std::panic::set_hook(Box::new(|info| {
        let info = info.location().unwrap();
        println!("panic info:{:?}", info.to_string());
        println!(
            "location file: {} line: {} column:{}",
            info.file(),
            info.line(),
            info.column()
        );
    }));
```
