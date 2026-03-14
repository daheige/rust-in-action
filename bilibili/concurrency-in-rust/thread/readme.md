# 使用spawn创建线程

在 Rust 语言中，线程是通过 std::thread::spawn 函数创建的。该函数接收一个
FnOnce 特征的闭包函数，其源码定义如下：

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```

从 spwan 函数的签名来看，似乎这是一个非常复杂的函数，实际上理解 spawn 函数的
设计并不难。接下来，让我们对其中的每一项内容进行逐一分析：

- spwan 是一个包含 F 和 T 的泛型函数，参数 f 是 F 类型，函数返回结果是一
  个 JoinHandle<T>泛型。随后的 where 语句指定了 F 和 T 需要满足的类型约
  束。
- `F: FnOnce() -> T` 这表示 F 实现了一个只能被调用一次的 FnOnce 闭包。换句
  话说，f 是一种特殊的闭包，它可以捕获外部变量以及消耗其所有权，并且
  只能被调用一次。这种特性使得 `FnOnce` 闭包在创建时可以自动捕获外部变
  量的所有权，并在闭包内访问和使用这些外部变量。一旦闭包执行完毕后，
  这些外部变量在闭包外部就不能再次被使用。
- `F: Send + 'static` 这表示 f 闭包必须满足 `Send` 特征和`'static` 静态生命周期，并
  且闭包内引用的任何类型也需要实现 Send 特征和具有'static 静态生命周期，
  能够在整个闭包内运行过程中一直有效。
- `T: Send + 'static` 是 f 闭包执行后的返回结果，也必须满足 Send 特征和'static
  静态生命周期。
- 在 `spawn` 函数主体中，使用 `Builder::new()`创建了一个 `thread::builder` 对象，
  并将 f 闭包传给 `builder` 对象 `spawn`方法创建了一个线程。

这里需要强调一点：`Send` 特征在 Rust 中是一种类型标记，意味着实现了 `Send` 特
征的类型可以安全地发送到多个线程中，这也表明该类型是一种移动类型。在 Rust
语言中，大多数类型都实现了 `Send` 特征，未实现 `Send` 特征的主要有指针、引用类型
等，例如：`&T`，除非 `T 是 Sync 类型`（如果某些类型 T 实现了 Sync 特征，那它也就
实现了 Send 特征）。此外，`Send` 特征是自动派生的特征，例如：如果结构体中的所
有字段都满足 `Send` 特征，那么该结构体就实现了 `Send` 特征。

下面是一个简单示例：thread-output

```rust
use std::{thread, time};

fn main() {
    thread::spawn(|| {
        for c in 'a'..='z' {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", c);
        }

        println!("");
    });
    thread::spawn(|| {
        for i in 1..=9 {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", i);
        }

        println!("");
    });

    // 停顿2s
    thread::sleep(time::Duration::from_secs(2));
}
```

首先通过 `thread::spawn` 函数创建了两个线程，其中第一个线程用
于输出 a-z 字符，第二个线程用于输出 1-9 数字。然后，在每个线程输出内容之前都
停顿了 100ms。最后，在主线程中调用 `thread::sleep` 函数停顿了 2s 等待上述两个线程
执行完毕。

如何解决呢？

```rust
use std::{thread, time};

fn main() {
    let handler1 = thread::spawn(|| {
        for c in 'a'..='z' {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", c);
        }

        println!("");
    });

    let handler2 = thread::spawn(|| {
        for i in 1..=9 {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", i);
        }

        println!("");
    });

    // 通过join方法等待线程执行完毕
    let _ = handler1.join();
    let _ = handler2.join();
    println!("the two threads are finished");
    println!("main thread will exit");
}
```

# 自定义线程和 move 关键字

通过 thread::spawn 函数创建的线程，默认最小堆栈在大部分情
况下是 2MB（该默认值取决于平台并且可能会发生变化）。在 64 位的操作系统中，
我们可以使用 ulimit -s 命令查看堆栈大小，默认是 8MB。也就是说，当我们在 64 位
操作系统下编写 Rust 程序时，其主线程默认堆栈大小为 8MB。然而，用户程序创建
的任一线程，默认最小堆栈值是 2MB。

可以通过以下两种方式改变堆栈的大小：

- 使用 thread::Builder 构建线程，并将所需的堆栈大小传递给 stack_size 方法。
- 通过 RUST_MIN_STACK 环境变量设置堆栈大小（以字节为单位的整数）。
  这里需要注意的一点：stack_size 方法设置的堆栈大小会覆盖 RUST_MIN_STACK 环
  境变量设置的堆栈大小。此外，栈大小并不是无限制的增长，操作系统会对线程的栈
  大小有所限制，超过限制会导致线程创建失败。

custom-thread

```rust
use std::thread;

fn main() {
    // 设置线程栈大小为1MB并设置线程的名字
    let stack_size = 1 * 1024 * 1024; // 1MB
    let builder = thread::Builder::new().stack_size(stack_size).name("my_thread".to_string());

    println!("在自定义的线程中打印1-100的数字");
    let handler = builder.spawn(|| {
        for i in 1..101 {
            print!("{} ", i);
        }
    }).unwrap();

    // 等待线程执行完毕
    handler.join().unwrap();
}
```

通过 thread::Builder::new 函数创建了一个 buidler 实例对
象。然后，在 buidler 对象上调用 stack_size 方法以及 name 方法设置自定义线程的大
小和线程名字。接着，调用 builder 上面的 spawn 函数运行一个闭包函数。最后，在
闭包函数将 1-100 之间的数字输出到标准输出中。

如果我们在上述示例中没有指定栈的大小，那么栈的默认大小由操作系统来决定。
这里需要注意一点：当我们在设置栈大小时，需要格外小心，过大的栈可能会浪费内
存，而过小的栈也有可能导致栈溢出。

thread-stack-overflow

```rust
use std::thread;

fn main() {
    // 过小的堆栈大小
    let builder2 = thread::Builder::new()
        .name("worker thread".to_string())
        .stack_size(4 * 1024); // 4kb大小
    let handler2 = builder2.spawn(|| {
        panic!("oops!");
    });
    let child_status = handler2.unwrap().join();
    println!("child status:{:?}", child_status);
}
```

在 Rust 中，每个变量值都有一个唯一的所有者，在同一时间只能有一个所有者。
当变量超出作用域范围时，就会立即被销毁。然而，在多线程编程中，如果我们希望
在线程创建时将一些数据传递到线程中，并且希望该线程拥有这些数据的所有权。此
时，就需要使用 move 关键字将数据从一个作用域安全地转移到另一个作用域中。
move-ownship

```rust
fn main() {
    // 声明一个整数类型的向量
    let data = vec![1, 2, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        for i in data {
            println!("{}", i);
        }
    });

    // println!("data:{:?}", data);
    // 调用join方法等待线程执行完毕
    handle.join().unwrap();
}
```

运行 cargo run 命令运行

当我们在线程闭包函数中使用 move 关键字时，需要特别小
心变量的所有权转移问题。如果变量的所有权已经被移动到线程闭包函数中，此时在
线程外面继续使用该变量的话，可能会导致编译错误或运行错误（当然，使用 Arc 和
move 组合的场景可能会运行正常）。下面是一个通过 move 关键字将变量的所有权转
移到线程闭包函数中的简单示例:move-closure-value

```rust
fn main() {
    // 声明一个整数类型的向量
    let data = vec![1, 2, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        for i in &data {
            println!("{}", i);
        }
    });

    // 在闭包外部继续使用data，程序无法正常运行，因为data所有权已经被移动到了闭包函数中
    println!("data:{:?}", data);
    handle.join().unwrap();
}
```
