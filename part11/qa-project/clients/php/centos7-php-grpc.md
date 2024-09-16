# centos7 php grpc_php工具安装
    1、先安装 c-ares
        cd /usr/local
        yum install c-ares-devel c-ares
        sudo mkdir /usr/lib/pkgconfig
        cp /usr/local/lib/pkgconfig/libcares.pc  /usr/lib/pkgconfig
        vi ~/.bashrc
        export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
        source ~/.bashrc

    2、开始安装grpc php代码生成工具

        cd /usr/local/
        sudo mkdir /usr/local/grpc
        sudo chown -R $USER /usr/local/grpc
        git clone https://github.com/grpc/grpc.git

        或者使用 git clone -b $(curl -L https://grpc.io/release) https://github.com/grpc/grpc 克隆分支

        cd /usr/local/grpc
        git submodule update --init
        sudo make && sudo make install
        sudo make grpc_php_plugin

        #建立php grpc工具软链接
        sudo ln -s /usr/local/grpc/bins/opt/grpc_php_plugin /usr/bin/grpc_php_plugin
        sudo chmod +x /usr/bin/grpc_php_plugin

# centos7 安装php grpc和protobuf拓展

    请确保你已经安装了php5.6+版本的php
    php 拓展下载地址： http://pecl.php.net/
    1、protobuf拓展安装
        $ cd /usr/local/src
        $ sudo mkdir php
        $ sudo wget http://pecl.php.net/get/protobuf-3.11.4.tgz
        $ sudo tar xvf protobuf-3.11.4.tgz

        # 查看php-config路径
        $ whereis php-config
        $ cd protobuf-3.11.4
        $ sudo phpize

        $ sudo ./configure --with-php-config=/usr/bin/php-config
        $ sudo make && sudo make install
        安装好了会提示
        Installing shared extensions:     /usr/lib64/php/modules/

        $ cd /usr/lib64/php/modules/
        $ ll protobuf.so
        -rwxr-xr-x 1 root root 1882056 4月  10 01:08 protobuf.so
        添加 protobuf.so到php.ini
        extension=zip.so

        $ php -m | grep protobuf
        protobuf

    2、安装php grpc拓展
        $ cd /usr/local/src/php
        $ sudo wget http://pecl.php.net/get/grpc-1.28.0.tgz
        $ sudo tar xvf grpc-1.28.0.tgz
        $ cd grpc-1.28.0
        $ sudo phpize
        
        $ sudo ./configure --with-php-config=/usr/bin/php-config
        $ sudo make && sudo make install
        安装好了会提示
        Installing shared extensions:     /usr/lib64/php/modules/

        $ cd /usr/lib64/php/modules/
        $ ll grpc.so
        -rwxr-xr-x 1 root root 1882056 4月  10 01:38 grpc.so
        添加 grpc.so到php.ini
        extension=grpc.so

        $ php -m | grep grpc
        grpc
