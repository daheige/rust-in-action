#! /bin/bash

# 定义日志文件路径，使用绝对路径
log_file=$(cd "$(dirname "$0")";pwd)"/test.log"

# 输出当前时间
current_time=`date +%Y-%m-%d\ %H:%M:%S`
echo "current_time:"$current_time >> $log_file

echo "exec ok"
exit 0
