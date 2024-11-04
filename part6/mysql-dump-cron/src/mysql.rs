use chrono::Local;
use std::fmt::Debug;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::{env, fs};

// 数据库备份操作
pub fn backup_database() {
    // 数据库备份操作的基本信息
    let db_username = env::var("MYSQL_USER").expect("mysql user invalid");
    let db_password = env::var("MYSQL_PASSWORD").expect("mysql pwd invalid");
    let db_host = env::var("MYSQL_HOST").expect("mysql host invalid");
    let db_port = env::var("MYSQL_PORT").expect("mysql host invalid");
    let db_name = env::var("MYSQL_DATABASE").expect("mysql host invalid");
    let backup_dir = env::var("BACKUP_DIR").expect("backup_dir invalid");
    let expired_days: u64 = env::var("EXPIRED_DAYS")
        .expect("expired_days invalid")
        .parse()
        .unwrap_or(3);

    // 创建备份目录
    fs::create_dir_all(&backup_dir).expect("failed to create backup dir");

    // 清理过期的文件
    let res = clear_expired_files(&backup_dir, expired_days);
    if let Err(err) = res {
        println!("failed to clear expired sql file,error:{}", err);
    }

    // 定义备份文件名称
    let fmt = "%Y%m%d%H%M%S";
    let timestamp = Local::now().format(fmt).to_string();
    let backup_file = format!("{}_{}.sql", &db_name, timestamp);
    let backup_path = Path::new(&backup_dir).join(&backup_file);
    // 创建数据库备份文件
    fs::File::create(&backup_path).expect("failed to create mysql backup file");

    // 定义mysqldump命令执行的参数选项
    let mut cmd = Command::new("mysqldump");
    cmd.arg("--opt")
        .arg("-h")
        .arg(db_host)
        .arg("--port")
        .arg(db_port)
        .arg("-u")
        .arg(db_username)
        // --single-transaction 这个操作不会锁表，适用于支持事务的数据库引擎，例如InnoDB等，
        // 但是这个不能保证MyISAM表和MEMORY表的数据一致性。
        .arg("--single-transaction") // 确保数据一致性
        // mysql导出时加 --set-gtid-purged=OFF，导入新数据库时，会触发记录到新数据库的binlog日志。
        // 如果不加，则导入新数据库不记录binlog日志。
        // 因此，当主从时用了gtid时，用mysqldump备份时就要加--set-gtid-purged=OFF，
        // 否则在主上导入恢复了数据，主没有了binlog日志，同步则不会被同步。
        .arg("--set-gtid-purged=OFF") // 关闭GTID，避免与新版本MySQL不兼容的问题
        .arg(format!("-p{}", db_password))
        .arg(&db_name)
        .arg("-r")
        .arg(&backup_path);

    // println!("cmd:{:?}", cmd);

    // 执行mysql备份命令
    let res = cmd.output();
    // 通过match关键字匹配命令执行结果
    match res {
        Err(err) => {
            println!("failed to exec mysql dump,error:{}", err);
        }
        Ok(output) => {
            if output.status.success() {
                println!("backup database {} to {:?} success", &db_name, &backup_path);
                // 这里可以根据实际情况发送邮件或短信通知...
                mock_send_email(&db_name, &backup_file);
            }
        }
    }
}

// 模拟邮件发送
fn mock_send_email(db_name: &str, backup_file: &str) {
    println!("backup db:{} to {}", db_name, backup_file);
    // 省略定义发送邮件的业务代码...
}

fn clear_expired_files<P: AsRef<Path> + Debug>(dir: P, expired_days: u64) -> anyhow::Result<()> {
    // 尝试读取目录中的文件，将过期的文件删除
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let path = entry?.path(); // 获取文件的路径

        // 判断path是否是文件
        if path.is_file() {
            let ext = path.extension().unwrap();
            if ext.ne("sql") {
                println!("file:{:?} ext:{:?}", path, ext);
                continue;
            }

            // 删除过期的备份文件
            let metadata = path.metadata()?; // 获取文件的元信息
            let created = metadata.created()?; // 文件创建时间
            let now = SystemTime::now();
            let interval = now.duration_since(created)?;
            println!("interval:{:?}", interval);
            if interval > Duration::from_secs(expired_days * 86400) {
                println!("remove expired file:{:?} begin", path);
                // 尝试删除文件
                fs::remove_file(&path)?;
                println!("remove expired file:{:?} success", path);
            }
        }
    }

    Ok(())
}
