// 引入chrono、rcron库以及标准库std中的相关模块
use chrono::{DateTime, Local};
use filesize::PathExt; // filesize库用于获取文件大小
use rcron::{Job, JobScheduler};
use std::fs::{copy, OpenOptions};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn main() {
    //  启动一个rcron JobScheduler 实现日志切割功能
    let mut sched = JobScheduler::new();
    sched.add(Job::new("0 */1 * * * *".parse().unwrap(), || {
        let file_path = Path::new("./test.log"); // 日志文件
        rote_file(file_path).unwrap(); // 日志切割操作
    }));

    // 启动job scheduler
    loop {
        // tick方法为JobScheduler增加时间中断并执行待处理的任务
        sched.tick();

        // 这里建议至少停顿500毫秒
        thread::sleep(Duration::from_millis(500));
    }
}

// 日志备份文件的路径获取逻辑
fn log_bak_path(file_path: &Path) -> PathBuf {
    let file_dir = file_path.parent().unwrap(); // 获取当前文件的根目录
    let filename = file_path.file_name().unwrap().to_str().unwrap(); // 文件名称

    // 当前文件的后缀，不包含点
    let ext = file_path.extension().unwrap().to_str().unwrap();

    // 生成备份文件的路径
    let fmt = "%Y%m%d-%H%M%S";
    let now: DateTime<Local> = Local::now(); // 获取当前时间
    let dist_file_name = filename.replace(
        &(".".to_string() + ext),
        format!("-{}.{}", now.format(fmt).to_string(), ext).as_str(),
    );
    let bak_path = file_dir.join(dist_file_name);
    println!("bak_path: {:?}", bak_path);
    bak_path
}

fn rote_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 获取日志文件的大小
    let metadata = file_path.symlink_metadata()?; // 日志文件的元信息
    let real_size = file_path.size_on_disk_fast(&metadata)?; // 文件大小，单位bytes

    // 如果文件大小超过500MB，就对文件进行cp操作后，再清空文件
    // 这个文件大小可以根据实际情况更该
    if real_size >= 500 * 1024 * 1024 {
        let dist_path = log_bak_path(file_path); // 备份文件目标路径
        println!("copy {:?} to {:?} begin", file_path, dist_path);

        // 先备份后清空文件内容
        copy(file_path, dist_path)?;
        // 如果文件存在，就清空文件；如果不存在则创建该文件
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?
            .set_len(0)?;

        println!("finish truncate file:{:?}", file_path);
    } else {
        println!("the test.log file size less than 500MB")
    }

    Ok(())
}
