#! /bin/bash
log_file=$(cd "$(dirname "$0")";pwd)"/test.log"
date >> $log_file
echo "exec ok"
exit 0
