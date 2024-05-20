// foo.c 源码定义
// Created by daheige on 2024/5/16.

// 引入c语言的stdio和string库对应的头文件
#include <stdio.h>
#include <string.h>

// 引入自定义的头文件
#include "foo.h"

// 实现函数print_app_info
void print_app_info() {
    // 如果定义了WELCOME宏，就执行对应的if代码块逻辑
#ifdef WELCOME
    // 调用strcmp函数比较WELCOME宏字符串是否为YES
    if (strcmp(WELCOME, "YES") == 0) {
        printf("welcome to ");
    }
#endif

    // APP_NAME和VERSION两个宏变量是来自build.rs文件定义的宏变量
    // 也就是说当执行cargo build时，先执行build.rs文件中的main函数，然后就会生成C静态库libfoo.a文件
    printf("%s - current version:%s\n", APP_NAME, VERSION);
}

// 实现hello函数
void hello() {
    printf("hello,world\n");
}

// 实现greet函数，该函数的参数是一个char*类型
void greet(const char *name) {
    printf("hello,%s!\n", name);
}
