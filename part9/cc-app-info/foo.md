# foo.c文件说明

foo.h头文件内容：

```clang
// C代码的函数声明定义
void print_app_info();
// greet函数的参数是一个名为name的指针变量, 它指向一个字符型常量
void greet(const char *name);
```

接着，在foo.c文件中添加如下C代码：

```clang
// 引入c语言的stdio和string库对应的头文件
#include <stdio.h>
#include <string.h>
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
   printf("%s - current version:%s\n", APP_NAME, VERSION);
}

// 实现greet函数，该函数的参数是一个char*指针，本质是一个字符串
void greet(const char *name) {
   printf("hello,%s!\n", name);
}
```

- 在这个foo.c中，首先引入了C语言标准库stdio、string头文件和自定义的foo.h头文件。
- 然后，依次实现了print_app_info、greet函数。
- 这里需要强调的一点：WELCOME、APP_NAME、VESION三个宏都是在程序构建之前通过build.rs文件的main函数运行过程中声明的。
