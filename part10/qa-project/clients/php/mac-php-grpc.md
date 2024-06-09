# macOS安装php8.3
```shell
brew install php@8.3
```
安装完成后提示：
```
The php.ini and php-fpm.ini file can be found in:
    /usr/local/etc/php/8.3/

To start php now and restart at login:
  brew services start php
Or, if you don't want/need a background service you can just run:
  /usr/local/opt/php/sbin/php-fpm --nodaemonize
==> llvm
To use the bundled libc++ please add the following LDFLAGS:
  LDFLAGS="-L/usr/local/opt/llvm/lib/c++ -Wl,-rpath,/usr/local/opt/llvm/lib/c++"

llvm is keg-only, which means it was not symlinked into /usr/local,
because macOS already provides this software and installing another version in
parallel can cause all kinds of trouble.

If you need to have llvm first in your PATH, run:
  echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.zshrc

For compilers to find llvm you may need to set:
  export LDFLAGS="-L/usr/local/opt/llvm/lib"
  export CPPFLAGS="-I/usr/local/opt/llvm/include"
```
添加环境变量 vim ~/.bash_profile 加入如下内容：
```shell
export PHP_HOME=/usr/local/opt/php@8.3
# 按需求打开下面的注释
# export LDFLAGS="$LDFLAGS -L/usr/local/opt/llvm/lib"
# export CPPFLAGS="$CPPFLAGS -I/usr/local/opt/llvm/include"
# export LDFLAGS="$LDFLAGS -L$PHP_HOME/lib"
# export CPPFLAGS="$CPPFLAGS -I$PHP_HOME/include"

export PATH="$PHP_HOME/bin:$PATH"
export PATH="$PHP_HOME/sbin:$PATH"
```
:wq 保存退出，然后执行 `source ~/.bash_profile` 生效

# 启动php
```shell
brew services start php
# 重启执行下面的命令
# brew services restart php
```

# protobuf安装
```shell
brew install protobuf
```
或者编译安装：
```shell
git clone https://github.com/google/protobuf.git

cd protobuf

sh ./autogen.sh

./configure

make
sudo make install
```
# php grpc_php_plugin 插件安装
```shell
sudo mkdir -p /usr/local/grpc
sudo chown -R $USER /usr/local/grpc
export PHP_RELEASE_TAG_HERE=v1.64.2
cd /usr/local/grpc
git clone -b $PHP_RELEASE_TAG_HERE https://github.com/grpc/grpc
cd /usr/local/grpc
git submodule update --init
sudo make && sudo make install
sudo make grpc_php_plugin

#建立php grpc工具软链接
sudo ln -s /usr/local/bin/grpc_php_plugin /usr/bin/grpc_php_plugin
sudo chmod +x /usr/bin/grpc_php_plugin
```

或者使用这种方式安装
```shell
sudo mkdir -p /usr/local/grpc
sudo chown -R $USER /usr/local/grpc
export PHP_RELEASE_TAG_HERE=v1.64.2
cd /usr/local/grpc
git clone -b $PHP_RELEASE_TAG_HERE https://github.com/grpc/grpc
cd grpc
git submodule update --init
mkdir -p cmake/build
cd cmake/build
cmake ../..
sudo make protoc grpc_php_plugin
```
如果`grpc_php_plugin`安装成功，提示如下：
```
[ 50%] Built target statusor
[ 68%] Built target libprotobuf
[ 93%] Built target libprotoc
[100%] Built target grpc_plugin_support
[100%] Built target grpc_php_plugin
```
查看安装好的`grpc_php_plugin`路径:
```shell
which grpc_php_plugin
```
输出结果：/usr/local/bin/grpc_php_plugin

# php protobuf拓展安装
先判断是否安装php grpc.so and protobuf.so
```shell
php -m | grep proto
php -m | grep grpc
```
如果没有任何输出就需要安装grpc和protobuf拓展
```shell
pecl install grpc
pecl install protobuf
```
安装grpc拓展成功提示（这里我是php8.3版本）：
```
Installing '/usr/local/Cellar/php/8.3.8/pecl/20230831/grpc.so'
install ok: channel://pecl.php.net/grpc-1.64.1
Extension grpc enabled in php.ini
```

安装protobuf拓展成功提示：
```
Build process completed successfully
Installing '/usr/local/Cellar/php/8.3.8/pecl/20230831/protobuf.so'
install ok: channel://pecl.php.net/protobuf-4.27.1
Extension protobuf enabled in php.ini
```

查看php.ini路径目录
```shell
php --ini
```
将输出如下内容：
```
% php --ini
Configuration File (php.ini) Path: /usr/local/etc/php/8.3
Loaded Configuration File:         /usr/local/etc/php/8.3/php.ini
Scan for additional .ini files in: /usr/local/etc/php/8.3/conf.d
Additional .ini files parsed:      /usr/local/etc/php/8.3/conf.d/ext-opcache.ini
```

安装好后，在php.ini中添加对应的配置即可
(一般来说上面安装后，会自动添加依赖，如果没有添加，请手动添加即可)
```ini
extension=protobuf.so
extension=grpc.so
```
查看拓展是否安装成功：
```shell
php -m | grep grpc
php -m | grep protobuf
```

# php composer安装
```shell
cd ~
php -r "copy('https://install.phpcomposer.com/installer', 'composer-setup.php');"
php composer-setup.php
php -r "unlink('composer-setup.php');"
```
将安装好的composer.phar移动到/usr/local/bin/下面即可
```shell
sudo mv composer.phar /usr/local/bin/composer
```
设置镜像：
```shell
composer config -g repo.packagist composer https://packagist.org
```
到这里，php composer镜像设置成功
