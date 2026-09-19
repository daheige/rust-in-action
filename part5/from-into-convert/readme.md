# From 和 Into trait

> 📦 本文介绍 Rust 中最常用的类型转换 trait：`From` 与 `Into`，包括它们的定义、关系、惯用法、泛型应用、错误转换以及与 `AsRef`/`Cow` 的取舍。

## 目录

- [From 和 Into trait](#from-和-into-trait)
  - [目录](#目录)
  - [trait 定义](#trait-定义)
  - [为什么实现一个，另一个自动可用](#为什么实现一个另一个自动可用)
  - [惯例：只实现 From](#惯例只实现-from)
  - [在泛型入参中使用（最常见的 API 设计模式）](#在泛型入参中使用最常见的-api-设计模式)
  - [Into 作为返回值](#into-作为返回值)
  - [与 ? 运算符结合（错误转换）](#与--运算符结合错误转换)
  - [一个类型可以有多个 From 实现](#一个类型可以有多个-from-实现)
  - [为外部类型实现 From：孤儿规则](#为外部类型实现-from孤儿规则)
  - [转换语义约定：From 不应失败](#转换语义约定from-不应失败)
  - [标准库预置的常用转换](#标准库预置的常用转换)
  - [From/Into 与 AsRef/AsMut 的选择](#frominto-与-asrefasmut-的选择)
  - [小结](#小结)

## trait 定义

```rust
// std::convert
pub trait From<T>: Sized {
    /// Performs the conversion.
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    /// Performs the conversion.
    fn into(self) -> T;
}
```

两个要点：

- **类型参数的位置不同**：`From<T>` 的 `T` 是来源，`Self` 是目标；`Into<T>` 的 `T` 是目标，`Self` 是来源。所以 `impl From<A> for B` 和 `impl Into<B> for A` 表达的是同一次转换。
- **两者的 `Self` 都是 `Sized`**，意味着它们只用于具体的、有固定大小的类型。

## 为什么实现一个，另一个自动可用

标准库中有两条关键的 **blanket impl**（全覆盖实现）：

```rust
// 来源：标准库源码
impl<T, U> From<U> for T
where
    U: Into<T>
{ ... }

impl<T, U> Into<U> for T
where
    U: From<T>
{ ... }
```

这意味着：

- 你写了 `impl From<A> for B` → 编译器自动得到 `B: From<A>` 和 `A: Into<B>`
- 你写了 `impl Into<B> for A` → 编译器自动得到 `B: From<A>` 和 `A: Into<B>`

```rust
struct Wrapper(String);

// 只实现这一个：
impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Wrapper(s)
    }
}

// 以下全部自动可用：
fn main() {
    let w1 = Wrapper::from(String::from("a")); // From
    let w2: Wrapper = String::from("b").into(); // Into
    assert_eq!(w1.0, "a");
    assert_eq!(w2.0, "b");
}
```

## 惯例：只实现 From

原因：

- `From` 的写法更符合"**给目标类型添加一个构造来源**"的直觉；
- 文档和 API 使用习惯都倾向于 `From`；
- 泛型代码中 `where T: From<X>` 读起来比 `where X: Into<T>` 顺。

## 在泛型入参中使用（最常见的 API 设计模式）

```rust
// 接受 &str、String、Cow<'_, str> 等所有能转成 String 的类型
fn set_title<S: Into<String>>(title: S) { ... }

// 更简洁的 impl Trait 写法，效果相同
fn set_title2(title: impl Into<String>) { ... }

set_title("hello");                          // &str
set_title(String::from("hi"));               // String
set_title(std::borrow::Cow::Borrowed("c"));  // Cow
```

标准库里大量 API 采用这种设计，例如：

```rust
impl std::fs::File {
    // 用的是 AsRef<Path>，思路一致：让调用方少写转换
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> { ... }
}

std::thread::Builder::new()
    .name("worker".to_string())  // name 接受 Into<String>
    .spawn(...);
```

## Into 作为返回值

`Into` 也可以约束返回值，让一个函数返回"某种拥有所有权的目标类型"：

```rust
fn owned<T: Into<String>>(s: T) -> String {
    s.into()
}
```

> 💡 不过实际中返回 `Cow<'a, str>` 或具体类型更常见，`Into` 主要用于入参。

## 与 ? 运算符结合（错误转换）

`?` 在返回 `Result<T, E>` 的函数中遇到 `Result<T, OtherErr>` 时，会调用 `From::from` 把错误转成 `E`：

```rust
#[derive(Debug)]
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

> 💡 这也是 `thiserror` 等错误处理库 `#[from]` 属性背后的机制。

## 一个类型可以有多个 From 实现

`From` 的类型参数位置决定了同一个类型可以实现任意多个来源：

```rust
struct UserId(u64);

impl From<u64> for UserId {
    fn from(v: u64) -> Self { UserId(v) }
}

impl From<String> for UserId {
    fn from(s: String) -> Self { UserId(s.parse().unwrap_or(0)) }
}

let id1 = UserId::from(42u64);
let id2 = UserId::from(String::from("100"));
```

`Into` 同理：`String: Into<UserId>` 和 `u64: Into<UserId>` 可以同时成立。

## 为外部类型实现 From：孤儿规则

Rust 的**孤儿规则**（orphan rule）规定：你只能在"trait 或类型至少有一个是你自己定义的"情况下写 impl。`From` 是标准库的 trait，所以：

```rust
// ✅ 可以：自己的类型 MyType，任意来源
impl From<i32> for MyType { ... }

// ❌ 不行：String 和 u32 都不是你定义的
// impl From<u32> for String { ... }  // 编译错误
```

绕过的方式之一是**新类型模式**（newtype）：

```rust
struct Kilometers(u32);

impl From<u32> for Kilometers { ... }
```

## 转换语义约定：From 不应失败

标准库文档明确约定：`From` 的转换应该是**无信息的、不会失败的**。

- ✅ 有损但恒定的转换（如 `f64 -> f32`，精度可能下降但永远有结果）允许；
- ❌ 可能失败的转换（如字符串解析）应该用 `TryFrom` / `TryInto`：

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

`?` 同样能用于 `TryFrom` 的错误转换。`TryFrom`/`TryInto` 自 Rust 1.34 起稳定（2021 edition 中已无需 `use std::convert::TryFrom`）。

## 标准库预置的常用转换

```rust
// 字符串
let s = String::from("lit");        // &str -> String
let s: String = "lit".into();
let b: Box<str> = "lit".into();
let rc: std::rc::Rc<str> = "lit".into();

// 数字（仅无损方向：小 -> 大）
let x: u32 = 255u8.into();
let y: i64 = 1000i32.into();
// let z: i32 = y.into();           // 编译错误：i64 -> i32 有损

// 指针/集合
let v: Vec<u8> = [1u8, 2, 3].into();
let arr: [u8; 3] = [1, 2, 3];
let v2: Vec<u8> = Vec::from(arr);
```

## From/Into 与 AsRef/AsMut 的选择

| 需求 | 用哪个 |
| ---- | ---- |
| 函数需要**拥有**数据（可能存储、可能修改） | `impl Into<String>` / `From` |
| 函数只需要**读引用**，不接管所有权 | `impl AsRef<str>` / `impl AsRef<Path>` |
| 函数需要**可变引用** | `impl AsMut<T>` |
| 函数返回"借用或拥有"之一 | `Cow<'a, str>` |

一个反例说明区别：

```rust
// ❌ 不合理：为了读一下就把所有权拿走，调用方被迫 clone
fn print_bad(s: impl Into<String>) { println!("{}", s.into()); }

// ✅ 合理：只借引用
fn print_good(s: impl AsRef<str>) { println!("{}", s.as_ref()); }
```

> 📌 **判断口诀：转换用 From/Into，借用用 AsRef/AsMut。**

## 小结

- `From<T> for U` 与 `Into<U> for T` 互为镜像，实现其一另一个自动可用；
- 惯例是实现 `From`，获得方用 `.into()` 或 `T::from(x)`；
- 泛型函数入参用 `impl Into<T>` 可以让调用方传多种类型；
- `From` 用于不失败的转换，可能失败的用 `TryFrom`/`TryInto`；
- 需要引用而非所有权时改用 `AsRef`，返回"借用或拥有"时用 `Cow`。
