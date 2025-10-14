mod mysql;

use dotenv::dotenv;
use rcron::{Job, JobScheduler};
use std::thread;
use std::time::Duration;

fn main() {
    println!("backup mysql database...");
    // 读取.env配置文件
    dotenv().expect("failed to load .env file");
    mysql::backup_database(); // 演示操作

    println!("backup mysql database action...");
    // 创建一个rcron job实例
    let mut sched = JobScheduler::new();
    // 每天凌晨1点执行数据库备份操作
    sched.add(Job::new("0 0 1 * * *".parse().unwrap(), || {
        mysql::backup_database();
    }));

    // 启动job scheduler
    loop {
        // 调用 tick 方法执行待处理的任务
        sched.tick();
        // 建议至少停顿500毫秒
        thread::sleep(Duration::from_millis(500));
    }
}
