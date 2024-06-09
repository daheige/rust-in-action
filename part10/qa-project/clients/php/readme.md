# php grpc run
1. install php grpc tools
- for linux centos7: [centos7-php-grc](centos7-php-grpc.md)
- for macOS: [mac-php-grpc](mac-php-grpc.md)

2. install php composer dependencies
```shell
# Install php dependencies
composer install
```
3. call grpc micro service
```shell
php qa.php daheige
```
output:
```
status code: 0
reply token: abc
```

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

