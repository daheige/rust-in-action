use chrono::Local;
use dotenv::dotenv;
use std::io::Write;
use std::process::Command;
use std::{env, fs};

#[tokio::main]
async fn main() {
    // 读取.env配置文件
    dotenv().ok();

    // 数据库备份操作的基本信息
    let db_username = env::var("MYSQL_USER").expect("mysql user invalid");
    let db_password = env::var("MYSQL_PASSWORD").expect("mysql pwd invalid");
    let db_host = env::var("MYSQL_HOST").expect("mysql host invalid");
    let db_port = env::var("MYSQL_PORT").expect("mysql host invalid");
    let db_name = env::var("MYSQL_DATABASE").expect("mysql host invalid");

    // 定义mysqldump命令执行的参数选项
    let mut cmd = Command::new("mysqldump");
    cmd.arg("--opt")
        .arg("-h")
        .arg(db_host)
        .arg("--port")
        .arg(db_port)
        .arg("-u")
        .arg(db_username)
        .arg(format!("-p{}", db_password))
        .arg(&db_name);

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
                // 定义备份文件名称
                let fmt = "%Y%m%d%H%M%S";
                let timestamp = Local::now().format(fmt).to_string();
                let backup_path = format!("./{}_{}.sql", &db_name, timestamp);

                // 创建数据库备份文件
                let mut file =
                    fs::File::create(&backup_path).expect("failed to create mysql backup file");

                // 将命令在标准输入中的结果结果写入到文件中
                file.write_all(&output.stdout)
                    .expect("failed to write backup file");

                // 这里可以根据实际情况发送邮件或短信通知...
                mock_send_email(&db_name, &backup_path);
            }
        }
    }
}

fn mock_send_email(db_name: &str, backup_path: &str) {
    println!("backup database {} to {} success", db_name, backup_path);
    // 省略定义发送邮件的业务代码...
}
