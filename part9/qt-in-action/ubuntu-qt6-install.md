# ubuntu 22.04 系统安装qt

```shell
# 安装cmake make gcc g++ clang llvm
sudo apt update -y
sudo apt install -y make gcc g++ llvm clang cmake

# 安装qt6相关依赖
sudo apt install -y pkg-config libssl-dev zlib1g-dev
sudo apt install -y build-essential libgl1-mesa-dev gdb
sudo apt install -y libxkbcommon-dev
sudo apt install -y libvulkan-dev
sudo apt install -y wget vim bash curl git

# 安装qt6
sudo apt install -y qt6-base-dev qt6-declarative-dev
sudo apt install -y qt6*
```

# qt6 qmake设置

1. Select Qt6 system-wide

```shell
vim ~/qt6.conf
# 添加如下内容
qtchooser -install qt6 $(which qmake6)
```

:wq exist

```shell
sudo mv ~/qt6.conf /usr/share/qtchooser/qt6.conf
```

2. Set Qt6 as default option

```shell
sudo mkdir -p /usr/lib/$(uname -p)-linux-gnu/qt-default/qtchooser
sudo ln -n /usr/share/qtchooser/qt6.conf /usr/lib/$(uname -p)-linux-gnu/qt-default/qtchooser/default.conf
sudo ln -s /usr/bin/qmake6 /usr/bin/qmake
```

3. Select Qt6 for current user only

Generate qt6.conf based on path to qmake6

```shell
qtchooser -install qt6 $(which qmake6)
```

4. Select Qt6 as default (place in ~/.bashrc for persistence):
   vim ~/.bashrc

```shell
export QT_SELECT=qt6
```

source ~/.bashrc
