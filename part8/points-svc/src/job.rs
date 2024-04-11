use crate::config::{mysql, xredis, APP_CONFIG};
use chrono::Local;
use rcron::{Job, JobScheduler};
use redis::Commands;
use std::io::Write;
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
    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("exec task every 10 seconds!");
        // 通过tokio异步执行
        tokio::spawn(async { handler_read_count().await });
    }));

    // 启动job scheduler
    loop {
        // tick方法为JobScheduler增加时间中断并执行待处理的任务
        sched.tick();
        // 建议至少停顿500毫秒
        thread::sleep(Duration::from_millis(500));
    }
}

// 获取当前时间并输出到终端
async fn handler_read_count() {
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

    // 返回对应的key val key val...格式，对应的是id read_count增量计数器的字符串格式
    let res: redis::Iter<String> = conn.hscan_match(hash_key, "*").unwrap();
    let records: Vec<String> = res.collect();
    let len = records.len();
    if len == 0 {
        return;
    }

    // 执行文章阅读数增量更新操作
    let mut i: usize = 0;
    while i < len {
        let id: i64 = records.get(i).unwrap().parse().unwrap(); // 当前文章id
        let increment: i64 = records.get(i + 1).unwrap().parse().unwrap(); // 当前文章增量计数器
        i += 2; // 这里i的值第一次迭代时 i = 0，第二次迭代 i = 2,依次类推
        if increment == 0 || id <= 0 {
            continue;
        }

        println!("id:{} read_count increment:{}", id, increment);
        update_read_count(state.clone(), id, increment).await;
    }
}

// 将redis hash中文章增量计数器对应的数量，更新到数据表articles中，并对应减少对应的数量
async fn update_read_count(state: Arc<config::AppState>, id: i64, increment: i64) {
    log::info!("update id:{} read_count increment:{} begin", id, increment);
    let sql = "update articles set read_count = read_count + ? where id = ?";
    let res = sqlx::query(sql)
        .bind(increment)
        .bind(id)
        .execute(&state.mysql_pool)
        .await;
    println!("execute result:{:?}", res);
    if res.is_ok() {
        // 更新redis hash 文章阅读数对应的计数器
        // redis hash 的field是文章id，value是阅读数
        let hash_key = "article_sys:read_count:hash";
        let mut conn = state.redis_pool.get().expect("get redis connection failed");
        let remain: i64 = conn
            .hincr(hash_key, id.to_string(), -increment)
            .expect("redis hincr failed");
        log::info!("current article id:{} hincry result:{}", id, remain);
    }

    log::info!("update id:{} read_count increment:{} end", id, increment);
}
