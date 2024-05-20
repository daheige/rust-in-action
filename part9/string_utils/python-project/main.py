# 引入string_utils包
import string_utils;

# 调用string_utils包提供的函数
x = string_utils.sum_as_string(1, 2);
print("sum_as_string(1,2) = ", x);

s = string_utils.explode("a,b,c", ",");
print("字符串a,b,c按照逗号分割后的列表是：", s);

arr = s = string_utils.implode(['a', 'b', 'c'], ",");
print("列表['a', 'b', 'c']按照逗号连接后的字符串为：", arr);
