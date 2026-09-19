# macos gdb使用

## 方案一：安装 gdb（Homebrew）

```bash
brew install gdb
```

安装后要用还必须做一步——给 gdb 签名（macOS 的安全机制，否则不能 attach 进程）：

```bash
# 1. 创建自签名证书
sudo security add-trusted-cert -d -r trustRoot \
  -k /Library/Keychains/System.keychain gdb-cert.cer

# 或者更简单：用 Keychain Access 手动建一个叫 "gdb-cert" 的代码签名证书
# 类型选 Code Signing，然后信任它

# 2. 签名 gdb 二进制
sudo codesign -fs gdb-cert $(which gdb)
```

这一步比较繁琐，而且 macOS 版本越新限制越多。

## 方案二：直接用 lldb（强烈推荐）⭐

macOS 自带的调试器是 **lldb**（装了 Xcode 命令行工具就有），无需安装、无需签名：

```bash
# 确认已安装（装过 Xcode CLT 就行）
lldb --version

# 常见用法
lldb ./your_program          # 启动并调试
lldb -p 20287                # attach 到运行中的进程（相当于 gdb -p）
lldb -c core                 # 调试 core dump

# 常用命令对照（gdb → lldb）
run              →  run / process launch
break main       →  breakpoint set -n main（简写 b main）
continue         →  continue (c)
step             →  step (s)
next             →  next (n)
finish           →  finish (f)
print x          →  print x 或 p x（一样）
backtrace        →  bt（一样）
info registers   →  register read
```

如果想直接调试崩溃的现场，配合：

```bash
lldb -p 20287
(lldb) bt           # 看调用栈
(lldb) thread backtrace all
```

**结论：** 在 Mac 上日常调试直接用 `lldb` 就好，省去 gdb 签名那一堆麻烦。如果你确实需要 gdb（比如兼容某个脚本），再走
`brew install gdb` + 签名的流程。
