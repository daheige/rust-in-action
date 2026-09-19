# Rust 面试指南

> 基于本仓库 `interview-qa` 整理，覆盖 **语言基础、并发与异步、工程化、线上 Debug 实战** 四大板块。
> 适用场景：Rust 初中高级岗位技术面试准备 / 技术分享 / 直播讲解。

---

## 目录

1. [语言基础](#一语言基础)
   - [所有权与内存模型](#1-所有权与内存模型)
   - [引用与借用（含可变引用）](#2-引用与借用含可变引用)
   - [生命周期](#3-生命周期)
   - [Copy 与 Clone](#4-copy-与-clone)
   - [智能指针](#5-智能指针)
   - [结构体与枚举](#6-结构体与枚举)
   - [Trait 系统（泛型 vs 特征对象）](#7-trait-系统泛型-vs-特征对象)
   - [闭包（Fn / FnMut / FnOnce）](#8-闭包fn--fnmut--fnonce)
   - [错误处理（Option / Result / panic）](#9-错误处理option--result--panic)
   - [宏（声明宏 vs 过程宏）](#10-宏声明宏-vs-过程宏)
   - [其他高频基础题](#11-其他高频基础题)
2. [并发与异步](#二并发与异步)
   - [并发模型](#1-并发模型)
   - [Send 与 Sync](#2-send-与-sync)
   - [Mutex 与 Arc](#3-mutex-与-arc)
   - [共享数据与资源保护](#4-共享数据与资源保护)
   - [死锁与竞争条件](#5-死锁与竞争条件)
   - [Channel 与 Select](#6-channel-与-select)
   - [全局变量](#7-全局变量)
   - [异步编程（Future / async / await / Pin）](#8-异步编程future--async--await--pin)
   - [Tokio 运行时](#9-tokio-运行时)
3. [工程化](#三工程化)
   - [Cargo.toml 与 Cargo.lock](#1-cargotoml-与-cargolock)
   - [crate、module 与包管理](#2-cratemodule-与包管理)
   - [条件编译与 build.rs](#3-条件编译与-buildrs)
   - [标准库、I/O 与网络](#4-标准库io-与网络)
   - [Web 开发与数据库](#5-web-开发与数据库)
   - [Unsafe 代码管理](#6-unsafe-代码管理)
   - [文档（Rustdoc）](#7-文档rustdoc)
4. [线上 Debug 实战](#四线上-debug-实战)
   - [核心思路](#1-核心思路)
   - [发布前准备：让 release 二进制可调试](#2-发布前准备让-release-二进制可调试)
   - [拿到现场：backtrace / core dump / gcore](#3-拿到现场backtrace--core-dump--gcore)
   - [符号化：地址翻译成函数名](#4-符号化地址翻译成函数名)
   - [gdb / lldb / rust-gdb 分析](#5-gdb--lldb--rust-gdb-分析)
   - [性能排查：perf / 火焰图 / bpftrace](#6-性能排查perf--火焰图--bpftrace)
   - [Tokio 异步栈追踪与 tokio-console](#7-tokio-异步栈追踪与-tokio-console)
   - [内存问题：泄漏 / 越界 / 悬垂](#8-内存问题泄漏--越界--悬垂)
   - [死锁与卡死排查](#9-死锁与卡死排查)
   - [日志与可观测性](#10-日志与可观测性)
   - [常见问题速查表 + 发布 Checklist](#11-常见问题速查表--发布-checklist)
5. [面试答题建议](#五面试答题建议)

---

# 一、语言基础

## 1. 所有权与内存模型

**Q：Rust 如何保证内存安全？**

Rust 通过两种主要方法确保内存安全：

- **严格类型系统**：编译时检查类型，在代码运行前捕获错误，如访问已移动或已借用的值。
- **所有权和借用系统**：
  - 每个值有且只有一个所有者，所有者离开作用域时值被自动释放（RAII）；
  - 借用允许函数临时使用值而不取得所有权，由借用检查器（Borrow Checker）在编译时强制执行规则：
    - 同一时刻最多一个可变引用 `&mut T`；
    - 或者任意多个不可变引用 `&T`；
    - 两者不可同时存在。

> 加分点：对比 C/C++（半手动内存管理）和 Java/Go/Python（GC），Rust 在**编译时**验证引用，零运行时开销。
> 借用检查器同时保障了并发安全：数据竞争的本质是"同时存在多个可变引用"，所有权规则从根上杜绝了这一点。

## 2. 引用与借用（含可变引用）

**Q：Rust 中的可变引用是什么？**

可变引用（可变借用）允许在不转移所有权的情况下修改值，用 `&mut` 创建。编译器强制：

- **唯一性**：同一作用域内，一个值只能有一个 `&mut`，且不能有其他 `&` 或 `&mut` 指向它；
- **不可变引用可并存**：有 `&T` 时可以有任意多个 `&T`，但不能有任何 `&mut`；
- **不自动提升为所有权**：`&mut` 不是 Copy 类型，不能通过赋值转移所有权。

> 引用本质是"借用"，父函数保留所有权，子函数获得使用权，无需拷贝。

## 3. 生命周期

**Q：什么是生命周期？**

生命周期描述**引用有效的时间范围**，是类型系统的功能，用于表达不同值及其引用之间的关系。标注形如 `'a`。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

- 生命周期**防止悬垂引用**：编译器确保引用不会比它指向的数据活得更久；
- 常见标注：函数签名、结构体（持有引用时必须标注）、`static` 生命周期（整个程序执行期间有效）。

## 4. Copy 与 Clone

**Q：Copy 和 Clone 特征有什么区别？**

| | Copy | Clone |
|---|---|---|
| 语义 | 按位拷贝，拷贝后两值独立 | 显式 `clone()`，可自定义拷贝逻辑 |
| 成本 | 极低（数字、布尔、指针等） | 可能很高（String、Vec 等堆分配类型） |
| 触发方式 | 赋值即拷贝 | 必须显式调用 `.clone()` |
| 关系 | Copy 要求先实现 Clone | Clone 不要求 Copy |

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // 显式深拷贝
```

## 5. 智能指针

**Q：Rust 中的智能指针是什么？**

智能指针是具有指针特性 + 附加元数据/所有权语义的数据结构，离开作用域自动释放内存，避免内存泄漏和悬垂指针。

| 智能指针 | 所有权语义 | 线程安全 | 典型场景 |
|---|---|---|---|
| `Box<T>` | 独占（堆分配） | — | 递归类型、大对象转移 |
| `Rc<T>` | 共享（引用计数） | 否 | 单线程共享 |
| `Arc<T>` | 共享（**原子**引用计数） | 是 | 多线程共享 |
| `RefCell<T>` | 内部可变性（运行时检查） | 否 | 单线程绕开编译期借用规则 |
| `Mutex<T>` / `RwLock<T>` | 内部可变性（互斥） | 是 | 多线程修改共享数据 |
| `Weak<T>` | 弱引用（配合 Rc/Arc 打破循环） | — | 防止循环引用泄漏 |

> 经典组合：`Arc<Mutex<T>>` 多线程共享可变数据；`Rc<RefCell<T>>` 单线程共享可变数据。

## 6. 结构体与枚举

**Q：struct 和 enum 有什么区别？**

- **struct**：把不同类型的相关数据组合成一个单元，表示实体/对象；字段同时存在。
- **enum**：表示一组有限的命名状态/变体，每个变体还可以**携带不同类型和数量的附加数据**（tagged union）。

```rust
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },    // 匿名结构体
    Write(String),              // 元组
    ChangeColor(i32, i32, i32),
}
```

> 加分点：Rust 的 enum 是代数数据类型（ADT），`Option<T>`、`Result<T, E>` 都是 enum，配合 `match` 做穷尽性检查（exhaustiveness checking）。

## 7. Trait 系统（泛型 vs 特征对象）

**Q：trait 系统是什么？dyn Trait 与 impl Trait 的区别？什么是对象安全？**

- trait 定义类型必须实现的方法集合，支持泛型编程与代码复用；一个类型可实现多个 trait。
- **impl Trait**：编译期**静态分发**（单态化），性能好，零开销，但每种类型生成一份代码（代码体积大）。
- **dyn Trait**：运行时通过 **vtable 动态分发**，有一层间接调用开销，代码体积小，支持异构集合。

```rust
fn f(x: impl Drawable) {}        // 静态分发
fn g(x: &dyn Drawable) {}        // 动态分发
```

**对象安全（object safety）**：trait 要能用 `dyn` 必须满足，核心是：
- 方法不能返回 `Self`；
- 方法不能有泛型参数；
- 除第一个 `self` 参数外不能有 `Self` 出现在参数中。

> 这是中高级岗必考点。

## 8. 闭包（Fn / FnMut / FnOnce）

**Q：Rust 中的闭包是什么？三种捕获方式？**

闭包是捕获其创建环境变量的匿名函数，可推断参数和返回类型，可作为值传递。三种 trait 对应三种捕获方式：

| Trait | 捕获方式 | 说明 |
|---|---|---|
| `Fn` | `&T` 不可变借用 | 不修改捕获值，可多次调用 |
| `FnMut` | `&mut T` 可变借用 | 可修改捕获值，可多次调用 |
| `FnOnce` | 按值移动（T） | 消费捕获值，**只能调用一次** |

- 编译器根据闭包体自动推断实现哪个 trait；
- 闭包同时满足"只读"时，可降级使用：实现了 Fn 的闭包也能当 FnMut 用；
- 所有权模型防止 use-after-free 和数据竞争。

## 9. 错误处理（Option / Result / panic）

**Q：Option 和 Result 的区别？Rust 的错误处理方式？**

- **Option<T>**：`Some(值)` 或 `None`，表示值可能存在或不存在（避免空指针）；
- **Result<T, E>**：`Ok(值)` 或 `Err(错误)`，表示操作可能失败（结构化错误处理）。

```rust
let content = std::fs::read_to_string("a.txt")?; // ? 自动传播错误
```

错误处理机制全景：

1. `Result<T, E>` + `?` 传播 —— 可恢复错误的标配；
2. `Option<T>` —— 值可缺省场景；
3. `panic!` / `panic = "abort"` —— 不可恢复错误，终止线程/进程；
4. 生态库：`thiserror`（定义自定义错误类型）、`anyhow`（应用层错误封装）。

> 加分点：`panic = "unwind"` vs `panic = "abort"`：unwind 可捕获（catch_unwind / sentry 上报），abort 体积小但不能捕获。

## 10. 宏（声明宏 vs 过程宏）

**Q：Rust 支持哪两种宏？**

- **声明宏（macro_rules!）**：匹配代码**模式**并替换生成新代码，基于文本/语法匹配；
- **过程宏（proc-macro）**：编译时对**语法树（TokenStream）**操作的 Rust 函数，三种形式：
  - `#[derive(...)]` 派生宏
  - 属性宏 `#[route("/")]`
  - 函数式宏 `sqlx::query!()`

> 宏很强大但复杂难调试，应谨慎使用。面试可举例：serde 的 derive、tokio::select!、vec!。

## 11. 其他高频基础题

| 问题 | 要点 |
|---|---|
| 元组 tuple | 不同类型值的集合 `(10, "hello", true)`，类似数组但元素类型可不同 |
| match 模式匹配 | 按顺序匹配第一个命中分支；编译器做穷尽性检查 |
| 切片 slice | `&[T]`，指向连续内存的（指针+长度），轻量；`[start..end]`，end 不包含 |
| 迭代器 | 实现 `Iterator` trait 的类型，惰性按需产生值；`map/filter/collect` |
| 迭代器 vs 生成器 | 生成器用 `yield` 惰性产出值，可表示无限序列（迭代器是 trait，生成器是语法特性） |
| 函数指针 | `fn(i32) -> i32` 类型，可传给函数或存入数据结构；与 Fn trait 闭包不同，函数指针不捕获环境 |
| String vs str | `String` 堆分配可增长；`&str` 是 UTF-8 字节切片（视图） |
| 标准集合 | `Vec`（动态数组）、`HashMap`（键值对）、`HashSet`（唯一键） |
| 泛型 | `<T>` 类型参数，单态化保证零成本抽象（编译时展开具体类型） |
| 条件编译 | `#[cfg(target_os = "linux")]` 等属性，选择性编译代码块 |
| `static` 生命周期 | 数据存活整个程序执行期；`static` 变量全局可访问 |

---

# 二、并发与异步

## 1. 并发模型

**Q：Rust 的并发模型是什么？**

- 基于**所有权和借用**：每个值由单个线程拥有，所有权可通过消息传递跨线程转移；
- "通过通信来共享内存"（channel），而非"通过共享内存来通信"（锁）；
- 编译期防止数据竞争：不满足 Send/Sync 的类型无法跨线程传递，编译直接报错；
- 提供线程（`std::thread::spawn`）+ 同步原语（Mutex、channel 等）+ async 运行时三层工具。

## 2. Send 与 Sync

**Q：Send 和 Sync 是什么？**

- **Send**：类型的**值**可在线程间安全转移（move 到另一个线程）；
- **Sync**：类型的**引用** `&T` 可在线程间安全共享（多线程同时读）。

记忆口诀：**Send 能搬家，Sync 能合租**。大部分类型编译器自动实现；`Rc<T>` 不是 Send/Sync，`Arc<T>` 是。

```rust
struct MyStruct { data: u32 }
unsafe impl Send for MyStruct {}   // 手动实现需要 unsafe
```

> 手动实现 Send/Sync 是 `unsafe` 的责任：你必须自己保证线程安全。

## 3. Mutex 与 Arc

**Q：Mutex 和 Arc 分别解决什么问题？为什么经常一起用？**

- **Mutex（互斥锁）**：解决**并发访问**——任何时刻只有一个线程能持有锁、修改数据；`lock()` 返回 `MutexGuard`（可变引用），离开作用域自动解锁。
- **Arc（原子引用计数）**：解决**所有权共享**——Rust 禁止直接跨线程转移所有权，Arc 通过原子操作的引用计数让多个线程共享同一数据，计数归零自动释放。

```rust
let data = Arc::new(Mutex::new(0));
let t1_data = Arc::clone(&data);
let t2_data = Arc::clone(&data);
let t1 = thread::spawn(move || { *t1_data.lock().unwrap() += 1; });
let t2 = thread::spawn(move || { *t2_data.lock().unwrap() += 1; });
```

> 加分点：Mutex 在 Rust 中"守护数据"而非"守护代码"（data-oriented lock）；`MutexGuard` 不可跨 `.await` 使用（不是 Send 时编译报错），这是异步死锁的常见来源。

## 4. 共享数据与资源保护

三层防护体系：

1. **所有权系统**：每个值唯一所有者，离开作用域自动释放 → 防悬垂指针；
2. **借用规则**：`&mut` 唯一、`&` 可并存 → 编译期防数据竞争；
3. **同步类型**：`Mutex`/`RwLock`/`Atomic*` → 运行时跨线程保护。

## 5. 死锁与竞争条件

**Q：Rust 能避免死锁吗？如何排查？**

Rust 的所有权规则能编译期防**数据竞争**，但**不能防死锁和逻辑竞争**（锁是运行时原语）。

- **死锁**：两个线程互相等待对方持有的锁。避免原则：
  - 避免嵌套持锁；
  - 所有线程按**固定顺序**获取多把锁；
  - `parking_lot` 开 `deadlock_detection` feature 在线检测。
- **竞争条件**：结果取决于线程调度顺序。可用 `AtomicUsize` 等原子类型做无锁计数；`std::sync::atomic` 提供 `AtomicBool/Isize/Usize/Ptr`。

```rust
// 死锁模式：A 持 L1 等 L2，B 持 L2 等 L1
// 排查：gdb -p <PID> -batch -ex "thread apply all bt" 找互相等待的线程对
```

## 6. Channel 与 Select

**Q：channel 是什么？select 的作用？**

- **channel**：并发任务间单向传递消息的机制。
  - 标准库 `std::sync::mpsc`（多生产者单消费者）；
  - tokio `mpsc`（带缓冲的异步通道）、`broadcast`（广播）、`oneshot`、`watch`。
- **select**（`tokio::select!`）：异步 I/O 多路复用，同时监听多个异步操作，任一就绪即执行对应分支。

```rust
let (tx, mut rx) = mpsc::channel(10);
tokio::spawn(async move {
    while let Some(msg) = rx.recv().await { println!("{msg}"); }
});

tokio::select! {
    _ = time::sleep(Duration::from_millis(200)) => println!("timeout"),
    _ = interval.tick() => println!("tick"),
}
```

## 7. 全局变量

**Q：Rust 中如何声明全局变量？**

`static` 声明全局变量，'static 生命周期，需要常量表达式初始化。

```rust
static mut APP_DEBUG: bool = true;              // 需要 unsafe，不推荐
static APP_DEBUG2: AtomicBool = AtomicBool::new(true);  // 推荐
```

| 方案 | 线程安全 | 需要 unsafe |
|---|---|---|
| `static mut` + `addr_of!` | 无保障 | 是 |
| `AtomicBool` | 编译器保证 | 否 |
| `OnceLock<T>` / `lazy_static` | 是 | 否（复杂初始化） |

> 加分点：除非写 OS 底层/FFI 边界，一律选 `AtomicBool`（x86 上就是一条指令，性能无感知差异）。

## 8. 异步编程（Future / async / await / Pin）

**Q：async/await 的原理？为什么 Future 需要 Pin？**

- `async fn` 被编译器改写为返回**匿名状态机结构体**（实现了 `Future`）；函数体里每个 `.await` 是状态机的一个**挂起点**；
- Future 是惰性的：创建后什么都不做，被 `poll` 才推进；返回 `Poll::Pending` 时注册 waker，事件就绪后 `wake` 重新调度。

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- **Pin 存在的意义**：async 状态机可能**自引用**（某个状态持有指向自身其他字段的引用）。如果结构体被移动，内部引用会失效成悬垂指针。`Pin` 保证 Future 在 poll 之后不会被移动（`!Unpin` 类型）。

> 面试加分项：能一段话讲清"为什么 Future 需要 Pin"基本属于技术面加分项。
>
> 版本脉络：Rust 1.36（2019.07）Future trait 进标准库 → **1.39（2019.11）** async/await 语法稳定 → 1.75（2023.12）trait 中支持 async fn。

## 9. Tokio 运行时

- **多线程调度器**（默认 work-stealing）vs **单线程调度器**（`new_current_thread`，`spawn_local`）；
- 工作窃取：空闲 worker 从繁忙 worker 偷任务，充分利用 CPU；
- `spawn` vs `spawn_local`：前者 Send 任务可跨线程调度，后者只能当前线程运行（`!Send`）；
- tokio 下 `gdb bt` 常只看到 `epoll_wait`，因为业务逻辑在异步任务里调度 → 用 **tokio-console** 看任务级状态（poll 次数、耗时、waker、await 点）。

---

# 三、工程化

## 1. Cargo.toml 与 Cargo.lock

- **Cargo.toml**：项目元数据 + 依赖声明 + 构建配置（profile），TOML 格式；
- **Cargo.lock**：锁定**精确版本**（含传递依赖），保证任何人构建都用相同版本，避免依赖冲突。**应用**项目提交 lock，**库**项目不提交。

## 2. crate、module 与包管理

- **crate**：编译单元（二进制或库），crates.io 是中央包注册表；
- **module**：crate 内部的代码组织单位（`mod` 关键字），可嵌套、可跨文件；
- 一个项目（package）可含多个 crate，一个 crate 含多个 module。

## 3. 条件编译与 build.rs

- `#[cfg]` 属性按条件选择性编译（平台、feature、debug_assertions 等）；
- **build.rs** 构建脚本：编译前执行，用于生成代码、设置环境变量、编译外部依赖（如 C 库）、配置构建选项。
  - 实战用法：注入 git commit / 构建时间到二进制，方便线上定位版本。

## 4. 标准库、I/O 与网络

- 标准库随安装打包：I/O、集合、线程、网络等核心功能；`std::io` 处理输入输出；
- `std::net`：TCP/UDP socket、listener、IPv4/IPv6；
- HTTP 生态：`hyper`（底层）、`reqwest`（客户端）、`axum`/`actix-web`/`rocket`/`warp`（框架）。

## 5. Web 开发与数据库

- Web：异步非阻塞（高并发）、内存安全（无空指针/泄漏）、跨平台编译（单静态二进制部署）；
- 数据库：
  - **sqlx**：编译期检查 SQL、纯异步、支持 PostgreSQL/MySQL/SQLite；
  - **Diesel**：类型安全 ORM + 查询构建器；
  - **tokio-postgres**：PostgreSQL 异步驱动。

## 6. Unsafe 代码管理

Rust 允许但严格封装 unsafe：

- 原始指针（`*const T` / `*mut T`）算术与转换；
- `unsafe fn` / `unsafe` 块执行编译器无法保证安全的操作；
- unsafe 块应包含在**安全抽象**内部，对外暴露安全 API，安全性由封装者负责。

## 7. 文档（Rustdoc）

`///` 文档注释 + `cargo doc` 生成 HTML；支持代码示例自动测试（doc test）。标准库文档即 Rustdoc 产物。

---

# 四、线上 Debug 实战

> 核心思路：**平时埋好钩子，出事时能拿到足够信息**。
> 黄金三件套：**现场（crash/卡死/性能） + 符号（symbols） + 工具（gdb/perf 等）**。
> 线上 Debug 的能力，取决于发布前是否埋好了「符号 + 现场 + 日志」三件事。

## 1. 核心思路

```
现场（crash/卡死/性能）  +  符号（symbols）  +  工具（gdb/perf 等）
```

原则：

1. **先保现场，再谈分析**（别急着重启，先 dump / 抓栈）；
2. 最小化改动：能不停机抓信息就先抓；
3. 复现路径优先：日志回放/压测复现优于纯静态分析。

## 2. 发布前准备：让 release 二进制可调试

```toml
[profile.release]
debug = 1          # 生成 debug 信息（1=行号，2=完整）。关键！
debug-assertions = false
overflow-checks = false
lto = "thin"       # fat lto 内联更激进，栈更难读
codegen-units = 1
panic = "abort"    # 注意：abort 无法 unwind/catch
strip = false      # 不要 strip！
```

**分离符号的标准做法**（体积与可调试性兼得）：

```bash
objcopy --only-keep-debug myapp myapp.debug          # 1. 保留符号到独立文件
objcopy --strip-debug --strip-unneeded myapp myapp.stripped  # 2. 部署 stripped 版
objcopy --add-gnu-debuglink=myapp.debug myapp.stripped      # 3. 记录关联
```

- 部署 stripped 版，符号存制品库/S3，**版本号对齐**；
- **build.rs 注入 git commit + build time**，二进制自报版本；
- 折中权衡：`debug = 1` 会让二进制体积增 2~4 倍，按需选择。

## 3. 拿到现场：backtrace / core dump / gcore

**RUST_BACKTRACE**：

```bash
export RUST_BACKTRACE=1       # 简略栈
export RUST_BACKTRACE=full    # 完整栈
```

- 加 `panic = "abort"` + 全局 panic hook（`std::panic::set_hook` / `human-panic`），把 panic 栈写到日志并上报（sentry）；
- 结构性问题优先 `tracing` + `tracing-opentelemetry`（request_id、关键状态）。

**core dump**：

```bash
ulimit -c unlimited
echo '/tmp/core.%e.%p.%t' | sudo tee /proc/sys/kernel/core_pattern
# 容器坑：容器内 ulimit 常为 0，需 docker run --ulimit core=-1；core_pattern 写管道时容器拿不到
```

**进程卡死不重启抓现场**：

```bash
gdb -p <PID> -batch -ex "thread apply all bt" > /tmp/bt.txt
gdb -p <PID> -batch -ex "info proc mappings" > /tmp/maps.txt
gcore -o /tmp/core.myapp <PID>   # STW 短暂停顿后恢复，抓"正在卡死"现场
```

## 4. 符号化：地址翻译成函数名

无符号的栈全是 `0x55f2a1b3c4d0 in ?? ()`。翻译工具：

```bash
addr2line -f -C -e myapp.debug 0x1b3c4d0      # 地址 -> 文件:行号
llvm-symbolizer --obj=myapp.debug 0x1b3c4d0   # 支持内联展开，效果更好
echo "_ZN3app3foo17h1234567890abcdefE" | rustfilt   # => app::foo
```

**PIE 基址问题**：现代二进制默认 PIE，栈里是运行时地址，需减去基址（`/proc/<PID>/maps` 第一个可执行段）得相对地址再符号化。

## 5. gdb / lldb / rust-gdb 分析

```bash
gdb ./myapp.debug ./core.12345
(gdb) bt / bt full / thread apply all bt
(gdb) info threads / info locals / info args
(gdb) frame 3 / p *ptr / x/16x 0xADDR
```

- Rust 类型在 gdb 里难看，用 **`rustup component add rust-gdb`** 的 `rust-gdb`（内置 pretty-printer，直接打印 String/Vec/HashMap）；
- 源码路径不一致：`set substitute-path /build/path /local/path`；
- **macOS 优先 lldb**（系统自带、免签名）：`lldb -p <pid>`、`bt`，免掉 gdb 签名的繁琐流程。

## 6. 性能排查：perf / 火焰图 / bpftrace

```bash
perf record -F 99 -g -p <PID> -- sleep 30   # 采样 30s（dwarf 调用图更准但开销大）
perf script | rustfilt | \
  FlameGraph/stackcollapse-perf.pl | FlameGraph/flamegraph.pl > flame.svg
bpftrace -e 'uprobe:/path/myapp:myapp::hot_fn { @[ustack] = count(); }'
```

| 场景 | 工具 |
|---|---|
| CPU 高 | `perf top -p <pid>`、`cargo flamegraph` |
| 内存泄漏/膨胀 | `heaptrack`、jemalloc malloc 采样 |
| 线程卡死/死锁 | `gdb thread apply all bt` 看哪些线程停在 lock 上 |
| 异步任务卡住 | tokio-console（需 `--cfg tokio_unstable` 编译） |

## 7. Tokio 异步栈追踪与 tokio-console

tokio 下 `bt` 只看到 `epoll_wait`——业务在异步任务里。**tokio-console** 看任务级：

```toml
[dependencies]
console-subscriber = { version = "0.5", optional = true }
tokio = { version = "1", features = ["full", "tracing"] }

[features]
tokio-console = ["dep:console-subscriber"]
```

```rust
#[cfg(feature = "tokio-console")]
console_subscriber::init();   // 必须在 main 第一行，runtime 启动前
```

```bash
RUSTFLAGS="--cfg tokio_unstable" cargo run --features tokio-console
cargo install tokio-console
tokio-console    # 默认连 127.0.0.1:6669，t=任务 r=资源 a=异步操作 Enter=详情
```

能看到**每个任务的 poll 状态、耗时、waker、await 点**，直接定位"哪个任务卡住了"。

> ⚠️ **仅本地调试，线上勿开**：每个 task 的创建/poll/waker 全被记录，有性能开销、内存增长、6669 端口信息泄露风险。release 不带 feature 即零开销。

## 8. 内存问题：泄漏 / 越界 / 悬垂

```bash
cat /proc/<PID>/status | grep VmRSS    # 观察 RSS 趋势
heaptrack ./myapp                       # 追踪分配点
valgrind --leak-check=full ./myapp      # 快速验证
```

Rust 常见泄漏来源：

- `Rc`/`Arc` **循环引用**（用 `Weak` 打破）；
- 全局 `static` 无上限缓存容器；
- `spawn` 任务/线程未回收；channel 消费慢于生产；
- `mem::forget` / `Box::leak`。

**越界/UB/悬垂**（unsafe 相关）：预发环境用 sanitizer 复现——

```bash
RUSTFLAGS="-Zsanitizer=address" cargo +nightly build -Zbuild-std --target x86_64-unknown-linux-gnu
```

ASan（越界/悬垂）、TSan（数据竞争）、MSan（未初始化内存）。

## 9. 死锁与卡死排查

```bash
gdb -p <PID> -batch -ex "thread apply all bt" -ex detach   # 找互相等待的线程对
```

- 找**互相等待**模式：A 持锁 1 等锁 2，B 持锁 2 等锁 1；
- `parking_lot` 开 `deadlock_detection`，定时 `check_deadlock()` 自动检测；
- **异步死锁**：`async` 里用 `std::sync::Mutex` 跨 `.await` 持锁；单线程 runtime 跑阻塞任务卡住整个 runtime；用 tokio-console 看任务长时间 poll 不返回。

## 10. 日志与可观测性

| 手段 | 工具 | 用途 |
|---|---|---|
| 结构化日志 | `tracing` + `tracing-subscriber`(JSON) | 时序还原、检索 |
| 指标 | `metrics` + Prometheus | 趋势、突刺 |
| 分布式追踪 | `tracing` + OpenTelemetry | 跨服务链路 |
| 错误上报 | `sentry` | 崩溃聚合 + 符号化栈 |
| 动态日志级别 | `tracing_subscriber::reload` | 不重启调高某模块日志 |

```rust
#[instrument]   // 自动记录入参、耗时、嵌套 span
async fn handle_order(order_id: u64) -> Result<(), Error> { ... }
```

## 11. 常见问题速查表 + 发布 Checklist

| 现象 | 可能原因 | 排查手段 |
|---|---|---|
| 进程崩溃无栈 | panic=abort + 无 core | 开 core dump + RUST_BACKTRACE |
| 栈全是 `??` | 二进制被 strip | 对应 debug 符号 + addr2line |
| 栈看不到业务函数 | LTO 内联 | debug=1 + llvm-symbolizer |
| CPU 打满 | 死循环 / 算法问题 | perf top + 火焰图 |
| RSS 持续增长 | 内存泄漏 | heaptrack / VmRSS |
| 请求卡住不返回 | 死锁 / await 未唤醒 | gdb bt / tokio-console |
| 随机崩溃 | UB / unsafe 越界 | 预发跑 ASan |
| 异步任务不推进 | 阻塞 runtime | tokio-console |

**发布 Checklist**（逐条确认，省掉 80% 排查痛苦）：

- [ ] release profile 设 `debug = 1`（或外置符号）
- [ ] 每次发布**归档对应 debug 符号**，版本可查
- [ ] 二进制内置 git commit + build time（build.rs）
- [ ] 线上开 core dump（ulimit + core_pattern 可写，容器场景确认能落盘）
- [ ] 启动脚本 `export RUST_BACKTRACE=1`
- [ ] panic hook 崩溃自动上报（sentry）
- [ ] 结构化日志 + 可动态调级
- [ ] 基础指标（RSS/CPU/QPS/延迟）
- [ ] 出事 SOP：**先抓栈/gcore，再考虑重启**

**典型线上排查流程**：

```
告警 → 确认版本(commit hash) → 保现场(core/gdb/perf) → 取对应符号
→ 符号化(gdb bt full / perf+rustfilt / tokio-console) → 定位根因
→ 写复现用例 → 修复 → 预发 ASan 回归 → 复盘更新 Checklist
```

---

# 五、面试答题建议

1. **先给定义，再讲机制，最后上例子**。例如可变引用：定义 → 唯一性规则 → 一段代码。
2. **主动对比**：Copy vs Clone、Rc vs Arc、impl Trait vs dyn Trait、`std::sync::Mutex` vs `tokio::sync::Mutex`——对比是面试官最想听的。
3. **说清编译期 vs 运行时**：Rust 的大部分保证在编译期（零成本抽象），死锁/竞争条件是运行时要靠工具排查的。
4. **异步必背三板斧**：Future 惰性 + poll/wake 模型；async 是状态机语法糖；Pin 防自引用结构体被移动。
5. **工程题讲实战**：线上 Debug 题直接背"黄金三件套"和发布 Checklist，有真实案例更好。
6. **承认边界**：unsafe、宏的复杂度、`static mut` 的坑——知道什么不该做，比知道能做什么更加分。
