use crate::config::{mysql, xredis, APP_CONFIG};
use chrono::{DateTime, Local};
use env_logger::{Builder, Target};
use std::env;
use std::io::Write;

use r2d2::Pool;
use redis::Commands;
use std::process;
use std::sync::Arc;

// 定义项目相关的模块
mod config;
mod entity;
mod infras;

// 在终端中运行方式：RUST_LOG=debug cargo run --bin article-job
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    env_logger::builder()
        .target(Target::Stdout)
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
    println!("app_debug:{:?}", APP_CONFIG.app_debug);
    println!("current process pid:{}", process::id());

    // mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("create mysql pool failed");

    // redis pool
    let redis_pool = xredis::pool(&APP_CONFIG.redis_conf);
    // 通过arc引用计数的方式传递state
    let app_state = Arc::new(config::AppState {
        mysql_pool: mysql_pool,
        redis_pool: redis_pool,
    });

    handler_read_count(app_state).await;
    Ok(())
}

// 获取当前时间并输出到终端
async fn handler_read_count(state: Arc<config::AppState>) {
    // 读取redis hash记录
    let hash_key = "article_sys:read_count:hash";
    let redis_pool = state.redis_pool.clone();
    let mut conn = redis_pool.get().expect("get redis connection failed");

    // 返回对应的key val key val...格式，对应的是id read_count字符串格式
    let res: redis::Iter<String> = conn.hscan_match(hash_key, "*").unwrap();
    let mut iterator = res.into_iter();
    while let Some(key) = iterator.next() {
        let id: i64 = key.parse().unwrap();
        let read_count: i64 = iterator.next().unwrap().parse().unwrap();
        log::info!("id:{} read_count:{}", id, read_count);
        if read_count == 0 {
            continue;
        }

        let state = state.clone();
        update_read_count(state, id, read_count).await;
    }
}

// 将redis hash中文章增量计数器对应的数量，更新到数据表articles中，并对应减少对应的数量
async fn update_read_count(state: Arc<config::AppState>, id: i64, read_count: i64) {
    let sql = "update articles set read_count = read_count + ? where id = ?";
    let affect_rows = sqlx::query(sql)
        .bind(read_count)
        .bind(id)
        .execute(&state.mysql_pool)
        .await;
    println!("{:?}", affect_rows);
    if affect_rows.is_ok() {
        decrease_read_count(state.redis_pool.clone(), id, read_count);
    }
}

// 更新redis hash 文章阅读数对应的计数器
fn decrease_read_count(pool: Pool<redis::Client>, id: i64, read_count: i64) {
    // redis hash 的field是文章id，value是阅读数
    let hash_key = "article_sys:read_count:hash";
    let mut conn = pool.get().expect("get redis connection failed");
    let remain: i64 = conn
        .hincr(hash_key, id.to_string(), -read_count)
        .expect("redis hincr failed");
    println!("current article id:{} hincry result:{}", id, remain);
}
