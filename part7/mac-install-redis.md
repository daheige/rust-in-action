# MacOS安装redis服务：

在MacOS操作系统上，你可以使用Homebrew在MacOS上安装Redis，它是macOS上安装Redis最简单的方法，执行命令如下：

```shell
brew install redis
```

当你安装好redis后，你可以通过如下命令启动redis服务。

```shell
brew services start redis
```

接着，你可以通过ping和info命令查看redis服务是否正常运行。

# redis服务其他命令

如果你需要停止、重启Redis服务，你可以只需要执行如下命令：

```shell
# 停止redis服务
brew services stop redis
# 重启redis服务
brew services restart redis
```
