# Rust 中 Cow（Clone-on-Write）的用法

Cow 是标准库 std::borrow 中的一个智能指针枚举，核心思想是：能借用就借用，必须修改时才克隆。

```rust
pub enum Cow<'a, B: ?Sized + 'a>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

它有两种状态：

- Cow::Borrowed(&'a B)：零成本借用
- Cow::Owned(...)：拥有克隆出来的数据

常见形式是 Cow<'a, str>（写字符串处理 API 的经典选择）和 Cow<'a, [T]>

```rust
use std::borrow::Cow;

fn shout(input: &str) -> Cow<'_, str> {
    if input.chars().any(|c| c.is_lowercase()) {
        // 需要修改 -> 克隆一份并转为大写（Owned）
        Cow::Owned(input.to_uppercase())
    } else {
        // 不需要修改 -> 直接借用（Borrowed）
        Cow::Borrowed(input)
    }
}

fn main() {
    let a = shout("hello");   // Owned("HELLO")
    let b = shout("WORLD");   // Borrowed("WORLD")
    println!("{a} {b}");      // HELLO WORLD
}
```

调用方拿到的都是 Cow<str>，按需统一处理，调用者无需关心内部是借用还是拥有。

## Cow 的关键行为：deref

Cow 实现了 Deref，所以 &Cow<str> 可以像 &str 一样使用，无需先解包：

```rust
fn takes_str(s: &str) { println!("{s}"); }

let c: Cow<str> = Cow::Borrowed("hi");
takes_str( & c);               // 自动 Deref
println!("{}", c.len());     // 直接用 str 的方法
```

## 写操作时自动升级（写时克隆）

对 Cow 做可变操作时（比如 to_mut()），如果是 Borrowed 状态，会自动克隆并转为 Owned：

```rust
let mut c: Cow<str> = Cow::Borrowed("hello");
c.to_mut().make_uppercase();  // 触发克隆，变为 Owned
assert!(matches!(c, Cow::Owned(_)));
```

常用方法：

| 方法                                 | 作用                       |
|------------------------------------|--------------------------|
| `cow.to_mut()`                     | 拿到 `&mut Owned`，必要时自动克隆  |
| `cow.into_owned()`                 | 消费自身，得到 `Owned` 值（必要时克隆） |
| `cow.is_borrowed()` / `is_owned()` | 判断当前状态                   |

## 典型场景：字符串规范化处理

这是 Cow 最经典的用途——"大多数情况下不需要改，偶尔需要改"：

```rust
use std::borrow::Cow;

fn trim_and_normalize(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim();
    if trimmed.contains("  ") {
        // 有多余空格，需要处理 -> 克隆
        Cow::Owned(trimmed.split_whitespace().collect::<Vec<_>>().join(" "))
    } else {
        // 已经正常 -> 零成本借用
        Cow::Borrowed(trimmed)
    }
}
```

## 自定义类型配合 Cow

要让 Cow<'a, MyType> 能用，类型需要实现 ToOwned（通常 Clone 就够，因为标准库有 blanket impl：impl<T: Clone> ToOwned for T）：

```rust
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
struct Config {
    host: String,
    port: u16,
}

fn with_default_port(cfg: &Config) -> Cow<'_, Config> {
    if cfg.port == 0 {
        let mut c = cfg.clone();
        c.port = 8080;
        Cow::Owned(c)
    } else {
        Cow::Borrowed(cfg)
    }
}
```

## 在结构体字段中使用 Cow

拥有数据但想避免不必要的拷贝时，字段也可以存 Cow：

```rust
struct Greeting<'a> {
    message: Cow<'a, str>,
}

impl<'a> Greeting<'a> {
    fn new_static() -> Self {
        Greeting { message: Cow::Borrowed("hello") } // 静态字符串，零分配
    }

    fn new_owned(s: String) -> Self {
        Greeting { message: Cow::Owned(s) }
    }

    fn new_ref(s: &'a str) -> Self {
        Greeting { message: Cow::Borrowed(s) }
    }
}
```

这就是很多序列化库（如 serde）里 Cow<'a, str> 字段允许"反序列化时借用输入数据"的原理。

## Cow 作为函数返回值 vs 参数

- 返回值：Cow<'a, str> 让调用者拿到"可能是借用也可能是拥有"的统一类型，非常方便。
- 参数：参数更常用的是 &str / impl Into<Cow<'a, str>>，后者可以接受 &str、String、Cow：

```rust
fn greet(name: impl Into<Cow<'static, str>>) {
    let name = name.into();
    println!("Hello, {name}!");
}

greet("world");                    // &'static str
greet(String::from("Rust"));      // String
greet(Cow::Borrowed("cow"));      // Cow
```

完整示例：路径处理

```rust
use std::borrow::Cow;
use std::path::{Path, PathBuf};

fn normalize_path(p: &Path) -> Cow<'_, Path> {
    if p.is_absolute() {
        Cow::Borrowed(p)
    } else {
        Cow::Owned(std::env::current_dir().unwrap().join(p))
    }
}

fn main() {
    let abs = normalize_path(Path::new("/etc/hosts"));
    let rel = normalize_path(Path::new("Cargo.toml"));
    println!("{:?}", abs);  // Borrowed
    println!("{:?}", rel);  // Owned
}
```

## 常见坑

```rust
// 1. Cow<str> 不是 String，不能直接传给要 String 的函数
let c: Cow<str> = Cow::Borrowed("hi");
let s: String = c.into_owned(); // 需要显式转换

// 2. 反复 to_mut 每次都克隆吗？不会——一旦 Owned 之后直接复用
let mut c = Cow::Borrowed("a");
c.to_mut().push('b');  // 克隆一次
c.to_mut().push('c');  // 不再克隆

// 3. 生命周期：Borrowed 不能超过源数据的生命周期
fn bad<'a>(s: &'a str) -> Cow<'static, str> {
    Cow::Borrowed(s) // 编译错误！
}
```

## 小结

- Cow = "借用优先，写时克隆"，避免为偶尔需要的修改付出总是克隆的代价。
- 返回 Cow<'a, str> 是字符串处理 API 的常见模式：不改直接借用，要改才克隆。
- 写自定义类型时，只要实现 Clone（ToOwned 自动满足）就能和 Cow 配合。
- 配合 Deref，使用方基本感知不到内部是借用还是拥有。
