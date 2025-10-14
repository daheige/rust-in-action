#! /bin/bash
# 日志文件
log_file=$(cd "$(dirname "$0")";pwd)"/test_env.log"

# 输出当前时间
current_time=`date +%Y-%m-%d\ %H:%M:%S`
echo "current_time:"$current_time >> $log_file

# 输出环境变量
echo $RUSTUP_DIST_SERVER >> $log_file
echo $RUSTUP_UPDATE_ROOT >> $log_file

echo "exec ok"
exit 0
