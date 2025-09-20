# 安装qt必要的工具链

```shell
brew install llvm cmake make gcc mold clang-format
```

```shell
brew install qt6
brew install qt-creator # 可选，如果需要用c++开发qt项目，就需要安装qt-creator
```

当看到下面这个表示qt-creator

```
==> Installing Cask qt-creator
==> Moving App 'Qt Creator.app' to '/Applications/Qt Creator.app'
🍺  qt-creator was successfully installed!
```

安装好的目录在：`/usr/local/Cellar/qt`中，需要注意的一点：brew安装的qt版本也有可能是高版本。 在对应的目录执行ls命令查看Qt版本，如下所示：

```shell
% ls
6.9.2
```

# qt link 绑定

这一步需要做，不然cxx-qt找不到qt相关的路径

```shell
brew link qt
```

# 设置 qt 相关的环境变量

vim ~/.bash_profile

```shell
# 分别对 LDFLAGS CPPFLAGS PKG_CONFIG_PATH 配置
export QT_HOME=/usr/local/Cellar/qt/6.9.2 # qt安装目录

# 请通过下面的方式设置LDFLAGS
export LDFLAGS="$LDFLAGS -L$QT_HOME/lib"
export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib"
export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib/c++ -Wl,-rpath,/usr/local/opt/llvm/lib/c++"

# 请通过下面的方式设置CPPFLAGS
export CPPFLAGS="$CPPFLAGS -I$QT_HOME/include"
export CPPFLAGS="$CPPFLAGS -I/usr/local/opt/llvm/include"

# 对于 PKG_CONFIG_PATH 设置，如果系统中没有设置过
export PKG_CONFIG_PATH="$QT_HOME/lib/pkgconfig"
# 如果你执行 echo $PKG_CONFIG_PATH 命令，输出结果不为空，就通过通过下面的方式设置
# export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$QT_HOME/lib/pkgconfig"

export PATH="$QT_HOME/bin:$PATH"
export PATH="/usr/local/opt/llvm/bin:$PATH"
```

保存:wq 之后，再执行如下命令生效

```shell
source ~/.bash_profile
```

# 查看qt对应的qmake版本

```shell
qmake --version
```

# qt-creator 配置（可选）

如果需要用c++开发qt项目，就需要qt link
打开qt-creator填写qt安装路径`/usr/local/Cellar/qt/6.9.2`就完成了qt link设置
