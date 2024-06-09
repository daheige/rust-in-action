# php grpc run

     composer install
     php hello_client.php

# About whether to use protobuf.so

     For php7.0+, protoc3 can install php protobuf extension
     vim php.ini
     ; It is not necessary to install, it is generally recommended to use protobuf to expand it is better
     extension=protobuf.so
     extension=grpc.so

     At this time, you can remove the composer2.json
     "google/protobuf": "^3.8"
     mv composer2.json composer.json
     Then composer update

     For those who do not support php protobuf expansion, you can replace composer2.json with composer.json

# composer mirror settings

     Use composer config -g repo.packagist composer https://mirrors.aliyun.com/composer/

# run php client
```shell
composer install
php qa.php daheige
```
output:
```
status code: 0
reply token: abc
```
