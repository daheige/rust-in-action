use crate::config::{APP_CONFIG, mysql, xredis};
use chrono::Local;
use rcron::{Job, JobScheduler};
use redis::Commands;
use std::io::Write;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use std::{process, thread};

// 定义项目相关的模块
mod config;
mod entity;
mod infras;

// 在终端中运行方式：RUST_LOG=debug cargo run --bin article-job
#[tokio::main]
async fn main() {
    // 初始化日志,这里采用自定义日志输出
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .format(|buf, record| {
            let file = record.file().unwrap_or("??"); // 文件名
            let line = record.line().unwrap_or(0); // 行号
            writeln!(
                buf,
                "{} [{}] - file:{}#{} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // 本地时间格式
                record.level(),
                file,
                line,
                record.args()
            )
        })
        .init();
    println!("article job...");
    println!("current process pid:{}", process::id());

    // 处理文章阅读数
    // handler_read_count().await; // 这种方式适合linux crontab执行

    // 通过rcron库执行
    let mut sched = JobScheduler::new();
    sched.add(Job::new("*/10 * * * * *".parse().unwrap(), || {
        println!("exec task every 10 seconds!");
        // 通过tokio异步执行
        tokio::spawn(async {
            let res = handler_read_count().await;
            if res.is_err() {
                println!("handler read_count error: {}", res.err().unwrap());
            }
        });
    }));

    // 启动job scheduler
    loop {
        // 调用 tick 方法执行待处理的任务
        sched.tick();
        // 建议至少停顿500毫秒
        thread::sleep(Duration::from_millis(500));
    }
}

// 获取当前时间并输出到终端
async fn handler_read_count() -> anyhow::Result<()> {
    // mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("create mysql pool failed");

    // redis pool
    let redis_pool = xredis::pool(&APP_CONFIG.redis_conf);
    // 通过arc引用计数的方式传递state
    let state = Arc::new(config::AppState {
        mysql_pool: mysql_pool,
        redis_pool: redis_pool,
    });

    // 读取redis hash记录
    let hash_key = "article_sys:read_count:hash";
    let redis_pool = state.redis_pool.clone();
    let mut conn = redis_pool.get().expect("get redis connection failed");

    // 通过hscan游标方式遍历数据
    let mut cursor: u64 = 0;
    let pattern = "*";
    let count = 50; // 每次扫描返回的数量

    loop {
        // 执行HSCAN命令
        // 当我们使用Redis hscan游标匹配数据时，
        // 第一个元素是field，第二个元素是field对应的value值
        let result: (u64, Vec<(String, String)>) = redis::cmd("HSCAN")
            .arg(hash_key)
            .arg(cursor)
            .arg("MATCH")
            .arg(pattern)
            .arg("COUNT")
            .arg(count)
            .query(conn.deref_mut())?;

        let (new_cursor, records) = result;
        for (field, value) in records.iter() {
            let id: i64 = field.parse().unwrap_or(0); // 当前文章id
            let increment: i64 = value.parse().unwrap_or(0); // 当前文章增量计数器
            if increment == 0 || id <= 0 {
                continue;
            }

            println!("current id:{} read_count increment:{}", id, increment);
            let res = update_read_count(state.clone(), id, increment).await;
            if res.is_err() {
                println!("update_read_count error: {}", res.err().unwrap());
            }
        }

        // 当游标回到0时表示遍历完成
        if cursor == 0 {
            break;
        }

        cursor = new_cursor;
    }

    Ok(())
}

// 将redis hash中文章增量计数器对应的数量，更新到数据表articles中，并对应减少对应的数量
async fn update_read_count(
    state: Arc<config::AppState>,
    id: i64,
    increment: i64,
) -> anyhow::Result<()> {
    log::info!("update id:{} read_count increment:{} begin", id, increment);
    let sql = "update articles set read_count = read_count + ? where id = ?";
    let res = sqlx::query(sql)
        .bind(increment)
        .bind(id)
        .execute(&state.mysql_pool)
        .await?;
    if res.rows_affected() == 0 {
        return Err(anyhow::anyhow!(
            "failed to update id:{} read_count increment:{}",
            id,
            increment
        ));
    }

    println!(
        "execute update id:{} increment:{} result:{:?}",
        id, increment, res
    );

    // 更新redis hash 文章阅读数对应的计数器
    // redis hash 的field是文章id，value是阅读数
    let hash_key = "article_sys:read_count:hash";
    let mut conn = state.redis_pool.get().expect("get redis connection failed");
    let remain: i64 = conn
        .hincr(hash_key, id.to_string(), -increment)
        .map_err(|err| {
            anyhow::anyhow!(
                "failed to handler id:{} increment:{} hincr error:{}",
                id,
                increment,
                err
            )
        })?;

    log::info!("current article id:{} hincry result:{}", id, remain);
    log::info!("update id:{} read_count increment:{} end", id, increment);

    Ok(())
}
