// 引入chrono时间相关的包
use chrono::{DateTime, Local};
use rcron::{Job, JobScheduler};
use std::thread;
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();
    // 每3秒运行一次
    sched.add(Job::new("*/3 * * * * *".parse().unwrap(), || {
        print_current_time();
    }));

    // 启动job scheduler
    loop {
        // 调用 tick 方法执行待处理的任务
        // 建议至少停顿500毫秒
        sched.tick();
        thread::sleep(Duration::from_millis(500));
    }
}

// 获取当前时间并输出到终端
fn print_current_time() {
    // 时间格式，eg:2023-01-01 09:09:09
    let fmt = "%Y-%m-%d %H:%M:%S";
    // 获取当前时间
    let now: DateTime<Local> = Local::now();
    let time = now.format(fmt);
    let str_date = time.to_string();
    println!("当前时间: {}", str_date);
}
