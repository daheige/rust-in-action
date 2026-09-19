# Rust 面试指南

- 基于本仓库 `interview-qa` 整理，覆盖 **语言基础、并发与异步、工程化、线上 Debug 实战** 四大板块。
- 适用场景：Rust 初中高级岗位技术面试准备 / 技术分享 / 直播讲解。
- 文档制定者：大黑哥 《Rust实战项目开发》书籍作者

---

## 目录

- [Rust 面试指南](#rust-面试指南)
  - [目录](#目录)
- [一、语言基础](#一语言基础)
  - [1. 所有权与内存安全](#1-所有权与内存安全)
  - [2. 数据类型（标量类型与复合类型）](#2-数据类型标量类型与复合类型)
  - [3. static 与 const 常量](#3-static-与-const-常量)
  - [4. 引用与借用（含可变引用）](#4-引用与借用含可变引用)
  - [5. 生命周期](#5-生命周期)
  - [6. Copy 与 Clone](#6-copy-与-clone)
  - [7. 智能指针](#7-智能指针)
  - [8. 结构体与枚举](#8-结构体与枚举)
  - [9. Trait 系统（泛型 vs 特征对象）](#9-trait-系统泛型-vs-特征对象)
  - [10. 闭包（Fn / FnMut / FnOnce）](#10-闭包fn--fnmut--fnonce)
  - [11. 错误处理（Option / Result / panic）](#11-错误处理option--result--panic)
  - [12. 宏（声明宏 vs 过程宏）](#12-宏声明宏-vs-过程宏)
  - [13. 其他高频基础题](#13-其他高频基础题)
  - [14. 类型转换（From / Into / TryFrom）](#14-类型转换from--into--tryfrom)
  - [15. Cow（写时克隆）](#15-cow写时克隆)
  - [16. 泛型（Generics）与单态化](#16-泛型generics与单态化)
  - [17. 字符串（String / \&str / 字面量）](#17-字符串string--str--字面量)
- [二、并发与异步](#二并发与异步)
  - [1. 并发模型](#1-并发模型)
  - [2. Send 与 Sync](#2-send-与-sync)
  - [3. 线程创建：thread::spawn 签名解析](#3-线程创建threadspawn-签名解析)
  - [4. 线程栈大小与自定义线程](#4-线程栈大小与自定义线程)
  - [5. move 闭包与跨线程所有权转移](#5-move-闭包与跨线程所有权转移)
  - [6. Mutex / RwLock 与 Arc](#6-mutex--rwlock-与-arc)
  - [7. 共享数据与资源保护](#7-共享数据与资源保护)
  - [8. 死锁与竞争条件](#8-死锁与竞争条件)
  - [9. Channel 与 Select](#9-channel-与-select)
  - [10. 全局变量](#10-全局变量)
  - [11. 异步编程（Future / async / await / Pin / Waker）](#11-异步编程future--async--await--pin--waker)
  - [12. Tokio 运行时](#12-tokio-运行时)
- [三、工程化](#三工程化)
  - [1. Cargo.toml 与 Cargo.lock](#1-cargotoml-与-cargolock)
  - [2. crate、module 与包管理](#2-cratemodule-与包管理)
  - [3. 模块化编程实践（mod 机制 / 文件目录组织 / pub use）](#3-模块化编程实践mod-机制--文件目录组织--pub-use)
  - [4. Cargo 工具链与常用命令](#4-cargo-工具链与常用命令)
  - [5. rustup 工具与交叉编译](#5-rustup-工具与交叉编译)
  - [6. 条件编译与 build.rs](#6-条件编译与-buildrs)
  - [7. 标准库、I/O 与网络](#7-标准库io-与网络)
  - [8. Web 开发与数据库](#8-web-开发与数据库)
  - [9. Unsafe 代码管理与 FFI](#9-unsafe-代码管理与-ffi)
  - [10. 日志组件（log / env\_logger）](#10-日志组件log--env_logger)
  - [11. 配置读取与序列化（serde 实践）](#11-配置读取与序列化serde-实践)
  - [12. 文档（Rustdoc）](#12-文档rustdoc)
  - [13. 基础设施组件库封装（hera 实践）](#13-基础设施组件库封装hera-实践)
  - [14. 项目实战：Web 开发、gRPC 微服务与 BFF 网关](#14-项目实战web-开发grpc-微服务与-bff-网关)
- [四、线上 Debug 实战](#四线上-debug-实战)
  - [1. 核心思路](#1-核心思路)
  - [2. 发布前准备：让 release 二进制可调试](#2-发布前准备让-release-二进制可调试)
  - [3. 拿到现场：backtrace / core dump / gcore](#3-拿到现场backtrace--core-dump--gcore)
  - [4. 符号化：地址翻译成函数名](#4-符号化地址翻译成函数名)
  - [5. gdb / lldb / rust-gdb 分析](#5-gdb--lldb--rust-gdb-分析)
  - [6. 性能排查：perf / 火焰图 / bpftrace](#6-性能排查perf--火焰图--bpftrace)
  - [7. Tokio 异步栈追踪与 tokio-console](#7-tokio-异步栈追踪与-tokio-console)
  - [8. 内存问题：泄漏 / 越界 / 悬垂](#8-内存问题泄漏--越界--悬垂)
  - [9. 死锁与卡死排查](#9-死锁与卡死排查)
  - [10. 日志与可观测性](#10-日志与可观测性)
  - [11. 常见问题速查表 + 发布 Checklist](#11-常见问题速查表--发布-checklist)
- [五、面试答题建议](#五面试答题建议)

---

# 一、语言基础

## 1. 所有权与内存安全

**Q：Rust 如何保证内存安全？**

Rust 通过两种主要方法确保内存安全：

- **严格类型系统**：编译时检查类型，在代码运行前捕获错误，如访问已移动或已借用的值。
- **所有权和借用系统**：
  - 每个值有且只有一个所有者，所有者离开作用域时值被自动释放（RAII）；
  - 借用允许函数临时使用值而不取得所有权，由借用检查器（Borrow Checker）在编译时强制执行规则：
    - 同一时刻最多一个可变引用 `&mut T`；
    - 或者任意多个不可变引用 `&T`；
    - 两者不可同时存在。

> 💡 加分点：对比 C/C++（半手动内存管理）和 Java/Go/Python（GC），Rust 在**编译时**验证引用，零运行时开销。
> 借用检查器同时保障了并发安全：数据竞争的本质是"同时存在多个可变引用"，所有权规则从根上杜绝了这一点。

**Q：内存安全具体指什么？Rust 靠什么机制保证？（高频基础题）**

内存安全 = 程序运行时**不会发生与内存相关的未定义行为（UB）**。传统语言里常见的五类内存错误，Rust 在编译期或类型层面根除：

| 内存错误 | 典型场景 | Rust 的防线 |
|---|---|---|
| 悬垂指针 / use-after-free | 释放后再访问 | 生命周期 + 所有权（编译期拒绝） |
| 双重释放 double free | 同一块内存释放两次 | 所有权唯一（移动即失效） |
| 缓冲区越界 | 数组/切片越界读写 | 切片带长度 + 边界检查（越界 panic 而非 UB） |
| 空指针解引用 | 忘判空就访问 | `Option<T>` 强制处理空值 |
| 数据竞争 | 多线程同时改同一数据 | `Send`/`Sync` + 借用规则（编译期拒绝） |

三大机制分工协作：

1. **所有权（RAII）**：值有唯一所有者，离开作用域自动释放 → 杜绝 double free、内存泄漏；
2. **借用检查器**：`&mut` 唯一、`&` 可并存 → 编译期杜绝数据竞争与悬垂借用；
3. **生命周期**：追踪引用有效期 → 杜绝悬垂指针。

内存分区（与 C 一致，但管理方式不同）：

| 区域 | 存放内容 | 管理方式 |
|---|---|---|
| 栈 stack | 局部变量、定长数组、函数调用帧 | 编译期确定大小，自动入栈/出栈 |
| 堆 heap | `Box`/`String`/`Vec` 等动态数据 | 所有权 + RAII 自动释放 |
| 静态区 | `static`/`const`/字符串字面量 | 编译期写入，程序整个生命周期存在 |

> 💡 加分点：内存安全是**编译期保证**（零运行时开销），不是 GC 的运行时回收、也不是 C/C++ 的"靠人自觉"。`unsafe` 是这条铁律唯一的"逃生门"——safe Rust 范围内内存安全由编译器兜底，跨入 unsafe 则责任转交给开发者（详见工程化篇第 9 节）。

## 2. 数据类型（标量类型与复合类型）

**Q：Rust 有哪些基本数据类型（标量类型）？**

| 类型 | 说明 | 默认/要点 |
|---|---|---|
| 整数 | `i8/i16/i32/i64/i128/isize`（有符号）、`u8/u16/u32/u64/u128/usize`（无符号） | 字面量默认 `i32`；`usize`/`isize` 位宽与平台一致，用于索引/长度 |
| 浮点 | `f32` / `f64` | 字面量默认 `f64`；遵循 IEEE-754 |
| 布尔 | `bool` | `true`/`false`，占 1 字节 |
| 字符 | `char` | **4 字节** Unicode 标量值（不是 1 字节 ASCII！） |
| 单元 | `()` | 空元组，表示"无值" |

```rust
let x = 42;        // i32（整数默认）
let y: u8 = 255;   // 显式标注；超范围（如 256）编译报错
let z = 3.14;      // f64（浮点默认）
let f: f32 = 2.0;  // 显式 f32
let b = true;      // bool
let c = '中';      // char：Unicode 标量值，4 字节
let u = ();        // 单元类型
```

> 📌 面试要点：
> - **整数溢出**：debug 模式溢出会 panic，release 默认**回绕（wrapping）**；用 `wrapping_add`/`checked_add`/`saturating_add` 显式表达意图；
> - **`char` ≠ `u8`**：`char` 是 4 字节 Unicode 标量值，一个字节是 `u8`——高频混淆点；
> - **`usize` 用于索引**：`Vec` 索引、`len()` 返回的都是 `usize`，32/64 位平台自适应。

**Q：什么是复合类型？元组、数组、切片怎么用？**

| 类型 | 语法 | 特点 |
|---|---|---|
| 元组 tuple | `(T1, T2, ...)` | 不同类型值的集合，定长，可解构 |
| 数组 array | `[T; N]` | **同类型、定长**，栈上分配 |
| 切片 slice | `&[T]` | 动态长度视图（指针 + 长度），借用连续内存 |
| 结构体 struct | `struct S { ... }` | 具名字段聚合（见第 8 节） |
| 枚举 enum | `enum E { ... }` | 有限命名变体（见第 8 节） |

```rust
// 元组：不同类型
let t = (10, "hello", true);
let (a, b, c) = t;      // 解构
let first = t.0;        // 按位置访问

// 数组：同类型定长，栈上
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 5];     // [0,0,0,0,0] 简写

// 切片：&[T] 视图，不拥有数据
let s: &[i32] = &arr[1..3];  // [2, 3]
```

> 📌 数组 vs 元组 vs 切片：数组**同类型定长**（栈上连续）、元组**异构定长**、切片**不定长视图**（借用）。`[T; N]` 的 `N` 是编译期常量；要可增长用 `Vec<T>`。

**Q：类型别名和类型推断是什么？**

```rust
type UserId = u64;               // 类型别名：给类型起名，不产生新类型
let id: UserId = 1;              // 只是 u64 的别名

// 类型推断：编译器从上下文/后续使用推断类型，多数场景可省略标注
let v = vec![1, 2, 3];           // Vec<i32> 推断
let s = "hello".to_string();     // String 推断
```

> 💡 加分点：`type` 只是**别名**（与原类型完全等价）；要"强类型区分"需用**新类型模式**（`struct UserId(u64)`，见第 14 节 From/Into 的 newtype）。

## 3. static 与 const 常量

**Q：const 和 static 有什么区别？（高频基础题）**

| | `const` | `static` |
|---|---|---|
| 本质 | 编译期**内联的常量值**，用到处直接替换 | 有**固定内存地址**的全局变量 |
| 地址 | 无固定地址（内联展开） | 有（`&'static` 引用指向固定位置） |
| 类型标注 | 必须显式 | 必须显式 |
| 求值时机 | 编译期常量表达式 | 编译期初始化 |
| 可变性 | 永远不可变 | 默认不可变；`static mut` 可变但 unsafe |
| 典型用途 | 魔法数字、常量配置 | 全局共享数据、全局单例 |

```rust
const MAX_POINTS: u32 = 100_000;       // 编译期内联，用到处替换成字面量
static APP_NAME: &str = "my-app";      // 固定内存地址，整个程序共享
static mut COUNTER: u32 = 0;           // 可变静态，只能 unsafe 访问（不推荐）

fn main() {
    println!("{}", MAX_POINTS);        // 内联成 100000
    let name: &'static str = APP_NAME; // static 的引用是 'static
    // COUNTER += 1;                   // ❌ 静态可变必须 unsafe
}
```

**Q：const 还有哪些用法？（资深岗加分）**

1. **关联常量**：`impl` 块内定义的常量，属于类型而非实例：

```rust
struct Circle { radius: f64 }
impl Circle {
    const PI: f64 = 3.14159;           // 关联常量
    fn area(&self) -> f64 { Self::PI * self.radius * self.radius }
}
```

2. **const fn（编译期函数）**：结果可作为 `const`、数组长度、泛型常量参数：

```rust
const fn square(n: u32) -> u32 { n * n }
const AREA: u32 = square(5);           // 编译期算出 25
let arr = [0u8; square(4)];            // 数组长度也可用 const fn
```

> 📌 面试要点：
> - **核心区别一句话**：`const` 是**值内联**（无地址），`static` 是**全局变量**（有地址、可取 `&'static` 引用）；
> - `static` 跨线程共享，要求类型 `Sync`（否则编译报错）；可变全局不用 `static mut`，改用 `Atomic*`/`OnceLock`/`Mutex`（见并发篇第 10 节）；
> - `static mut` 是"危险区"：全局可写 = 数据竞争温床，只能 `unsafe` 访问，现代 Rust 基本弃用。

## 4. 引用与借用（含可变引用）

**Q：Rust 中的可变引用是什么？**

可变引用（可变借用）允许在不转移所有权的情况下修改值，用 `&mut` 创建。编译器强制：

- **唯一性**：同一作用域内，一个值只能有一个 `&mut`，且不能有其他 `&` 或 `&mut` 指向它；
- **不可变引用可并存**：有 `&T` 时可以有任意多个 `&T`，但不能有任何 `&mut`；
- **不自动提升为所有权**：`&mut` 不是 Copy 类型，不能通过赋值转移所有权。

> 📌 引用本质是"借用"，父函数保留所有权，子函数获得使用权，无需拷贝。

## 5. 生命周期

**Q：什么是生命周期？**

生命周期描述**引用有效的时间范围**，是类型系统的功能，用于表达不同值及其引用之间的关系。标注形如 `'a`。

- 生命周期**防止悬垂引用**：编译器确保引用不会比它指向的数据活得更久；
- 生命周期是**编译期检查**，不影响生成的机器码（零运行时开销）。

**本质认知（资深岗加分）**：

- 生命周期**纯粹是编译期构造**——它跟踪"引用从哪来、是否比借用的值活得长"，是**编译器推断引用有效性用的工具**，不是开发者运行时要用到的机制；
- 编译器能推断就自动推断（elision），**推断不了才需要开发者显式标注**辅助编译器；
- 显式标注的场景就三处：**函数签名、结构体（含引用字段）、impl 块**。

> 📚 权威参考：RFC 1414（lifetime elision）、RFC 2115（argument lifetimes）、RFC 556（raw lifetime）。

**Q：悬垂引用为什么会被编译器拒绝？**

```rust
let r;                    // 声明 r，尚未初始化
let x = 5;
r = &x;                   // ✅ x 比 r 活得久，合法
println!("r: {r}");

let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}   // string2 在此处被 drop
// println!("The longest string is {result}"); // ❌ 编译错误：result 可能指向已释放的 string2
```

借用检查器在编译期追踪每个引用的有效范围，任何"引用比数据活得久"的代码都过不了编译——这是 Rust 杜绝 use-after-free 的核心机制。

**Q：函数签名中的生命周期标注怎么理解？**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

含义：返回的引用 `'a` 的有效期 = 两个入参中**较短**的那个。调用时 `'a` 被实例化为 x、y 生命周期的交集：

```rust
let string1 = String::from("abcd");
let string2 = "xyz";                       // 'static
let result = longest(string1.as_str(), string2); // 'a = string1 的生命周期
println!("The longest string is {result}");    // ✅ string1 仍存活
```

> 📌 标注**不改变**任何生命周期，只是向编译器声明约束关系。返回值若是引用，其生命周期必须能由入参推导出来（否则只能返回 'static 或泄漏）。

**Q：什么是输入型/输出型生命周期？（核心规则）**

- **输入型生命周期**：函数**参数**上的生命周期（引用入参）；
- **输出型生命周期**：函数**返回值**上的生命周期。

**黄金法则：任何输出型生命周期都源自输入型生命周期**——返回值引用必然来自某个入参（或 'static），不可能凭空产生一个与入参无关的引用（那必是悬垂引用）。输出生命周期的时长**只能小于或等于**对应的输入生命周期。

```rust
// ❌ 两个输入引用，一个输出引用——编译器不知道返回值跟哪个入参走
fn foo(x: &str, y: &str) -> &str {
    // expected named lifetime parameter
    x
}

// ✅ 显式声明：输出 'a 与两个输入的交集绑定
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str { x }

// ✅ 只有一个输入引用时，elision 规则自动推导，不用写
fn get_one(x: &u8) -> &u8 { x }
```

这条法则也是编译器拒绝 `fn foo(x: &str, y: &str) -> &str` 的原因——"返回值的生命周期并不明显，需要我们的帮助"。

**Q：结构体持引用时为什么要标注生命周期？**

结构体实例可能比局部变量活得更久，编译器必须确认**结构体活多久、里面的引用就最多活多久**：

```rust
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,   // 引用字段必须标注：part 不能比 'a 指向的数据活得更久
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let i = ImportantExcerpt { part: first_sentence }; // 'a = novel 的借用期
```

`impl` 块同样要声明 `'a`——**先声明后使用**；方法本身不引用 `'a` 数据时标注可省略：

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }                  // 返回值与 'a 无关，标注可省
    fn part(&self) -> &'a str { self.part }       // 返回引用字段时必须写 'a
}

#[derive(Debug)]
struct NumberInfo<'a> { n: &'a i64 }

impl<'a> NumberInfo<'a> {
    fn new(n: &'a i64) -> Self { Self { n } }
    fn get_num(&self) -> &'a i64 { self.n }
    fn set_num(&mut self, number: &'a i64) { self.n = number } // 换入的引用也必须满足 'a
}
```

**Q：多个生命周期参数怎么管理？什么是 outlives 约束？**

复杂结构体可声明多个生命周期 + 类型参数混用，约束关系用 `where` 从句表达：

```rust
// 解码器：schema 与 reader 来自不同的借用源，各自独立的生命周期
struct Decoder<'a, 'b, S, R> {
    schema: &'a S,
    reader: &'b R,
}

impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
where
    S: std::fmt::Display,
    R: std::fmt::Display,
    'a: 'b,          // outlives 约束：'a 至少活得跟 'b 一样长
{
    fn new(schema: &'a S, reader: &'b R) -> Self { Self { schema, reader } }
    fn get_schema(&self) -> &'a S { self.schema }
}
```

- `'a: 'b` 读作"**'a outlives 'b**"——'a 的生命周期覆盖 'b，保证 'b 的东西消亡时 'a 还活着；
- 典型用途：结构体同时持有"长寿命的配置/元数据"和"短寿命的输入数据"时，声明两者不消亡顺序颠倒。

**Q：'static 到底是什么？（易混淆考点）**

三层含义要分清：

1. **`&'static T`**：一个能活到程序结束的引用。字符串字面量天然是 `&'static str`（编译期进二进制）；`static` 变量的引用也是；
2. **`T: 'static`（bound）**：类型**不含任何非 'static 引用**——值可以持有到程序结束。这是泛型约束里的常见写法（`fn configure_logger<T: Send + 'static + Debug>(t: T)`），**不要求值真的活到程序结束**，`String`/`Vec` 这类 owned 类型天然满足；
3. **`static` 变量**：编译期创建、整个程序期间存在的内存资源，必须显式标注类型，**永远不会被 drop**：

```rust
static PI: f64 = 3.1415;              // 全局静态变量，必须指定类型

fn main() {
    let msg: &'static str = "Hello";  // 字面量：'static
    let p: &'static f64 = &PI;

    // static mut：可被修改的静态变量——全局可读写 = 数据竞争温床，只能 unsafe 访问
    static mut SECRET: &'static str = "swordfish";
    unsafe {
        SECRET = "abcddd";            // 只能用 unsafe 块修改
    }
}
```

> ⚠️ 运行时也能创建 'static 资源：`Box::leak` 把堆内存"泄漏"成 `&'static mut T`、`once_cell`/`LazyLock` 的全局实例——符合"内存无限延续到程序结束"的定义。**static mut 是危险区**：多线程全局可改，现代 Rust 推荐 `Atomic*` / `OnceLock`（见并发篇第 10 节）。

**Q：什么是生命周期省略规则（elision）？**

编译器对**函数签名**有三条默认推导规则，满足就不用手写：

1. 每个引用型入参各分配一个独立生命周期（`'a, 'b, ...`）；
2. 只有一个入参引用时，返回值生命周期自动等于它；
3. 有 `&self` / `&mut self` 时，返回值生命周期自动等于 self。

超过规则覆盖范围（如 `longest` 有两个入参引用且返回其中之一）就必须显式标注。

**Q：泛型和生命周期如何组合使用？**

生命周期与类型参数完全正交，可同时声明、独立约束：

```rust
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,   // 类型参数走 trait bound，引用参数走生命周期
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

## 6. Copy 与 Clone

**Q：什么是 move（移动）语义？**

Rust 的赋值默认不是拷贝而是**所有权转移**：

```rust
let s1 = String::from("hello");
let s2 = s1;              // s1 的所有权移动到 s2，s1 失效（浅拷贝指针后禁用旧名，避免二次释放）
// println!("{s1}");      // ❌ 编译错误：value borrowed after move
println!("{s2}");

let x = 5;
let y = x;                // i32 实现 Copy，按位拷贝，x 仍可用
println!("{x} {y}");
```

函数传参同理：String 传入函数后所有权被消费（函数结束 drop）；i32 等 Copy 类型传入后仍可继续用。想让调用方继续用堆类型，传引用 `&String`/`&str`，或把值返回去（`fn calculate_length(s: String) -> (String, usize)`）。

**Q：Copy 和 Clone 特征有什么区别？**

| | Copy | Clone |
|---|---|---|
| 语义 | 按位拷贝，拷贝后两值独立 | 显式 `clone()`，可自定义拷贝逻辑 |
| 成本 | 极低（数字、布尔、指针等） | 可能很高（String、Vec 等堆分配类型） |
| 触发方式 | 赋值即拷贝 | 必须显式调用 `.clone()` |
| 关系 | Copy 要求先实现 Clone | Clone 不要求 Copy |

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // 显式深拷贝：堆数据复制一份，s1 仍有效
```

> 📌 哪些类型是 Copy：一切可安全按位复制的类型——整型/浮点/布尔/字符、定长数组 `[T; N]`（T: Copy）、元组（元素全 Copy）。**不允许手动实现 Copy 的类型**：实现了 Drop 的类型（需要清理资源，按位复制会双重释放）。
> Clone 还可以是"语义化拷贝"——`Rc::clone` 只增加引用计数，不复制数据。

## 7. 智能指针

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

> 📌 经典组合：`Arc<Mutex<T>>` 多线程共享可变数据；`Rc<RefCell<T>>` 单线程共享可变数据。

## 8. 结构体与枚举

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

> 💡 加分点：Rust 的 enum 是代数数据类型（ADT），`Option<T>`、`Result<T, E>` 都是 enum，配合 `match` 做穷尽性检查（exhaustiveness checking）。

## 9. Trait 系统（泛型 vs 特征对象）

**Q：trait 是什么？默认方法如何工作？**

trait 定义类型必须实现的方法集合，支持泛型编程与代码复用；一个类型可实现多个 trait。trait 可以带**默认实现**——实现者不重写就用默认版本，重写则覆盖：

```rust
pub trait Summary {
    fn summarize(&self) -> String;                    // 必须实现

    fn more(&self) -> String {                        // 默认实现
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String { /* ... */ }
    fn more(&self) -> String { /* 覆盖默认实现 */ }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String { /* ... */ }       // more 用默认版本
}
```

> 💡 默认实现的方法可以调用 trait 的其他方法（如 `more` 里调 `self.summarize()`），这是模板方法模式的基础。

**Q：`impl Trait` 和 `<T: Trait>` 是一回事吗？**

作为**函数入参**时两者完全等价，都是静态分发：

```rust
pub fn notify(item: &impl Summary) { /* ... */ }   // 语法糖
pub fn notify2<T: Summary>(item: &T) { /* ... */ } // 完全相同的单态化结果
```

差别在能力上：具名类型参数 `T` 可在签名中**多处使用**（如 `fn eq<T: PartialEq>(a: &T, b: &T)` 要求两者同类型），`impl Trait` 每处是独立匿名类型。

**Q：多个 trait bound 怎么写？**

```rust
// 同时要求 Summary + Display，+ 号连接
pub fn notify3<T: Summary + Display>(item: &T) { /* ... */ }

// 约束复杂时用 where 从句，签名更清晰
pub fn notify4<T>(item: &T)
where
    T: Display + Summary,
{ /* ... */ }
```

**Q：impl Trait vs dyn Trait 怎么选？**

- **impl Trait**：编译期**静态分发**（单态化），性能好、零间接开销，但每种类型生成一份代码（体积大）；编译期类型固定。
- **dyn Trait**：运行时通过 **vtable 动态分发**，一层间接调用开销，代码体积小；支持**异构**——运行时决定具体类型。

```rust
fn f(x: impl Drawable) {}        // 静态分发
fn g(x: &dyn Drawable) {}        // 动态分发

// dyn 的典型价值：返回"多种可能类型之一"
fn returns_summarizable() -> Box<dyn Summary> {
    if /* 条件 */ {
        Box::new(NewsArticle { /* ... */ })
    } else {
        Box::new(SocialPost { /* ... */ })
    }
}
```

> 📌 `dyn Trait` 必须放在**指针**后面（`&dyn`、`Box<dyn>`、`Arc<dyn>`），因为具体大小未知，需要间接层 + vtable（数据指针 + 方法表指针）。

**Q：什么是对象安全（object safety）？**

trait 要能用 `dyn` 必须满足，核心是：

- 方法不能返回 `Self`；
- 方法不能有泛型参数；
- 除第一个 `self` 参数外，`Self` 不能出现在其他参数中。

原因：dyn 分发时具体类型已被擦除，`Self` 大小未知、泛型无法单态化。有默认实现的方法（如 `more`）本身满足条件，可通过 `dyn Summary` 调用。

> 📌 这是中高级岗必考点。反例：`Clone`（返回 Self）、`Iterator::collect<B>`（泛型参数）都不是对象安全的。

**Q：孤儿规则对 trait impl 的限制？（关联第 14 节）**

可以为**本地类型实现外部 trait**（如给 `SocialPost` 实现标准库 `Display`），也可以给外部类型实现**本地 trait**；但**外部 trait × 外部类型**不行——这就是孤儿规则，也是 newtype 模式存在的动机之一。

**Q：泛型与 trait 的使用决策：`<T>` / `impl Trait` / `dyn Trait` 到底怎么选？（决策总结）**

把本节前几个 Q 的要点收敛成一张决策表，面试时照着讲即可：

| 需求 | 选择 | 原因 |
|---|---|---|
| 类型**编译期确定**、要零成本 | `<T: Trait>` 泛型 | 静态分发/单态化，无运行时开销 |
| 函数入参只出现一次 | `impl Trait` | `<T: Trait>` 的语法糖，等价 |
| 同一类型要**多处约束**（如两参同型） | `<T: Trait>`（具名） | `impl Trait` 每处是独立匿名类型 |
| 运行时**异构**、存同一容器/返回多态 | `dyn Trait`（`Box<dyn>`/`&dyn`） | vtable 动态分发 |
| 约束复杂（多 bound、关联类型） | `where` 从句 | 可读性 |

```rust
fn same<T: PartialEq>(a: &T, b: &T) -> bool { a == b }  // 两参必须同型 → 具名泛型
fn show(item: impl Display) {}                            // 入参只用一次 → impl Trait
fn get_shape(kind: u8) -> Box<dyn Shape> { /* ... */ }   // 运行时选具体类型 → dyn
```

> 💡 决策口诀：**编译期确定、图性能 → 泛型；运行时异构、图灵活 → dyn；入参糖衣 → impl Trait**。三者本质都围绕"静态分发 vs 动态分发"（单态化零成本 vs vtable 一层间接，见第 16 节）。

## 10. 闭包（Fn / FnMut / FnOnce）

**Q：Rust 中的闭包是什么？三种捕获方式？**

闭包是捕获其创建环境变量的匿名函数，可推断参数和返回类型，可作为值传递。三种 trait 对应三种捕获方式：

| Trait | 捕获方式 | 说明 |
|---|---|---|
| `Fn` | `&T` 不可变借用 | 不修改捕获值，可多次调用 |
| `FnMut` | `&mut T` 可变借用 | 可修改捕获值，可多次调用 |
| `FnOnce` | 按值移动（T） | 消费捕获值，**只能调用一次** |

- 编译器根据闭包体自动推断实现哪个 trait；
- 闭包同时满足"只读"时，可降级使用：实现了 Fn 的闭包也能当 FnMut/FnOnce 用；
- 所有权模型防止 use-after-free 和数据竞争。

**Q：闭包捕获方式在实战中怎么体现？**

```rust
// move 闭包：所有权移入闭包（线程场景的标配，见并发篇第 5 节）
let list = vec![1, 2, 3];
thread::spawn(move || println!("{list:?}")).join().unwrap();
// println!("{list:?}"); // ❌ 所有权已移动到闭包中

// FnMut：修改捕获值。sort_by_key 对每个元素调用一次闭包，故要求 FnMut
let mut count = 0;
list.sort_by_key(|r| { count += 1; r.width });   // ✅ 可变借用 count
println!("sorted in {count} operations");

// 若闭包把捕获的值移出环境（如 push(value)），编译器推断为 FnOnce，
// 不满足 FnMut 约束 → 编译错误。这正是 trait bound 在保护调用方。
```

> 📌 判断口诀：**只读选 Fn，改写选 FnMut，移出选 FnOnce**；API 设计者按"调用几次、是否改环境"选择 bound，`sort_by_key` 要 FnMut、`thread::spawn` 要 FnOnce 都是这个原因。

## 11. 错误处理（Option / Result / panic）

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

> 💡 加分点：`panic = "unwind"` vs `panic = "abort"`：unwind 可捕获（catch_unwind / sentry 上报），abort 体积小但不能捕获。

## 12. 宏（声明宏 vs 过程宏）

**Q：Rust 支持哪两种宏？**

- **声明宏（macro_rules!）**：匹配代码**模式**并替换生成新代码，基于文本/语法匹配；
- **过程宏（proc-macro）**：编译时对**语法树（TokenStream）**操作的 Rust 函数，三种形式：
  - `#[derive(...)]` 派生宏
  - 属性宏 `#[route("/")]`
  - 函数式宏 `sqlx::query!()`

> 💡 宏很强大但复杂难调试，应谨慎使用。面试可举例：serde 的 derive、tokio::select!、vec!。

## 13. 其他高频基础题

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
| 泛型 | `<T>` 类型参数，单态化零成本抽象——深入见[第 16 节](#16-泛型generics与单态化) |
| 条件编译 | `#[cfg(target_os = "linux")]` 等属性，选择性编译代码块 |
| `static` 生命周期 | 数据存活整个程序执行期；`static` 变量全局可访问 |

## 14. 类型转换（From / Into / TryFrom）

**Q：From 和 Into 是什么关系？为什么只实现一个就能用另一个？**

`From<T>` 与 `Into<T>` 互为镜像：`From<T>` 的类型参数是**来源**，`Self` 是目标；`Into<T>` 的类型参数是**目标**，`Self` 是来源。标准库内置两条 blanket impl（全覆盖实现）：

```rust
impl<T, U> From<U> for T where U: Into<T> { ... }
impl<T, U> Into<U> for T where U: From<T> { ... }
```

所以实现 `From<A> for B`，`B: From<A>` 和 `A: Into<B>` 同时自动可用。

```rust
struct Wrapper(String);
impl From<String> for Wrapper {
    fn from(s: String) -> Self { Wrapper(s) }
}

let w1 = Wrapper::from(String::from("a")); // From
let w2: Wrapper = String::from("b").into(); // Into
```

**惯例：只实现 `From`**（给目标类型添加一个构造来源，更符合直觉），调用方用 `.into()` 或 `T::from(x)` 消费。

**Q：泛型函数中怎么用 Into？（最常见的 API 设计模式）**

函数入参用 `impl Into<String>`，让调用方少写转换，天然接受 `&str`/`String`/`Cow` 等多种类型：

```rust
fn set_title<S: Into<String>>(title: S) { ... }
fn set_title2(title: impl Into<String>) { ... }  // 等价简洁写法

set_title("hello");                          // &str
set_title(String::from("hi"));               // String
std::thread::Builder::new().name("worker".into()).spawn(...);
```

**Q：`?` 运算符是怎么做错误转换的？**

`?` 在返回 `Result<T, E>` 的函数中遇到 `Result<T, OtherErr>` 时，自动调用 `From::from` 把错误转成 `E`——这是自定义错误类型 + `thiserror` 的 `#[from]` 属性背后的机制：

```rust
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self { AppError::Parse(e) }
}

fn read_and_parse() -> Result<i32, AppError> {
    let content = std::fs::read_to_string("num.txt")?; // io::Error 自动转换
    let n = content.trim().parse::<i32>()?;            // ParseIntError 自动转换
    Ok(n)
}
```

**Q：转换可能失败怎么办？为什么不用 From？**

标准库约定 `From` 的转换**不应失败**（有损但恒定如 `f64 -> f32` 可以，可能失败的必须换 `TryFrom`/`TryInto`，返回 `Result`）：

```rust
use std::convert::TryFrom;

struct Port(u16);
impl TryFrom<String> for Port {
    type Error = std::num::ParseIntError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<u16>().map(Port)
    }
}

let p = Port::try_from(String::from("8080")).unwrap();
let bad = Port::try_from(String::from("abc")); // Err
```

> Rust 1.34 起 `TryFrom`/`TryInto` 稳定（2021 edition 中已无需显式 import）；`?` 同样支持 TryFrom 的错误转换。

**Q：一个类型能有多个 From 实现吗？能为外部类型实现 From 吗？**

- **可以多个**：`From` 的类型参数位置决定了同一个类型可实现任意多个来源（`impl From<u64> for UserId` + `impl From<String> for UserId` 并存）。
- **孤儿规则**：`From` 是标准库 trait，只能在"来源或目标至少有一个是自己定义的类型"时写 impl。想给外部类型加转换能力，用**新类型模式**（newtype）：

```rust
struct Kilometers(u32);
impl From<u32> for Kilometers { ... }
// impl From<u32> for String { ... }  // ❌ 编译错误：两个都不是自己定义的
```

**Q：From/Into 和 AsRef/AsMut 怎么选？**

| 需求 | 用哪个 |
|----|----|
| 函数需要**拥有**数据（存储/修改） | `impl Into<String>` / `From` |
| 只需要**读引用**，不接管所有权 | `impl AsRef<str>` / `impl AsRef<Path>` |
| 需要**可变引用** | `impl AsMut<T>` |
| 返回"借用或拥有"之一 | `Cow<'a, str>` |

```rust
// ❌ 不合理：为了读一下就拿走所有权，调用方被迫 clone
fn print_bad(s: impl Into<String>) { println!("{}", s.into()); }
// ✅ 合理：只借引用
fn print_good(s: impl AsRef<str>) { println!("{}", s.as_ref()); }
```

> 📌 **判断口诀：转换用 From/Into，借用用 AsRef/AsMut。**

**标准库预置的常用转换**：

```rust
let s: String = "lit".into();
let rc: std::rc::Rc<str> = "lit".into();
let x: u32 = 255u8.into();        // 仅无损方向：小 -> 大
// let z: i32 = y.into();         // i64 -> i32 有损，编译错误
let v: Vec<u8> = [1u8, 2, 3].into();
```

## 15. Cow（写时克隆）

**Q：Cow 是什么？解决什么问题？**

`Cow`（Clone-on-Write，`std::borrow::Cow`）是一个智能指针枚举，核心思想：**能借用就借用，必须修改时才克隆**——避免为"偶尔需要修改"的场景付出"总是克隆"的代价。

```rust
pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned {
    Borrowed(&'a B),               // 零成本借用
    Owned(<B as ToOwned>::Owned),  // 拥有克隆出来的数据
}
```

最常见形式是 `Cow<'a, str>` 和 `Cow<'a, [T]>`：

```rust
fn shout(input: &str) -> Cow<'_, str> {
    if input.chars().any(|c| c.is_lowercase()) {
        Cow::Owned(input.to_uppercase())   // 需要修改 -> 克隆（Owned）
    } else {
        Cow::Borrowed(input)               // 不需要修改 -> 直接借用（Borrowed）
    }
}

let a = shout("hello");  // Owned("HELLO")
let b = shout("WORLD");  // Borrowed("WORLD")，零分配
```

**Q：Cow 的关键行为是什么？**

- 实现了 **Deref**：`&Cow<str>` 可当 `&str` 直接用，使用方感知不到内部是借用还是拥有；
- **写时自动升级**：对 `Borrowed` 状态调用 `to_mut()` 会自动克隆并转为 `Owned`；已是 `Owned` 则直接复用，**不会重复克隆**。

| 方法 | 作用 |
|----|----|
| `cow.to_mut()` | 拿到 `&mut Owned`，必要时自动克隆一次 |
| `cow.into_owned()` | 消费自身，得到 `Owned` 值（Borrowed 时克隆） |
| `cow.is_borrowed()` / `is_owned()` | 判断当前状态 |

**Q：Cow 的典型使用场景？**

1. **字符串规范化**："大多数情况不用改，偶尔要改"——已经正常则借用，需要处理才克隆；
2. **结构体字段**：`struct Greeting<'a> { message: Cow<'a, str> }` 静态字符串零分配、拥有数据也能存——这是 serde 反序列化时 `Cow<'a, str>` 字段能**借用输入数据**的原理；
3. **返回值**：让调用方拿到"可能借用也可能拥有"的统一类型；
4. **参数**：更常用 `impl Into<Cow<'a, str>>`，同时接受 `&str` / `String` / `Cow`。

自定义类型只要实现 `Clone`（标准库有 blanket impl `impl<T: Clone> ToOwned for T`）即可配合 Cow 使用。

**Q：Cow 有哪些常见坑？**

```rust
// 1. Cow<str> 不是 String，传给要 String 的函数需显式转换
let s: String = c.into_owned();

// 2. to_mut 只克隆一次——Owned 之后直接复用
let mut c = Cow::Borrowed("a");
c.to_mut().push('b');  // 克隆一次
c.to_mut().push('c');  // 不再克隆

// 3. 生命周期：Borrowed 不能超过源数据的生命周期
fn bad<'a>(s: &'a str) -> Cow<'static, str> {
    Cow::Borrowed(s) // ❌ 编译错误
}
```

> 📌 与第 14 节呼应：返回"借用或拥有"之一用 `Cow`；只读借用用 `AsRef`；要所有权用 `Into`。

## 16. 泛型（Generics）与单态化

**Q：为什么需要泛型？**

没有泛型时，同一逻辑要为每种类型重复实现一遍——代码重复、维护困难：

```rust
fn largest_i32(list: &[i32]) -> &i32 { /* ... */ }
fn largest_char(list: &[char]) -> &char { /* 完全相同的逻辑再写一遍 */ }
```

泛型把**类型提升为参数**，一份代码通吃所有满足约束的类型：

```rust
// T: PartialOrd 是 trait bound：保证 T 之间可以比较大小
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

largest(&vec![34, 50, 25, 100, 65]); // T = i32
largest(&vec!['y', 'm', 'a', 'q']);  // T = char
```

**Q：泛型结构体和 impl 怎么写？**

```rust
struct Point<T> { x: T, y: T }        // 单类型参数：x、y 必须同类型
struct Point2<T, U> { x: T, y: U }    // 多类型参数：x、y 可不同类型

// impl 后的 <T> 是重新声明类型参数，与结构体定义的 <T> 是两套（只是同名对应）
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

let p = Point { x: 5, y: 10 };       // 编译器自动推断 T = i32
let f = Point2 { x: 1, y: 4.0 };     // T = i32, U = f64
```

> 💡 只有特定类型参数组合才有的方法，可以在 impl 上**加约束**：`impl Point<f32> { fn distance_from_origin(&self) -> f32 { ... } }`——只有 `Point<f32>` 实例能调用。

**Q：什么是单态化（monomorphization）？泛型的性能如何？**

编译期把泛型代码**按实际使用的具体类型逐一展开**生成代码——所以泛型是**零成本抽象**：

```rust
largest(&vec![1, 2, 3]);  // 编译器生成 largest_i32 版本
largest(&vec!['a']);       // 编译器生成 largest_char 版本
```

- 运行时**没有**类型擦除/装箱/虚函数调用的开销，与手写具体类型版本性能完全一致；
- 代价是**代码体积膨胀**（每种类型一份副本），以及编译时间增加。

> 📌 对比第 9 节：泛型走 `impl Trait` **静态分发**（单态化、快、体积大）；`dyn Trait` **动态分发**（vtable、一层间接、体积小）——性能与体积的权衡。

**Q：trait bound 的两种写法？**

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }        // 简短，约束少时用
fn report<T>(item: &T) where T: std::fmt::Debug + Clone { ... } // where 从句，约束多/复杂时用，可读性好
```

多个约束用 `+` 连接（`T: Debug + Clone`）；约束之间存在关联类型等复杂关系时用 `where` 从句更清晰。

## 17. 字符串（String / &str / 字面量）

**Q：Rust 中有几种字符串类型？有什么区别？**

Rust 的字符串类型可归为五族（共 10 余种），先给全景再逐个讲：

**A. 核心 UTF-8 字符串**

| 类型 | 内存 | 所有权 | 说明 |
|---|---|---|---|
| `str` | — | 无 | 未定长 primitive，字符串切片的**本体**，几乎总以 `&str` 出现 |
| `&str` | 静态区或指向 String 内部 | 借用 | 指针+长度，不可变；字面量的类型 |
| `String` | 堆 | 拥有 | 可增长、可变的 UTF-8 缓冲区（内部 Vec\<u8\> + len + capacity） |
| `&String` | — | 借用 | 几乎总该用 `&str` 替代（deref coercion 自动转） |
| `Box<str>` | 堆 | 拥有 | 无 capacity 的 String，省 8 字节，适合"建好不再变" |
| `Rc<str>` / `Arc<str>` | 堆 | 共享 | 共享不可变字符串，clone 只加引用计数、不复制数据 |
| `Cow<'a, str>` | 堆/借用 | 两用 | 写时克隆（见第 15 节） |

**B. 操作系统原生字符串（可能非 UTF-8）**

| 类型 | 说明 |
|---|---|
| `OsStr` / `OsString` | 平台原生字符串：Unix 上是字节序列，Windows 上是 UTF-16；**可能含非法 UTF-8** |
| `Path` / `PathBuf` | 文件系统路径，本质是 `OsStr` / `OsString` 的轻量包装 |

**C. C 互操作字符串（NUL 结尾）**

| 类型 | 说明 |
|---|---|
| `CStr` / `CString` | C ABI 字符串，`\0` 结尾、不能含内部 NUL，FFI 专用（见工程化篇第 9 节） |

**D. 字节串（原始字节）**

| 类型 | 说明 |
|---|---|
| `&[u8]` / `Vec<u8>` | 字节切片/向量，网络、加密、二进制协议等场景的"字符串" |
| `[u8; N]` | 定长字节数组（`b"..."` 字面量的类型） |

**E. 字面量语法**

| 语法 | 类型 | 说明 |
|---|---|---|
| `"..."` | `&'static str` | 普通字符串字面量 |
| `r"..."` / `r#"..."#` | `&'static str` | **原始字符串**，反斜杠不转义（正则、Windows 路径好用） |
| `b"..."` | `&'static [u8; N]` | **字节串**字面量 |
| `br"..."` | `&'static [u8; N]` | 原始字节串 |
| `'c'` | `char` | Unicode 标量值（单字符，非字符串） |

```rust
let data = "initial contents";        // &'static str 字面量，编译期进二进制
let s = data.to_string();             // &str -> String（拷贝到堆）
let s = String::from("initial contents"); // 等价写法
let s: String = "lit".into();         // 等价写法

let re = r"\d+\.\d+";                 // 原始字符串：反斜杠原样保留
let win = r"C:\Program Files";        // 不用写 \\ 
let bytes = b"hello";                 // &[u8; 5]，不是 &str
```

**Q：String 的常用操作有哪些？拼接时注意什么？**

```rust
let mut s = String::new();
s.push_str("bar");                    // 追加 &str
s.push('!');                          // 追加单个 char
s = s.replace("foo", "world");        // 查找替换（返回新 String）
s.contains("hello");                  // 查找

// + 拼接：注意 s1 被移动！
let s3 = s1 + &s2;  // 等价 s1.add(&s2)，add(self, s: &str) 拿走 s1 的所有权并复用其缓冲区
// println!("{s1}"); // ❌ s1 已失效

// format! 不移动任何操作数，最常用
let s = format!("{s1}-{s2}-{s3}");
```

> 💡 `+` 的签名是 `fn add(mut self, s: &str) -> String`，`&String` 能直接传是因为 deref coercion 自动转成 `&str`。

**Q：为什么不能按索引访问字符串？怎么正确遍历？**

String 是 **UTF-8** 编码：一个"字符"可能是 1~4 个字节，**字节索引不落在字符边界会 panic**：

```rust
let hello = "Здравствуйте";   // 西里尔字母，每字符 2 字节
// let s = &hello[0..1];      // ❌ 运行时 panic：不是字符边界

for c in hello.chars() { /* char：按 Unicode 标量值遍历 */ }
for b in hello.bytes() { /* u8：按底层字节遍历 */ }
```

面试要点：**UTF-8 是 Rust 字符串的根基**——`len()` 返回字节数不是字符数（`"你好".len() == 6`），随机索引被禁止，`chars().count()` 才是字符数。

**Q：Path / OsStr / OsString 是什么？和 String 什么关系？**

`OsStr`/`OsString` 是**平台原生**字符串：Unix 上是任意字节序列，Windows 上是 UTF-16——**不保证是合法 UTF-8**（例如 Linux 文件名可以是任意非 `\0`、`/` 的字节）。`Path`/`PathBuf` 就是 `OsStr`/`OsString` 的轻量包装，专用于文件路径：

```rust
use std::path::{Path, PathBuf};

let p: &Path = Path::new("src/main.rs");       // Path 是 &OsStr 类似物（借用）
let pb: PathBuf = PathBuf::from("src");        // PathBuf 是 OsString 类似物（拥有）
let pb = pb.join("main.rs");                    // 拼接路径（自动处理分隔符）

// 与 str 的互转（可能失败，因为 OsStr 可能非 UTF-8）
let s: Option<&str> = p.to_str();              // 合法 UTF-8 → Some，否则 None
let lossy = p.to_string_lossy();               // 非法序列用 � (U+FFFD) 替换
let os: &std::ffi::OsStr = std::ffi::OsStr::new("hello");
```

关键区别：**`String` 保证 UTF-8，`OsStr` 不保证**——所以 `to_str()` 返回 `Option<&str>`，`to_string_lossy()` 会做有损替换。凡是碰文件系统、环境变量、命令行参数，底层都是 `OsStr`。

> 📌 实用惯例：读写文件的函数入参用 `impl AsRef<Path>`，同时接受 `&str`、`String`、`Path`、`PathBuf`：
> ```rust
> fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
>     std::fs::read_to_string(path)
> }
> read_file("test.md");                  // &str
> read_file(PathBuf::from("test.md"));   // PathBuf
> ```

**Q：CStr / CString 和字节串 [u8] 什么时候用？**

**CStr/CString**：FFI 调 C 库时用（见工程化篇第 9 节），`\0` 结尾、内部不能含 `\0`：

```rust
use std::ffi::CString;

let c = CString::new("hello").unwrap();  // 若含内部 \0 会返回 Err
let ptr = c.as_ptr();                    // *const c_char，传给 C 函数
let s: &str = c.to_str().unwrap();       // 转回 &str
```

**字节串 [u8] / Vec<u8>**：二进制协议、网络包、加密哈希等"不是给人读的字符串"场景，用 `&[u8]` 而非 `&str`：

```rust
let packet: &[u8] = &[0x00, 0x01, 0xff];
let s = String::from_utf8(packet.to_vec());  // 字节 → String：合法 UTF-8 才成功（Result）
let raw = s.into_bytes();                     // String → Vec<u8>
```

> 📌 选型口诀：给人读的文本用 `&str`/`String`；文件路径用 `Path`/`PathBuf`；跨 FFI 用 `CStr`/`CString`；二进制数据用 `&[u8]`/`Vec<u8>`——四类各有归属，不可混用。

**Q：函数签名中字符串参数/返回值怎么选？（高频设计题）**

| 场景 | 签名 |
|---|---|
| 只读字符串 | `fn f(s: &str)` —— 同时接受字面量、`String`、`Cow`（deref coercion） |
| 需要拥有/存储它 | `fn f(s: impl Into<String>)` 或 `From<&str>` |
| 返回"借用或拥有" | `-> Cow<'a, str>`（见第 15 节） |

> 📌 `&str` vs `String` 是最经典的设计题：入参默认 `&str`（最不挑剔），要所有权才 `Into<String>`；返回值想避免无谓拷贝就用 `Cow<'a, str>`。

---

# 二、并发与异步

## 1. 并发模型

**Q：Rust 的并发模型是什么？**

- 基于**所有权和借用**：每个值由单个线程拥有，所有权可通过消息传递跨线程转移；
- 两种并发范式：
  - **共享内存**（锁 + 原子类型）："通过共享内存来通信"；
  - **消息传递**（CSP）："不要通过共享内存来通信，而要通过通信来共享内存"（Go 名言，Rust `std::sync::mpsc` 同属此类），消除了共享内存的复杂性，从根本上规避数据竞争；
- 编译期防止数据竞争：不满足 Send/Sync 的类型无法跨线程传递，编译直接报错；
- 提供线程（`std::thread::spawn`）+ 同步原语（Mutex、channel 等）+ async 运行时三层工具。

**Q：并发（Concurrency）和并行（Parallelism）有什么区别？（高频概念题）**

- **并发（Concurrency）**：指**处理**多个任务的能力——任务交替执行，任一时刻 CPU 可能只跑一个任务。核心是**结构**：把任务拆解成可独立推进、可交错执行的小块（如多个 Future 交替 poll）；
- **并行（Parallelism）**：指**同时**执行多个任务的能力——多核 CPU 上真正多个任务同时运行。核心是**硬件**：依赖多核。

```text
并发（单核交替）          并行（多核同时）
task A ██░░██░░            task A ████████
task B ░░██░░██            task B ████████
       时间轴 →                   时间轴 →
```

关键区别与联系：

| | 并发 | 并行 |
|---|---|---|
| 本质 | 任务**交错**执行 | 任务**同时**执行 |
| 依赖 | 单核即可 | 需要多核 |
| 解决 | 结构问题（任务拆分/调度） | 性能问题（CPU 利用率） |
| Rust 对应 | `async`/`await` + 单线程 runtime | `thread::spawn` + 多核 / 多线程 runtime |

> 📌 面试话术：并发是"关于**如何处理**多个任务"（结构），并行是"关于**如何同时做**多个任务"（硬件）；**并发不一定并行**（单核时间片切换、单线程 runtime 上跑多个 async 任务），**并行必是并发**的更高形态。tokio 多线程 runtime 是"并发 + 并行"兼得：多个 task 并发调度 + 多个 worker 线程并行执行。

## 2. Send 与 Sync

**Q：Send 和 Sync 是什么？**

- **Send**：类型的**值**可在线程间安全转移（move 到另一个线程）；
- **Sync**：类型的**引用** `&T` 可在线程间安全共享（多线程同时读）。

记忆口诀：**Send 能搬家，Sync 能合租**。大部分类型编译器自动实现；`Rc<T>` 不是 Send/Sync，`Arc<T>` 是。

```rust
struct MyStruct { data: u32 }
unsafe impl Send for MyStruct {}   // 手动实现需要 unsafe
```

> 💡 加分点：
> - Send/Sync 是**自动派生**的标记 trait——结构体所有字段都满足 Send，则该结构体自动实现 Send；
> - `&T` 实现 Send 的条件是 `T: Sync`；
> - 手动实现 Send/Sync 是 `unsafe` 的责任：你必须自己保证线程安全。

## 3. 线程创建：thread::spawn 签名解析

**Q：std::thread::spawn 的签名为什么这样设计？**

标准库源码（完整签名）：

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(miri, track_caller)]   // 便于 Miri 回溯 panic 位置
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,   // ①
    F: Send + 'static,  // ②
    T: Send + 'static,  // ③
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```

- spawn 是 `F`、`T` 两个类型参数的泛型函数：`f` 是闭包入参，返回 `JoinHandle<T>`；实际由 `Builder::new().spawn(f)` 创建线程（Builder 可定制栈大小、线程名，见第 4 节）。

三个约束逐一拆解（中高级岗高频追问点）：

- **① `FnOnce() -> T`**：线程入口是只能调用一次的闭包，可捕获并**消费**外部变量的所有权——线程独立运行、只执行一次，与 FnOnce 语义天然匹配；闭包一旦执行完，被捕获的外部变量在闭包外就不可再用；
- **② `F: Send + 'static`**：闭包本身及其捕获的所有变量必须能安全转移到新线程（Send），且不借用任何可能失效的引用（'static，指不持有非 'static 引用，而非"永远存活"）；
- **③ `T: Send + 'static`**：闭包返回值要传回主线程（通过 `join()`），同样需要可转移、无悬垂引用。

**Q：Send 到底是怎样"自动"实现的？**

`Send` 是**自动派生的标记 trait**——只要结构体的所有字段都满足 `Send`，编译器就自动为该结构体实现 `Send`。大多数类型都是 Send；**不满足 Send 的主要是引用/指针**：`&T` 是 Send **当且仅当 `T: Sync`**（因为 `&T` 意味着多线程可同时读）。`Rc<T>` 不是 Send（引用计数非原子），`Arc<T>` 是（见第 2、6 节）。

```rust
struct MyStruct { data: u32 }   // 自动 impl Send，因为 u32: Send
unsafe impl Send for MyStruct {}  // 手动实现才需要 unsafe（对编译器承诺线程安全）
```

**Q：为什么"主线程退出会杀掉子线程"？join 怎么解决？（必考陷阱）**

```rust
use std::{thread, time};

fn main() {
    thread::spawn(|| {
        for c in 'a'..='z' {            // 输出 a-z
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", c);
        }
    });
    thread::spawn(|| {
        for i in 1..=9 {                // 输出 1-9
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", i);
        }
    });

    thread::sleep(time::Duration::from_secs(2)); // ❌ 靠 sleep 硬等，脆弱
}
```

问题：进程在主线程 `main` 结束时**立即退出**，仍在运行的子线程被直接终止。上面靠"主线程 sleep 2 秒"硬等——但子线程耗时一变化就会丢输出。正确做法是 `join`（阻塞主线程直到子线程结束）：

```rust
let handler1 = thread::spawn(|| { /* 输出 a-z */ });
let handler2 = thread::spawn(|| { /* 输出 1-9 */ });

// join 阻塞等待线程结束，并取回返回值
handler1.join().unwrap();
handler2.join().unwrap();
println!("the two threads are finished");
println!("main thread will exit");
```

> 📌 要点：`join()` 返回 `Result<T, Box<dyn Any>>`（`T` 是闭包返回值，`Err` 表示子线程 panic）；主线程 exit 会杀掉未 join 的线程，所以**凡是 spawn 了就要考虑何时 join**。

## 4. 线程栈大小与自定义线程

**Q：Rust 线程默认栈多大？如何自定义？**

- 主线程栈大小由操作系统决定（Linux 64 位默认 8MB，可用 `ulimit -s` 查看）；
- `thread::spawn` 创建的线程默认最小栈 **2MB**（依平台变化）；
- 两种自定义方式：
  1. `thread::Builder::new().stack_size(n)`（代码内）；
  2. `RUST_MIN_STACK` 环境变量（字节数）；**`stack_size()` 会覆盖环境变量**；
- 栈不是越大越好：过大浪费内存，过小可能栈溢出；超 OS 限制会创建失败。

```rust
let builder = thread::Builder::new()
    .stack_size(1 * 1024 * 1024)   // 1MB
    .name("my_thread".to_string()); // 线程命名，排查时线程名直接可见
let handler = builder.spawn(|| { /* ... */ }).unwrap();
handler.join().unwrap();
```

> 💡 加分点：给线程命名（`name()`）是线上排查的好习惯，`gdb thread apply all bt` 时一眼定位业务线程。

## 5. move 闭包与跨线程所有权转移

**Q：为什么线程闭包前面要写 move？**

`spawn` 要求 `'static`，闭包不能借用父作用域的局部变量（子线程可能活得更久）。`move` 把捕获变量的**所有权**移入闭包，由新线程拥有：

```rust
let data = vec![1, 2, 3, 4, 5];
let handle = std::thread::spawn(move || {
    for i in data { println!("{}", i); } // data 的所有权已转移
});
handle.join().unwrap();
// println!("{:?}", data); // ❌ 编译错误：所有权已移动
```

要点：

- 所有权移入后，原作用域**不可再用**该变量（编译错误兜底，杜绝 use-after-free）；
- 若需要多线程共享同一数据，用 `Arc::clone` + `move`：每个线程持有一个引用计数克隆。

## 6. Mutex / RwLock 与 Arc

**Q：Mutex 和 Arc 分别解决什么问题？为什么经常一起用？**

- **Mutex（互斥锁）**：解决**并发访问**——同一时刻只有一个线程能持有锁修改数据；`lock()` 返回 `MutexGuard`，**离开作用域自动解锁**（RAII）。
- **Arc（原子引用计数）**：解决**所有权共享**——Rust 禁止直接跨线程转移所有权，Arc 通过原子引用计数让多个线程共享同一数据，计数归零自动释放。

```rust
let counter = Arc::new(Mutex::new(0));
for _ in 0..5 {
    let counter = counter.clone();          // 克隆的是引用计数，不是数据
    let h = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    h.join().unwrap();
}
```

Mutex 的代价与选型：

- **性能**：高并发下频繁加锁导致线程阻塞/唤醒；
- **死锁风险**：嵌套持锁、加锁顺序不一致；
- **读多写少**场景用 `RwLock`（`Arc<RwLock<T>>`）缩短阻塞时间；或换 `parking_lot` 降低锁开销。

> 💡 加分点：Mutex 在 Rust 中"守护数据"而非"守护代码"（data-oriented lock）；`MutexGuard` 不可跨 `.await` 使用（不是 Send 时编译报错），这是异步死锁的常见来源。

## 7. 共享数据与资源保护

三层防护体系：

1. **所有权系统**：每个值唯一所有者，离开作用域自动释放 → 防悬垂指针；
2. **借用规则**：`&mut` 唯一、`&` 可并存 → 编译期防数据竞争；
3. **同步类型**：`Mutex`/`RwLock`/`Atomic*` → 运行时跨线程保护。

## 8. 死锁与竞争条件

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

## 9. Channel 与 Select

**Q：channel 是什么？select 的作用？**

- **channel**：并发任务间单向传递消息的机制，CSP 范式的实现。
  - 标准库 `std::sync::mpsc`：**多生产者、单消费者**（Multiple Producer Single Consumer）；
  - tokio `mpsc`（带缓冲的异步通道）、`broadcast`（广播）、`oneshot`、`watch`。
- **select**（`tokio::select!`）：异步 I/O 多路复用，同时监听多个异步操作，任一就绪即执行对应分支。

```rust
// mpsc：Sender 是 Clone 的，可克隆出多个生产者；receiver 实现了 Iterator
let (tx, rx) = std::sync::mpsc::channel();
let tx1 = tx.clone();
thread::spawn(move || { tx1.send("hello".to_string()).unwrap(); }); // send 需 move 转移所有权
thread::spawn(move || { tx.send("hello,world".to_string()).unwrap(); });
for msg in rx {   // 接收端当迭代器用，通道关闭即结束
    println!("Received msg: {}", msg);
}
```

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

> 📌 要点：发送端 `send` 需要拿到 Sender 的所有权（`move` 进线程）；**接收端唯一**且实现了 `Iterator`，`for msg in rx` 会在通道关闭后自然结束。

## 10. 全局变量

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

> 💡 加分点：除非写 OS 底层/FFI 边界，一律选 `AtomicBool`（x86 上就是一条指令，性能无感知差异）。

## 11. 异步编程（Future / async / await / Pin / Waker）

**Q：async/await 的原理？为什么 Future 需要 Pin？**

- `async fn` 被编译器改写为返回**匿名状态机结构体**（实现了 `Future`）；函数体里每个 `.await` 是状态机的一个**挂起点**；
- Future 是惰性的：创建后什么都不做（直接调用返回 Future 而不 poll，什么都不打印），被 `poll` 才推进；返回 `Poll::Pending` 时注册 waker，事件就绪后 `wake` 重新调度。

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- **Pin 存在的意义**：async 状态机可能**自引用**（某个状态持有指向自身其他字段的引用）。如果结构体被移动，内部引用会失效成悬垂指针。`Pin` 保证 Future 在 poll 之后不会被移动（`!Unpin` 类型）。

**Q：Waker 是什么？poll 返回 Pending 之后发生了什么？（资深岗必问）**

`Context` 内部持有一个 `Waker`，它是连接 Future 与 executor 的唤醒句柄：

```rust
pub struct Waker { waker: RawWaker }

pub struct RawWaker {
    data: *const (),               // 类型擦除指针（如指向任务的 Arc）
    vtable: &'static RawWakerVTable, // wake/wake_by_ref/drop 等虚函数表
}
```

运行闭环：

1. `poll` 发现资源未就绪 → 把 `Waker` 注册到事件源（如 epoll/reactor）→ 返回 `Poll::Pending`；
2. 事件就绪 → 事件源调用 `waker.wake()` → executor 收到通知，把该任务重新放入调度队列；
3. executor 再次 `poll` 该任务，直至返回 `Poll::Ready(T)`。

**Q：block_on 是怎么实现的？为什么 Rust 没有内置异步运行时？**

标准库只提供 Future/Poll/Context/Waker 等**基本要素**，不含 executor——运行时由社区提供（tokio、async-std、futures、smol）。futures 的 `block_on` 核心实现：

```rust
pub fn block_on<F: Future>(f: F) -> F::Output {
    pin_mut!(f);                              // 把 Future 固定在栈上（Pin 的体现）
    run_executor(|cx| f.as_mut().poll(cx))    // 循环 poll 直至 Ready
}
```

**Q：Rust 的 Future 与 JS/Python 的协程有何本质区别？（加分项）**

Rust 的 Future **不代表一个发生在后台的计算，Future 本身就是计算**——它是一个状态机，惰性且零开销，其所有者有责任推进它（poll）。没有隐式的事件循环线程在背后驱动，调度权完全在 executor 手中，这也是 Rust 异步"零成本抽象"的根基。

**Q：async fn 与 async 块有何区别？async move 块是什么？**

Rust 有两种创建 Future 的语法：**async fn**（函数）和 **async 块**（表达式），都返回实现 `Future` 的值。async 块和普通闭包一样支持 `move` 关键字：

```rust
use futures::executor::block_on;
use futures::Future;

// ① 普通 async 块：借用局部变量，多个块可同时访问同一变量
async fn blocks() {
    let greet = "hello,world".to_string();

    let future_one = async { println!("{greet}"); };
    let future_two = async { println!("{greet}"); };

    // 两个 Future 并发执行，各借用一次 greet，都正常打印
    futures::join!(future_one, future_two);
}

// ② async move 块：把所有权移入 Future，允许它超出原作用域
fn move_block() -> impl Future<Output = ()> {
    let lang = "rust".to_string();
    async move {                       // lang 的所有权转移到 async 块中
        println!("{}", lang);
    }
}

fn main() {
    block_on(blocks());

    let future = move_block();          // 返回的 Future 已持有 lang，不依赖原作用域
    block_on(future);
}
```

区别与要点：

- **普通 async 块**：借用局部变量——同一作用域内**多个 async 块可同时借用同一变量**（如上面的 `greet`），借用规则照常生效；
- **async move 块**：把捕获变量的**所有权移入** Future，让 Future 能**超出变量的原始作用域**存活（`move_block` 里 `lang` 是局部变量，但返回的 Future 已拥有它）；代价是**放弃与其他代码共享该变量**的能力；
- 典型场景：`tokio::spawn` 要求 `Future + Send + 'static`（见第 12 节），当闭包/块需要持有非 'static 的局部数据时，用 `async move` 把所有权移进去、自给自足，才满足 `'static` 约束。

> 📌 记忆：async 块的 `move` 与闭包的 `move`（第 10 节）语义完全一致——"能借用就不 move，要跨作用域/进 spawn 就 move"。

> 📌 版本脉络：Rust 1.36（2019.07）Future trait 进标准库 → **1.39（2019.11）** async/await 语法稳定 → 1.75（2023.12）trait 中支持 async fn。

## 12. Tokio 运行时

**Q：`#[tokio::main]` 到底做了什么？**

它是属性宏，把 `async fn main` 展开为**运行时创建 + block_on** 的代码：

```rust
#[tokio::main]
async fn main() { /* ... */ }

// 等价于：
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { /* ... */ });
}
```

手动使用 `Runtime` 的等价写法：

```rust
let rt = Runtime::new().unwrap();
rt.block_on(async { /* 驱动 Future 执行 */ });

let handler = rt.spawn(async { /* 后台异步任务 */ }); // spawn 立即在后台运行
rt.block_on(handler).unwrap();                         // JoinHandle 本身也是 Future
```

**Q：tokio::spawn 的签名与 std::thread::spawn 有何异同？**

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
```

- 参数从闭包换成 Future，但 `Send + 'static` 约束**完全相同**（tokio task 可能被调度到任意线程执行）；
- 返回值 `JoinHandle<T>` 概念与 `std::thread::JoinHandle` 一致，且**实现了 Future**，`.await` 它即可等待任务完成；
- tokio task 类似绿色线程（可类比 Go 协程/Erlang 进程调度），由运行时而非 OS 调度。

> 💡 **绿色线程与调度类比（加分点）**：tokio 的 `task` 是**用户态协作式调度**的轻量任务——不是 OS 线程，由 tokio 运行时在**少量 worker 线程**上调度海量 task。可类比：
> - **Go 的 GMP 调度**：G（goroutine）≈ task，M（OS 线程）≈ worker 线程，P（逻辑处理器）≈ worker 队列；
> - **Erlang 进程**：语言级轻量进程 + 抢占/归约式调度；
>
> 差异在于：tokio task **不抢占**，只有在 `.await` 点让出控制权（协作式），因此**阻塞 CPU 或 std::sync 锁会卡住整个 worker**（见第四篇死锁排查）。

**Q：tokio 运行时内部由哪些组件构成？（执行原理）**

一个 `Runtime` 由四大组件协同工作：

```text
                        ┌─────────────────────────────┐
                        │         Runtime             │
                        ├─────────────────────────────┤
  task::spawn ────────▶ │  Scheduler（调度器）         │
                        │  · worker 线程 × N(=CPU核数)  │
                        │  · 每 worker 一个本地队列(LIFO)│
                        │  · 全局注入队列(spawn 落点)   │
                        ├─────────────────────────────┤
        .await 网络/文件  │  I/O Driver（Reactor）      │
        ───────────────▶ │  · Mio 封装 epoll/kqueue     │
                        │  · 负责注册/监听 fd 就绪事件   │
                        ├─────────────────────────────┤
        sleep/timeout    │  Timer（时间轮）             │
        ───────────────▶ │  · sleep/interval/timeout    │
                        ├─────────────────────────────┤
        spawn_blocking   │  Blocking Pool（阻塞线程池） │
        ───────────────▶ │  · 跑 CPU 密集/阻塞操作       │
                        └─────────────────────────────┘
```

| 组件 | 作用 | 对应 API |
|---|---|---|
| **Scheduler** | 调度 task 到 worker 线程，work-stealing | `tokio::spawn`、`Runtime::new` |
| **I/O Driver** | 基于 Mio 封装 epoll/kqueue，监听 fd 就绪 | `tokio::net`、`tokio::fs` |
| **Timer** | 哈希时间轮管理定时任务 | `tokio::time::sleep` |
| **Blocking Pool** | 专用线程跑阻塞/CPU 密集操作，不占用 worker | `tokio::task::spawn_blocking` |

`enable_all()` 就是同时开启 I/O Driver + Timer；纯计算场景（无 IO/无 sleep）可只开调度器。

**Q：一个 task 从 spawn 到完成经历了什么？（运行机制）**

1. **入队**：`tokio::spawn(future)` 把 task 投入调度队列（默认全局注入队列）；
2. **取出 poll**：某个空闲 worker 从队列取出 task，调用 `future.poll()` 推进；
3. **遇 .await 未就绪**：`.await` 的网络/IO 操作把 `Waker` 注册到 Reactor，返回 `Poll::Pending`；
4. **worker 不阻塞，继续干活**：worker **立刻转去 poll 队列里下一个 task**（这就是"并发"——一个 worker 交替推进多个 task）；
5. **事件就绪 → 唤醒**：epoll 检测到 fd 可读 → Reactor 调 `waker.wake()` → task 被重新放回队列；
6. **再次 poll**：worker 重新 poll 该 task，这次返回 `Poll::Ready(T)`，task 完成。

> 📌 核心：**worker 线程从不阻塞在单个 task 上**——IO 等待期间它去跑别的 task；这也是为什么 `gdb bt` 只看到 `epoll_wait`：当所有 task 都 pending 时，worker 没活干，整体阻塞在 epoll 上等 IO 事件。

**Q：work-stealing 是怎么工作的？为什么能充分利用多核？**

多线程调度器（默认）的核心算法：

- 每个 worker 线程有自己的**本地任务队列**（LIFO，取任务最快、缓存友好）；
- `spawn` 的任务进**全局注入队列**，由 worker 按需取用；
- 某 worker 本地队列**空了**，就"**偷**"其他 worker 本地队列**另一端**（FIFO 端）的任务；
- 被偷的往往是**最老**的任务——老任务依赖的缓存大概率已失效，偷它不伤原 worker 的局部性。

效果：任务在 worker 间自动负载均衡，多核 CPU 得到充分利用；无需显式分配任务到线程。

**Q：单线程 vs 多线程调度器怎么选？spawn_blocking 什么时候用？**

- **多线程调度器**（`new_multi_thread`，默认）：work-stealing 利用多核，`tokio::spawn` 任务可被任意 worker 执行；
- **单线程调度器**（`new_current_thread`）：只有一个 worker，配合 `spawn_local` 运行 `!Send` 任务（`Rc`、非 Send 句柄），无跨线程同步开销；适合小工具、测试、嵌入式；
- **`spawn_blocking`**：把**阻塞/CPU 密集**操作（同步文件 IO、耗时计算、`std::sync::Mutex` 长持锁）丢到独立线程池，**避免卡死 worker**——否则一个阻塞调用会占住 worker，该 worker 上的其他 task 全停摆。

```rust
// 协作式让出：耗时计算可手动 yield，避免长时间霸占 worker
let result = tokio::task::spawn_blocking(|| {
    // 阻塞/CPU 密集操作在这里跑，不占用异步 worker
    std::thread::sleep(std::time::Duration::from_secs(1));
    42
}).await.unwrap();
```

> 📌 一句话总结执行原理：**Runtime = 调度器（work-stealing）+ I/O 驱动（epoll）+ 定时器（时间轮）+ 阻塞线程池**；task 是协作式调度的状态机，`.await` 点让出 worker，IO 就绪靠 Waker 唤醒重新入队。

**Q：异步测试怎么写？**

```rust
#[tokio::test]           // 自动创建运行时执行测试
async fn test_spawn() {
    let _ = tokio::spawn(async { println!("hello"); }).await;
}
```

**Q：多线程调度器与单线程调度器怎么选？**

- **多线程调度器**（默认，`new_multi_thread`，work-stealing）：空闲 worker 从繁忙 worker **偷任务**，充分利用多核；
- **单线程调度器**（`new_current_thread`）：配合 `spawn_local` 运行 `!Send` 任务（如 `Rc`、非 Send 句柄），无跨线程同步开销；
- `spawn` vs `spawn_local`：前者任务必须 `Send`（可跨线程调度），后者只能当前线程运行。

> 💡 加分点：tokio 下 `gdb bt` 常只看到 `epoll_wait`，因为业务逻辑在异步任务里调度 → 用 **tokio-console** 看任务级状态（poll 次数、耗时、waker、await 点）。

---

# 三、工程化

## 1. Cargo.toml 与 Cargo.lock

- **Cargo.toml**：项目元数据 + 依赖声明 + 构建配置（profile），TOML 格式；
- **Cargo.lock**：锁定**精确版本**（含传递依赖），保证任何人构建都用相同版本，避免依赖冲突。**应用**项目提交 lock，**库**项目不提交。

## 2. crate、module 与包管理

- **crate**：编译单元（二进制或库），crates.io 是中央包注册表；
- **module**：crate 内部的代码组织单位（`mod` 关键字），可嵌套、可跨文件；
- 一个项目（package）可含多个 crate，一个 crate 含多个 module。

## 3. 模块化编程实践（mod 机制 / 文件目录组织 / pub use）

**Q：模块有哪几种定义方式？**

两种：**内联代码块**（小项目/测试）和**文件即模块**（工程实践主流）：

```rust
// ① 内联定义
mod service {
    pub mod user {
        pub fn say_hello(name: String) { println!("hello,{}", name); }
    }
}

// ② 文件即模块：src/user.rs 就是 user 模块，crate 根用 mod 声明引入
// src/main.rs
mod user;                          // 告诉编译器加载 src/user.rs
fn main() { user::say_hello("daheige".to_string()); }
// src/user.rs
pub fn say_hello(name: String) { println!("hello,{}", name); } // pub 才对外可见
```

> 📌 要点：Rust **没有** `include!` 式的自动文件关联——文件要成为模块，必须在父模块中用 `mod xxx;` 显式声明；模块内的一切（函数、结构体、甚至子模块）**默认私有**，需要对外暴露必须加 `pub`。

**Q：多文件/目录项目怎么组织模块？**

两种目录模式（可混合使用）：

```text
模式 A：目录模块（模块还有子模块时）        模式 B：同名文件模块（模块代码量小时）
src/                                        src/
├── main.rs  → mod feed;                    ├── main.rs  → mod auth;
├── feed/                                   ├── auth.rs      → pub fn check() {}
│   ├── mod.rs     → mod feed;  ← 加载同目录 feed.rs
│   │                pub use feed::{show, edit};  ← 常用：在 mod.rs 里重导出
│   └── feed.rs    → pub fn show() {} ...
└── auth.rs  → pub mod post;                └── auth/
                 └── post.rs                   └── post.rs
```

```rust
// src/main.rs
mod auth;
mod feed;
use auth::post;                    // use 引入路径，省去每次写全路径

fn main() {
    feed::show();                  // feed/mod.rs 重导出了 show
    post::show("rust lang".to_string()); // 经由 use 直接使用
}
```

**Q：路径与可见性关键字怎么用？（self / super / crate / pub 系列）**

- **路径起点**：`crate::`（crate 根）、`self::`（当前模块）、`super::`（父模块）、裸标识符（当前模块内，或 use 引入的名字）；
- **pub 系列**（可见性梯度，资深岗加分）：
  - `pub`：对外完全公开；
  - `pub(crate)`：仅本 crate 内可见——**大型工程限制 API 面的首选**；
  - `pub(super)` / `pub(in path)`：限定父模块/指定路径可见；
  - 不加 pub：仅当前模块及子模块可见。

**Q：pub use 是什么？为什么需要重导出？**

`pub use` 把内部路径**重导出**为对外 API，调用方无需了解内部目录结构：

```rust
// 内部结构：src/service/user.rs、src/service/feed.rs
// 对外只暴露简洁入口（如 lib.rs 中）：
pub use crate::service::user;
pub use crate::service::feed;

// 调用方直接用，不必知道 service 这层
user::say_hello("kitty".to_string());
feed::show();
```

工程价值：

1. **隐藏内部实现**：内部目录可以随意重构（拆分/合并/移动），只要重导出的路径不变，外部代码零改动；
2. **API 面最小化**：标准库和主流 crate（serde、tokio）的 lib.rs 都是一长串 `pub use`，使用者只看到精心设计的顶层 API；
3. 常配合 `pub(crate)` 使用：内部模块互相调用走 crate 内路径，对外只开重导出的小口。

> 📌 面试话术：crate 是"发布/编译单元"，module 是"命名空间/可见性单元"；`mod` 建模块、`pub` 控可见性、`use` 简路径、`pub use` 定 API——四件套讲全即是工程化答案。

## 4. Cargo 工具链与常用命令

**Q：日常开发中最常用的 cargo 命令有哪些？**

| 命令 | 作用 | 备注 |
|---|---|---|
| `cargo new myapp [--lib]` | 创建二进制/库项目 | `--bin` 为默认 |
| `cargo run [--release]` | 编译并运行 | 默认 dev profile（快编译、慢运行） |
| `cargo build [--release]` | 只编译不运行 | release 做优化（慢编译、快运行） |
| `cargo check` | 只类型检查不生成代码 | **增量开发神器**，速度比 build 快数倍 |
| `cargo test [name]` | 运行测试（单测 + 集成测试 + doc test） | `-- --nocapture` 显示 println 输出 |
| `cargo clippy` | Lint 检查 | 配 `-D warnings` 进 CI 卡口 |
| `cargo fmt` | 代码格式化（rustfmt） | `cargo fmt --check` 用于 CI |
| `cargo doc --open` | 生成并打开文档 | 含所有依赖的文档 |
| `cargo clean` | 清理 target 目录 | |
| `cargo add/remove <dep>` | 增删依赖 | cargo-edit 已内置 |
| `cargo tree` | 查看依赖树 | 排查依赖冲突利器 |
| `cargo update` | 按语义化版本升级依赖 | 只动 Cargo.lock 的小版本 |
| `cargo install <tool>` | 安装二进制工具到 `~/.cargo/bin` | 如 `cargo-watch`、`cargo-audit` |
| `cargo bench` | 运行基准测试 | 配 criterion.rs |

> 💡 资深岗加分：`cargo check` 与 `cargo build` 的差别（不生成机器码、不做单态化代码生成）是"为什么 Rust 检查快"的回答核心。

**Q：什么是 workspace？什么场景使用？**

一个仓库管理多个 crate 的机制，根 `Cargo.toml` 声明成员：

```toml
[workspace]
members = ["crates/logger", "crates/server", "crates/common"]
resolver = "2"   # 统一依赖解析器版本（2021 edition 推荐显式声明）
```

- 成员间用**路径依赖**互相引用：`logger = { path = "../logger" }`；
- 整个 workspace **共享一个 Cargo.lock**，保证所有 crate 用同一版本依赖；
- 根目录一条 `cargo test` / `cargo clippy --workspace` 跑全部成员；
- 适用场景：大型项目按业务拆 crate（编译隔离、复用发布）、monorepo。

**Q：feature 与可选依赖怎么用？**

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", optional = true }        # 可选依赖

[features]
default = []
json = ["dep:serde"]                               # 显式 feature（推荐 dep: 写法）
```

- `cargo build --features json` 条件启用；`default` 特性默认开；
- `dep:` 前缀避免隐式 feature 名泄漏；
- 常见考点：tokio 的 `full` vs 按需 `rt-multi-thread + macros + ...`，缩小编译时间和二进制体积。

**Q：如何做交叉编译？**

```bash
rustup target add x86_64-unknown-linux-musl   # 1. 安装目标平台标准库
cargo build --release --target x86_64-unknown-linux-musl  # 2. 指定 target 构建
```

- musl target 静态链接，产出无 glibc 依赖的单二进制，容器/Docker 部署首选；
- `.cargo/config.toml` 可配置默认 target、链接器（`[build]` / `[target.xxx]`）。

**Q：CI 中标准的 cargo 检查流水线是什么？**

```bash
cargo fmt --check && cargo clippy --all-targets -D warnings && cargo test --workspace
```

再加 `cargo audit`（检查依赖 CVE）和 `cargo deny`（许可证/依赖策略），即完整工程卡口。

## 5. rustup 工具与交叉编译

**Q：rustup 是什么？怎么管理 Rust 工具链？**

rustup 是 Rust 官方工具链管理器，负责安装、更新、切换 Rust 版本和平台 target：

```bash
# 安装最新稳定版
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装指定版本（--default-toolchain 指定）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain=1.94.0

# 升级到最新稳定版
rustup update stable

# 查看已安装工具链 / 切换默认工具链
rustup show
rustup default nightly          # 或 stable / 具体版本号

# 查看组件（rustfmt、clippy、rust-src、rust-analyzer 等）
rustup component list --installed
rustup component add rust-src rust-analyzer
```

常用场景：团队锁定某个 `rust-toolchain.toml`（项目根目录）统一版本，成员 `cd` 进去 rustup 自动切换到该版本，杜绝"我本地能编译、CI 挂"的版本漂移。

**Q：国内如何加速 rustup 与 crates.io？**

```bash
# rustup 走镜像（写进 ~/.zshrc 或 ~/.bash_profile）
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

```toml
# ~/.cargo/config.toml：crates.io 走镜像
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"

# 备选：rsproxy（字节）、tuna（清华）、sjtu（上海交大）
```

**Q：什么是交叉编译？怎么做？**

在**当前平台**编译出**另一平台/架构**可执行文件的机制，前提是安装对应 target 的标准库：

```bash
# 1. 查看所有支持的 target
rustup target list

# 2. 添加目标平台工具链
rustup target add x86_64-unknown-linux-musl

# 3. 指定 target 编译
cargo build --release --target x86_64-unknown-linux-musl
```

不同平台需要配不同链接器（`.cargo/config.toml`）：

```toml
# macOS 编译 Windows（需 brew install mingw-w64）
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"

# macOS 编译 Linux musl（静态链接，无 glibc 依赖）
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
```

**Q：musl 与 gnu 怎么选？（部署常见考点）**

| | x86_64-unknown-linux-**musl** | x86_64-unknown-linux-**gnu** |
|---|---|---|
| 链接 | 静态（musl libc 编进二进制） | 动态（依赖系统 glibc） |
| 部署 | **单二进制，无任何依赖**，Docker/老系统通吃 | 需目标机有兼容 glibc 版本 |
| 体积 | 更小 | 稍大 |
| 适用 | 容器化、跨发行版分发 | 目标机明确、同发行版部署 |

> 📌 推荐：纯 Rust 服务**默认 musl** 静态编译，彻底规避 glibc 版本不兼容；FFI 调 C/C++ 时交叉编译变复杂，需补目标平台依赖。

**Q：cross 工具是什么？什么场景用？**

[`cross`](https://github.com/cross-rs/cross) 用 Docker 封装交叉编译环境，免去手工装目标平台工具链和链接器：

```bash
cargo install cross

cross build --target aarch64-unknown-linux-gnu   # ARM 服务器
cross build --target x86_64-unknown-linux-musl
cross run   --target x86_64-unknown-linux-musl   # 编译并运行
```

- 优点：Docker 保证构建环境一致、稳定，解决多平台依赖问题；
- 代价：需要 Docker 环境，构建比本地稍慢；
- **Windows 交叉编译**另选 `cargo-xwin`：`cargo xwin build --target x86_64-pc-windows-msvc`（用 MSVC 工具链，无需 mingw）。

> 📌 选型总结：纯 Rust 用 musl/cross 都行；调 FFI 才需要仔细配目标依赖；Windows 编译优先 cargo-xwin 或 cross。

## 6. 条件编译与 build.rs

- `#[cfg]` 属性按条件选择性编译（平台、feature、debug_assertions 等）；
- **build.rs** 构建脚本：编译前执行，用于生成代码、设置环境变量、编译外部依赖（如 C 库）、配置构建选项。
  - 实战用法：注入 git commit / 构建时间到二进制，方便线上定位版本。

## 7. 标准库、I/O 与网络

- 标准库随安装打包：I/O、集合、线程、网络等核心功能；`std::io` 处理输入输出；
- `std::net`：TCP/UDP socket、listener、IPv4/IPv6；
- HTTP 生态：`hyper`（底层）、`reqwest`（客户端）、`axum`/`actix-web`/`rocket`/`warp`（框架）。

## 8. Web 开发与数据库

- Web：异步非阻塞（高并发）、内存安全（无空指针/泄漏）、跨平台编译（单静态二进制部署）；
- 数据库：
  - **sqlx**：编译期检查 SQL、纯异步、支持 PostgreSQL/MySQL/SQLite；
  - **Diesel**：类型安全 ORM + 查询构建器；
  - **tokio-postgres**：PostgreSQL 异步驱动。

## 9. Unsafe 代码管理与 FFI

**Q：Rust 的安全保证是从哪来的？（安全模型总述）**

- **借用检查器**（rustc 组成部分，**编译期**验证引用，非运行时 GC/引用计数）：
  - 同一时刻最多一个可变引用 `&mut T`，或任意多个不可变引用 `&T`，两者永不重叠 → 杜绝数据竞争与悬垂引用；
- **RAII**：所有权追踪变量作用域，离开作用域自动释放 → 无泄漏、无 double free；
- **零运行时开销**：以上保证全部在编译期完成，不付出 GC 的性能代价（对比：C/C++ 半手动管理、Java/Go/Python 自动 GC）；
- 并发安全是同一套规则的副产品：数据竞争的本质是"同时存在多个可变共享引用"，唯一可变引用规则从根上杜绝；
- 经验法则：**代码能通过编译 + 充分测试 ≈ 不崩溃**（缓冲区越界、内存分配不当等 C 系经典错误被消灭）。

**Q：unsafe 是什么？能做什么？**

Rust 允许但严格封装 unsafe——编译器在此**暂停部分检查**，由开发者用 `unsafe` 块显式承诺"这里的安全性由我负责"：

- 解引用原始指针（`*const T` / `*mut T`）；
- 调用 unsafe 函数 / 访问 unsafe trait；
- 修改可变静态变量（`static mut`）；
- 实现 `Send` / `Sync` 等 unsafe trait；
- `union` 字段访问、`std::mem::transmute` 类型抹除。

```rust
unsafe impl Send for MyStruct {}   // 手动承诺线程安全
unsafe { *ptr = 1; }               // 裸指针解引用
```

**Q：unsafe 的使用原则是什么？（面试官想听的工程态度）**

1. **最小化范围**：`unsafe` 块越窄越好，能包三行绝不包整个函数；
2. **安全抽象封装**：unsafe 细节藏进私有函数/模块，对外暴露安全 API，"安全性由封装者负责、由调用者免费获得"；
3. **不变量写注释**：unsafe 块依赖的前置/后置条件必须注释（审计和 review 的依据）；
4. **能用 safe 就不用 unsafe**：优先用安全抽象（`slice::get_unchecked` → 先考虑 `get`），确有必要才下沉；
5. **审计工具兜底**：`cargo geiger` 统计 unsafe 密度，Miri（`cargo +nightly miri`）检测 UB，CI 卡口。

**Q：如何从 Rust 调用 C 代码？（FFI 基础）**

```rust
// 1. 声明外部函数签名（Rust 2024 edition：extern 块本身也要 unsafe）
use std::ffi::{c_double, c_int};

unsafe extern "C" {
    fn abs(num: c_int) -> c_int;
    fn sqrt(num: c_double) -> c_double;
}

fn main() {
    // 2. 调用必须包在 unsafe 块中——编译器无法验证 C 代码的内存安全
    unsafe {
        println!("{}", abs(-10));
        println!("{}", sqrt(64.0));
    }
}
```

要点：

- `extern "C"` 指定 **C ABI**（调用约定），不同语言间函数调用以此对接；
- `std::ffi` 提供 C 类型映射：`c_int`≈`i32`、`c_double`≈`f64`、`c_char`、`*const/mut T` 裸指针；严格说 `c_int` 只保证"至少 short"，跨平台差异用 `c_int` 而非写死 `i32`；
- **字符串不互通**：C 是 `\0` 结尾的 `char*`，Rust 是 `String`/`&str`（带长度）——互传时用 `std::ffi::CString` 转换，并注意所有权（谁分配谁释放）；
- 编译 C 代码用 **`cc` crate + build.rs**：`cc::Build::new().file("c_code/foo.c").compile("foo")` 产出静态库，Rust 侧 `#[link(name = "foo")]` 关联（呼应第 5 节 build.rs）。

**Q：如何把 Rust 库暴露给 Python 等语言调用？（反向 FFI）**

用 [pyo3](https://github.com/PyO3/pyo3) 把 Rust 函数编译成 Python 原生扩展模块（配合 maturin 构建）：

```rust
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: i64, b: i64) -> String { (a + b).to_string() }

#[pymodule]
fn string_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

```shell
python3 -m venv .env && source .env/bin/activate && pip3 install maturin
maturin develop          # 编译并安装为 python 模块
python3 -c "import string_utils; print(string_utils.sum_as_string(1, 2))"
```

pyo3 在宏层面封装了 GIL 管理、类型转换和异常处理——业务代码基本不直接写 unsafe，是"安全抽象封装 unsafe"的范例。同思路的跨语言方案：Node.js 用 `neon`、C++ 互操作用 `cxx` / `autocxx`。

**Q：FFI 调用的安全性注意事项？**

跨语言边界的内存安全编译器管不到，需人工保证：

1. **减少 unsafe 依赖**：优先用提供安全抽象的框架（safer_ffi、rustix 等），让 unsafe 集中在框架内部；
2. **正确管理内存**：约定并遵守"谁分配谁释放"——C 分配的内存必须交回 C 释放（`free`），Rust 侧 `Box::from_raw` 接手的指针必须对应此前的 `Box::into_raw`；否则泄漏或 double free；
3. **防止不当利用**：FFI 入口是攻击面（动态库加载、指针参数），对外暴露的 FFI 函数要当 public API 一样审查和 fuzz 测试；
4. **类型契约**：C 侧结构体布局、对齐、可空指针约定用 `#[repr(C)]` 固定布局，文档写清契约。

## 10. 日志组件（log / env_logger）

**Q：Rust 日志生态的 facade 模式是什么？**

`log` crate 是**日志门面**（只定义宏和接口，不产生输出），`env_logger` 是实现方（把日志写到 stdout/stderr）。业务代码依赖 `log` 宏编程，运行时由具体实现接管——换实现（env_logger / tracing / 自定义）业务零改动。

```text
业务代码 (use log::info)
    → log 宏生成 log::Record（含 level/module/target/key-value）
    → env_logger 接管，按 RUST_LOG 过滤，按格式输出
```

**Q：基本用法与级别控制？**

```rust
use log::{info, warn};

fn main() {
    env_logger::init();          // 读取 RUST_LOG 初始化

    info!("hello, {}", "world");
    warn!(target: "payment", "order paid");  // target：自定义分类标签
}
```

```bash
RUST_LOG=info cargo run               # 全局 info
RUST_LOG=payment=debug cargo run      # 仅 payment target 的 debug
RUST_LOG=payment=debug,notify=warn,info cargo run  # 多规则，后者为兜底
```

级别：`ERROR > WARN > INFO > DEBUG > TRACE`；target 默认等于模块路径，自定义 target 可按子系统独立调级——线上动态排查的标配手段（见第四篇第 10 节）。

**Q：如何输出结构化 JSON 日志？**

启用 `log` 的 `kv` feature 后支持 key/value 语法，配合自定义 format 即可输出 JSON：

```toml
[dependencies]
log = { version = "0.4", features = ["kv"] }
```

```rust
// key/value 在前，分号分隔消息体
info!(order_id = 12345, amount = 99.5; "order paid");

// 自定义 target + key/value
info!(target: "payment", order_id = 12345; "order paid");
```

```json
{"ts":"2026-07-18T14:25:16Z","level":"INFO","module":"my_app::main","target":"payment","caller":7,"msg":"order paid","order_id":12345}
```

工程实践（参考 hera 工具链的 logger 组件设计）：在 `env_logger` 的 `.format()` 上自定义格式化，提取 Record 的 ts/level/module/target/caller/msg 及 kv 拼 JSON；Logger 用**链式配置**（`with_json()` / `with_caller_line()` / `with_time_format()`）暴露配置；`init()`（失败 panic，用于 main）与 `try_init()`（返回 Result，用于测试）分离。组件 crate 用 `pub use log::{debug, error, info, warn}` 把宏重导出到根命名空间，业务侧 `use logger::{Logger, info}` 统一入口、零迁移成本（这正是第 3 节 `pub use` 重导出的实战范例）。

**Q：怎么自己封装一个自定义 Logger？以 hera 的 logger 组件为例**

上面的 JSON 输出不是 env_logger 原生能力，而是**自定义 `format` + 遍历 key/value** 实现的，核心三块：

**① 链式配置的 Logger（Builder 模式）**

```rust
use chrono::Utc;
use std::io::Write;

pub struct Logger {
    caller_line: bool,   // 是否输出行号
    enable_json: bool,   // 是否 JSON 格式
    time_format: String, // 时间格式
}

impl Logger {
    pub fn new() -> Self {
        Self { caller_line: false, enable_json: false,
               time_format: "%Y-%m-%dT%H:%M:%SZ".to_string() }
    }
    pub fn with_caller_line(mut self) -> Self { self.caller_line = true; self }
    pub fn with_json(mut self) -> Self { self.enable_json = true; self }
    pub fn with_time_format(mut self, f: &str) -> Self { self.time_format = f.to_string(); self }

    pub fn init(&self) {
        self.try_init().expect("logger already initialized");
    }
    pub fn try_init(&self) -> Result<(), log::SetLoggerError> {
        let mut builder = env_logger::Builder::from_default_env();
        builder.target(env_logger::Target::Stdout);

        let time_format = self.time_format.clone();
        if self.enable_json {
            // 关键：用 .format() 接管输出，把每条 Record 拼成 JSON
            builder.format(move |buf, record| {
                writeln!(buf, "{}", format_json(record, &time_format))
            });
        } else if self.caller_line {
            // 普通文本 + 行号
            builder.format(move |buf, record| {
                writeln!(buf, "[{} {} {}:{}] {}",
                    Utc::now().format(&time_format), record.level(),
                    record.module_path().unwrap_or("unnamed"),
                    record.line().unwrap_or(0), record.args())
            });
        }
        builder.try_init()
    }
}
```

**② 提取 key/value → JSON（visitor 模式）**

`log::Record` 的 key/value 通过 `log::kv::Source` 暴露，自定义 visitor 遍历并收集进 `serde_json::Map`：

```rust
// 遍历 key/value，收集到 serde_json::Map
impl<'kvs> log::kv::VisitSource<'kvs> for JsonVisitor {
    fn visit_pair(&mut self, key: log::kv::Key, value: log::kv::Value)
        -> Result<(), log::kv::Error> {
        let mut vv = JsonValueVisitor::new();
        value.visit(&mut vv)?;
        if let Some(v) = vv.value {
            self.map.insert(key.to_string(), v);
        }
        Ok(())
    }
}

// 单个 value 按类型转成 serde_json::Value
impl<'v> log::kv::VisitValue<'v> for JsonValueVisitor {
    fn visit_i64(&mut self, v: i64) -> Result<(), log::kv::Error> {
        self.value = Some(serde_json::Value::Number(v.into())); Ok(())
    }
    fn visit_u64(&mut self, v: u64) -> Result<(), log::kv::Error> {
        self.value = Some(serde_json::Value::Number(v.into())); Ok(())
    }
    fn visit_bool(&mut self, v: bool) -> Result<(), log::kv::Error> {
        self.value = Some(serde_json::Value::Bool(v)); Ok(())
    }
    fn visit_str(&mut self, v: &str) -> Result<(), log::kv::Error> {
        self.value = Some(serde_json::Value::String(v.to_string())); Ok(())
    }
    // visit_f64 / visit_char / visit_null / visit_any 同理
}
```

`format_json` 再把 `ts / level / module / target / caller / msg` 固定字段与 visitor 收集的 kv 一起手工拼成 JSON 字符串。

**③ `pub use` 重导出 + 依赖**

```toml
[dependencies]
log = { version = "0.4", features = ["kv"] }
env_logger = "0.11"
chrono = "0.4"      # 时间格式化
serde_json = "1"    # JSON 值类型
```

```rust
// 把 log 宏重导出到 logger 根命名空间，业务侧一个 use 搞定
pub use log::{debug, error, info, warn};
```

```rust
// 业务侧统一入口，零迁移成本
use logger::{Logger, info};

fn main() {
    Logger::new().with_caller_line().with_json().init();
    info!(order_id = 12345, amount = 99.5; "order paid");
}
```

> 📌 设计要点（面试可讲）：
> - **门面 + 实现分离**：组件只依赖 `log` 门面编程，底层 `env_logger` 可随时替换，业务零改动；
> - **Builder 链式配置**：必填无、可选 `with_*` 链式覆盖 + 默认值兜底（呼应第 3 节组件设计）；
> - **`init` vs `try_init`**：`init` 失败 panic（main 用），`try_init` 返回 `Result`（测试用，避免二次初始化 panic）；
> - **`.format()` 是唯一自定义点**：env_logger 把每条 `log::Record` 交给你格式化，JSON/文本/ltsv 都在这一个闭包里切换；
> - **key/value 靠 visitor 收集**：`log::kv::Source` / `VisitValue` 是官方结构化日志入口，按类型转 `serde_json::Value`，天然保留数字/布尔/字符串类型。

**Q：log + env_logger 与 tracing 怎么选？**

| | log + env_logger | tracing |
|---|---|---|
| 定位 | 轻量门面 + 简单输出 | 结构化日志 + span 跟踪 |
| key/value | 支持（kv feature） | 原生，生态更完整 |
| 异步/分布式追踪 | 弱 | 强（OpenTelemetry 集成） |
| 适用 | 中小项目、工具、库 | 服务端、微服务、复杂链路排查 |

> 📌 面试加分：能讲清 facade 模式（门面与实现分离）+ `RUST_LOG` 分级过滤 + JSON 结构化三件事，即覆盖日志题的主干；深入方向见第四篇第 10 节（tracing + OpenTelemetry）。

## 11. 配置读取与序列化（serde 实践）

**Q：serde 是什么？序列化生态怎么组成？**

`serde` 是 Rust 序列化框架核心，只定义 `Serialize`/`Deserialize` trait，格式由具体 crate 实现：

| crate | 格式 | 典型用途 |
|---|---|---|
| serde_json | JSON | API 交互、JSON 配置文件 |
| serde_yaml | YAML | 人类可读的服务配置 |
| serde + toml | TOML | Cargo.toml 同款，Rust 项目常用 |
| bincode / rmp | 二进制 | 高性能 RPC、缓存 |

```rust
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
struct AppConfig {
    app_debug: bool,
    app_name: String,
    app_port: i32,
}
```

**Q：配置文件加载的标准流程是什么？**

读取文件 → 反序列化为强类型结构体（**启动时加载、立即校验、fail-fast**）：

```rust
use serde_yaml::{self, Value};
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};

pub trait ConfigTrait {
    fn load(&mut self) -> Result<(), Error>;
    fn sections(&self) -> Value;      // 动态 Value 视图
    fn content(&self) -> &str;        // 原始文本
}

pub struct Config {
    path: PathBuf,
    sections: String,
}

impl Config {
    // 泛型入参 AsRef<Path>：同时接受 &str、String、PathBuf（呼应第 14 节）
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Config { path: PathBuf::from(path.as_ref()), sections: String::new() }
    }
}

impl ConfigTrait for Config {
    fn load(&mut self) -> Result<(), Error> {
        File::open(&self.path)?.read_to_string(&mut self.sections)?;
        Ok(())
    }
    fn sections(&self) -> Value {
        serde_yaml::from_str(&self.sections).unwrap()
    }
    fn content(&self) -> &str { &self.sections }
}

// 使用：两种反序列化路径
let mut c = Config::new("test.yaml");       // &str 直接传入
c.load().expect("read file failed");

// ① 文本 -> 强类型 struct（推荐：字段名/类型错在启动即报错）
let cfg: AppConfig = serde_yaml::from_str(c.content()).unwrap();

// ② 文本 -> Value -> struct（需要动态访问/合并配置时先走 Value）
let v: Value = c.sections();
let cfg: AppConfig = serde_yaml::from_value(v).unwrap();
```

> 💡 路径 ① vs ②：`from_str` 一步到位、最常用；`Value`（`serde_yaml::Value` / `serde_json::Value`）是动态中间表示，适合做**字段探查、多来源配置合并、缺省补齐**，再用 `from_value` 落到强类型。

**Q：serde 常用属性有哪些？（配置文件工程化核心）**

```rust
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default = "default_app_port")]  // 缺失时用函数返回默认值
    pub app_port: u16,

    #[serde(default)]                        // 缺失时用该类型的 Default
    pub token: String,

    #[serde(rename = "appName")]            // 配置键名与 Rust 字段名解耦
    pub app_name: String,

    #[serde(skip_serializing_if = "String::is_empty")] // 序列化时条件跳过
    pub secret: String,
}

fn default_app_port() -> u16 { 1338 }
```

- `default` / `default = "fn"`：配置字段可缺省，避免"加配置项必须改所有环境文件"；
- `rename`：对接外部系统命名规范（camelCase 配置 vs snake_case 字段）；
- `deny_unknown_fields`：严格模式，配置写错字段名直接报错，防止"配了但没生效"。

**Q：配置加载的工程实践要点？**

1. **启动即加载、fail-fast**：配置错误应在进程启动时报错退出，而不是运行到一半才 panic；
2. **强类型优先**：用 struct 承接配置，键名/类型/取值范围由 serde 在启动时校验；
3. **默认值策略**：新增配置项配 `default`，老环境文件零改动兼容；
4. **环境覆盖**：配置文件中放默认值，环境变量做覆盖（或直接用 `config` crate 聚合多来源）；
5. **密钥不进配置文件**：密码/token 走环境变量或密钥管理服务，配置文件只放引用名；
6. **配置热更新**：长驻进程可定时重读 + `ArcSwap`/`RwLock` 原子替换，注意校验失败保留旧值。

> 📌 面试话术：serde 三段论——derive 定契约（结构体即 schema）、serde_json/serde_yaml 做格式、属性做工程化（默认值/重命名/严格校验）；配置系统的目标一句话：**启动时把"配置错了"变成"启动失败"**。

## 12. 文档（Rustdoc）

`///` 文档注释 + `cargo doc` 生成 HTML；支持代码示例自动测试（doc test）。标准库文档即 Rustdoc 产物。

## 13. 基础设施组件库封装（hera 实践）

**Q：团队为什么要封装自己的基础设施组件库？**

直接答案：**把"每个服务重复造轮子"变成"一次封装、处处复用"**。以 [hera](https://github.com/rs-god/hera)（Rust 基础设施组件库）为例，它用 Workspace 多 crate 架构把通用能力沉淀为独立组件：

| 组件 | 定位 | 核心能力 |
|---|---|---|
| `config` | 配置读取 | YAML 配置加载，`ConfigTrait` 抽象（见第 10 节） |
| `crypto` | 加解密 | AES-128/192/256 CBC + PKCS7，Base64 输出，内置 key/iv 生成 |
| `logger` | 日志 | 文本/JSON 双格式，log kv 语法，caller 行号（见第 9 节） |
| `monitor` | 监控 | 基于 `autometrics` 函数级 Prometheus 指标 + SLO + `/metrics`、`/check` 端点 |
| `shutdown` | 平滑退出 | 监听 Ctrl+C / SIGTERM，跨平台 |
| `xmysql` | MySQL | 基于 `sqlx` 的异步连接池（max/min/idle/lifetime 可配） |
| `xredis` | Redis | 单节点/集群连接池，同步与异步 |
| `xpulsar` | Pulsar | 异步 Producer/Consumer 封装，Token 认证 |

**Q：组件库的引入与版本管理方式？**

```toml
[dependencies]
logger = { git = "https://github.com/rs-god/hera.git", tag = "v1.2.6" }
config = { git = "https://github.com/rs-god/hera.git", tag = "v1.2.6" }
```

- **git + tag 精确锁定**：公司内部组件库不走 crates.io，用 git 依赖 + tag 保证可复现构建；
- 语义化 tag 发版（v1.2.6），业务侧按需升级单组件；
- 对比第 1 节：外部 git 依赖同样进 Cargo.lock 锁定 commit。

**Q：封装组件库的核心设计模式有哪些？（面试可讲的设计点）**

1. **Builder 链式配置**：`MysqlConf::new(dsn).with_max_connections(10).init_pool().await`——必填项进 `new`，可选项 `with_*` 链式覆盖，默认值兜底；
2. **Trait 抽象能力面**：`ConfigTrait`（load/sections/content）定义能力契约，调用方依赖 trait 而非具体类型，便于替换实现与测试 mock；
3. **`pub use` 统一入口**：`pub use log::{debug, error, info, warn}` 把外部宏重导出到组件根命名空间，业务 `use logger::{Logger, info}` 一处引入（第 3 节重导出的实战范例）；
4. **门面 + 实现分离**：logger 基于 log 门面，monitor 基于 autometrics——组件库自身也是"薄封装"，换底层实现不影响业务；
5. **跨平台与默认值兜底**：shutdown 兼容 Unix/Windows；连接池参数给生产合理默认值，业务侧只改差异项。

```rust
// 平滑退出与 axum 集成的典型用法
axum::serve(listener, router)
    .with_graceful_shutdown(graceful_shutdown(Duration::from_secs(5)))
    .await
    .unwrap();
```

> 📌 面试话术：组件库的价值 = 统一团队技术选型 + 收敛最佳实践（默认值/SLO/日志格式）+ 降低新业务启动成本（git 依赖一行引入）；设计要点 = 单一职责 crate + Builder + trait 抽象 + 门面模式。

## 14. 项目实战：Web 开发、gRPC 微服务与 BFF 网关

**Q：Rust Web 服务的工程化分层怎么设计？（以 [rs-api](https://github.com/daheige/rs-api) 为例）**

技术栈：axum 0.8 + tokio + sqlx(MySQL) + redis/r2d2 + validator + thiserror/anyhow + `std::sync::LazyLock` 全局配置。

```text
          HTTP Request
               │
   middleware  ← access_log（注入 x-request-id、记耗时）/ no_cache_header
               ▼
   routes     ← 路由匹配、/api 分组、fallback 404
               ▼
   handlers   ← 提取参数(Path/State/ValidatedForm)、封装统一响应 {code,message,data}
               ▼
   services   ← 业务编排：先查 Redis 缓存，未命中查 MySQL，回写缓存
               ▼
   infras     ← sqlx 连接池 / r2d2 Redis 池
   entity     ← 请求 DTO / 响应 DTO / FromRow 模型（跨层传递的数据契约）
```

工程实践要点：

- **启动时全局加载配置**：`LazyLock` 读 `app.yaml`（支持 `CONFIG_DIR` 环境变量），全局唯一，连接池随配置初始化；
- **统一响应封装**：所有接口返回 `{code, message, data}`，handler 不直接抛错误；
- **自定义提取器**：`ValidatedForm` / `JsonOrForm` 基于 `validator` 做参数校验，校验失败统一 400；
- **缓存策略**：`service` 层 get_cache → miss 查库 → set_cache，缓存穿透由这一层兜底；
- **优雅关闭**：`with_graceful_shutdown` 监听信号，等待 `graceful_wait_time` 后退出；
- **独立 job 入口**：同一份配置/连接池封装可复用到非 Web 的定时任务二进制。

**Q：Rust gRPC 微服务怎么落地？（以 [rs-grpc](https://github.com/daheige/rs-grpc) 为例）**

技术栈：tonic + prost + tokio，build.rs 中 `tonic_prost_build` 编译 proto 生成代码。

```rust
// 1. proto 定义（proto/hello.proto，含 google.api.http 注解）
service Greeter {
    rpc Healthz(HealthzReq) returns (HealthzReply);
    rpc SayHello(HelloReq) returns (HelloReply);
}

// 2. build.rs：编译 proto + 生成 file descriptor（供 gRPC Reflection）
tonic_prost_build::configure()
    .file_descriptor_set_path(&descriptor_path)
    .out_dir(out_dir)
    .compile_protos(&file_list, &[proto_dir])?;

// 3. 服务端实现生成的 trait
#[async_trait::async_trait]
impl Greeter for GreeterImpl {
    async fn say_hello(&self, request: Request<HelloReq>)
        -> Result<Response<HelloReply>, Status> {
        let req = request.into_inner();
        Ok(Response::new(HelloReply { message: format!("hello,{}", req.name) }))
    }
}

// 4. 启动 Server + Reflection（grpcurl 动态发现 proto）
Server::builder()
    .add_service(GreeterServer::new(greeter_impl))
    .add_service(reflection_service)   // server_reflection_v1
    .serve(addr).await?;
```

实战架构模式：

| 模式 | 说明 | 场景 |
|---|---|---|
| 纯 gRPC 服务 | 默认端口 50051 + `/metrics` 独立端口 | 服务间通信 |
| **HTTP Gateway** | axum 网关把 HTTP/JSON 请求转成 gRPC 调用 | 对前端/外部暴露 REST |
| **Multiplex 单端口** | `tower::steer` 按 Content-Type 把 gRPC 与 HTTP 路由到同端口 | 减少端口占用、统一入口 |
| 多语言客户端 | Go/Node.js 客户端 + 各自代码生成脚本 | 跨语言生态对接 |

工程要点：

- **proto 即契约**：PB 定义托管在仓库 `proto/` 目录，build.rs 编译期生成 Rust 代码，类型安全从源头保证；
- **gRPC Reflection**：注册反射服务后 `grpcurl -plaintext localhost:50051 list` 即可动态调试，无需手写 proto 路径；
- **可观测接入**：`#[autometrics(objective = API_SLO)]` 注解在 RPC 方法上自动产出调用数/延迟/成功率指标；
- **基础设施复用**：配置（`config`）、日志（`logger`）、平滑退出（`shutdown`）、监控（`monitor`）直接引用 hera 组件库（第 12 节）——**真实项目就是组件库的消费方**；
- **容器化**：Dockerfile 多阶段构建（builder + runtime），Makefile 一键构建，musl 静态二进制部署。

**Q：什么是 BFF 架构？Rust 怎么实现 BFF 网关？（以 [rs-bff](https://github.com/daheige/rs-bff) 为例）**

BFF（Backend for Frontend）：对前端暴露聚合后的 HTTP API，对内以 gRPC 调用各后端微服务，完成 JSON ↔ Protobuf 协议转换。rs-bff 的三层结构与职责：

```text
interfaces  ← HTTP handler + 路由，依赖 AppState 调下游
providers   ← 外部连接管理（gRPC client），组装 AppState
infra       ← 配置解析、统一错误类型、AppState 定义
```

关键技术决策（面试重点）：

1. **gRPC 客户端懒加载**：`GrpcClientManager` 用 `tokio::sync::OnceCell<GreeterClient<Channel>>`——启动时不建连，首次调用自动初始化并缓存。收益：启动快、后端未就绪不阻塞 BFF 启动；代价：首次请求承担一次握手 RTT；
2. **Channel 而非直连**：`Endpoint::connect()` 返回的 `Channel` 内部由后台任务维护连接，支持 HTTP/2 多路复用；`Clone` 成本极低（内部 `Arc`），handler 每次取独立克隆实例，无锁竞争；可精确配置 TLS/超时/并发限制/拦截器/负载均衡；
3. **连接保活与超时对齐**：30s HTTP/2 心跳 + 20s 超时 + `keep_alive_while_idle`，单次 RPC `timeout(30s)`、`connect_timeout(10s)`——**超时阈值须与后端 P99 对齐**，过短误杀、过长失去熔断意义；
4. **错误分层映射**：统一 `AppError`（Grpc / GrpcTransport / Serialization / Internal）实现 `IntoResponse`，gRPC Code 自动转 HTTP 状态码（NotFound→404、Unavailable→503、DeadlineExceeded→504……），handler 无需关心状态码细节；
5. **配置禁止默认值**：服务地址缺失直接报错退出（fail-fast，呼应第 10 节）。

**Q：PB 协议为什么要独立仓库托管？（以 [hello-pb](https://github.com/daheige/hello-pb) 为例）**

多服务共享 protobuf 契约时的标准做法——**协议即产品，单独发版**：

```text
hello-pb（协议仓库）                    业务服务（rs-bff / rs-grpc / go-svc）
├── protos/hello.proto  ──build.rs──▶  ├── hello-pb = { git=..., tag="v1.1.2" }
├── build.rs（tonic-prost-build 生成）  └── 直接 use hello_pb::hello::...
├── src/hello.rs（生成的 Rust 代码）      无需各自维护 proto 和生成脚本
└── clients/（go / nodejs / php 生成物）  跨语言版本天然一致
```

- **一处生成，多处消费**：Rust/Go/Node.js/PHP 生成物集中在协议仓，业务服务只做 git + tag 依赖（呼应第 12 节组件库引入方式）；
- **版本即契约**：tag（v1.1.2）语义化发版，升级协议是显式动作，避免"各服务生成代码版本不一致"的经典事故；
- **业务仓零 proto**：服务侧不再放 proto 文件和生成脚本（rs-bff 演进记录：本地 build.rs 生成 → 外部 hello-pb 依赖的混合模式）；
- **CI 友好**：协议变更走独立仓库的 review 与 tag 发布流程，消费方升级依赖即生效。

> 📌 面试话术：微服务接口契约的管理 = **协议仓库 + 语义化 tag + git 依赖消费**；对比"每个服务自己生成"的模式，独立托管解决了版本漂移、重复脚本、跨语言不一致三个问题。

> 📌 面试总结：Web/gRPC 实战的考察重点不在框架 API，而在**分层与依赖方向**（handler 不碰存储、service 编排缓存与事务）、**统一契约**（响应封装/错误处理/proto 契约）、**可观测三件套**（日志/metrics/追踪）、**优雅生命周期**（配置启动加载 + 信号平滑退出）。

---

# 四、线上 Debug 实战

> 核心思路：**平时埋好钩子，出事时能拿到足够信息**。
> 黄金三件套：**现场（crash/卡死/性能） + 符号（symbols） + 工具（gdb/perf 等）**。
> 线上 Debug 的能力，取决于发布前是否埋好了「符号 + 现场 + 日志」三件事。

## 1. 核心思路

```text
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

```text
告警 → 确认版本(commit hash) → 保现场(core/gdb/perf) → 取对应符号
→ 符号化(gdb bt full / perf+rustfilt / tokio-console) → 定位根因
→ 写复现用例 → 修复 → 预发 ASan 回归 → 复盘更新 Checklist
```

---

# 五、面试答题建议

1. **先给定义，再讲机制，最后上例子**。例如可变引用：定义 → 唯一性规则 → 一段代码。
2. **主动对比**：Copy vs Clone、Rc vs Arc、impl Trait vs dyn Trait、`std::sync::Mutex` vs `tokio::sync::Mutex`——对比是面试官最想听的。
3. **说清编译期 vs 运行时**：Rust 的大部分保证在编译期（零成本抽象：泛型单态化、借用检查、生命周期标注全部零运行时开销），死锁/竞争条件是运行时要靠工具排查的。
4. **生命周期讲透三条**：防止悬垂引用是目的；标注不改变生命周期、只是声明约束（返回值 = 入参交集）；elision 三条省略规则——能徒手推 `longest<'a>` 的推导过程即达标。
5. **异步必背三板斧**：Future 惰性 + poll/wake 模型；async 是状态机语法糖；Pin 防自引用结构体被移动。资深岗加背两条：Waker/RawWaker 的 data + vtable 结构、poll 返回 Pending 后的唤醒闭环。
6. **并发题主动下沉一层**：`thread::spawn` / `tokio::spawn` 的 `FnOnce/Future + Send + 'static` 三约束能逐条讲原因；线程默认栈 2MB vs 主线程 8MB；mpsc 的 Sender 可克隆而 Receiver 唯一——这些细节是中高级与资深的分水岭。
7. **工程题讲实战**：线上 Debug 题直接背"黄金三件套"和发布 Checklist，有真实案例更好。
8. **承认边界**：unsafe、宏的复杂度、`static mut` 的坑——知道什么不该做，比知道能做什么更加分。
