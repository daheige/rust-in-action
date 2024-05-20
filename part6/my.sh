#! /bin/bash
log_file=$(cd "$(dirname "$0")";pwd)"/test_env.log"
echo $RUSTUP_DIST_SERVER >> $log_file
echo $RUSTUP_UPDATE_ROOT >> $log_file
echo "exec ok"
exit 0
