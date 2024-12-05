// 引入chrono时间相关的包
use chrono::{DateTime, Local};
use rcron::{Job, JobScheduler};
use std::thread;
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();
    // 每1s输出hello,world字符串和当前系统时间到标准输出中
    sched.add(Job::new("*/1 * * * * *".parse().unwrap(), || {
        mock_user_request()
    }));

    // 启动job scheduler
    loop {
        sched.tick();
        thread::sleep(Duration::from_millis(500));
    }
}

// 模拟用户请求
fn mock_user_request() {
    for _i in 1..=10000 {
        println!("{}", "hello,world");
        // 时间格式，eg:2024-12-01 09:09:09
        let fmt = "%Y-%m-%d %H:%M:%S";
        // 获取当前时间
        let now: DateTime<Local> = Local::now();
        let time = now.format(fmt);
        let str_date = time.to_string();
        println!("current sys time: {}", str_date);
    }
}
