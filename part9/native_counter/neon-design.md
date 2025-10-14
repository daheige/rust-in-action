# neon设计核心

- 这个Neon库的核心在于其对Rust和Nodejs
  V8引擎（Nodejs的js运行时）之间的接口API处理，它采用了一种JIT即时编译的方式，将Rust函数转换为可直接在Javascript环境中执行的形式，从而实现了Nodejs近乎原生的速度。
- 另外一点，Neon还确保了开发人员在调用Rust代码时的内存安全，避免了运行时常见的内存错误。
