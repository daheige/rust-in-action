// 导入本项目自定义的模块
use crate::config::{mysql, xpulsar, APP_CONFIG};
use crate::entity::members::MembersEntity;
use crate::entity::points_details::PointsDetailsEntity;
use crate::entity::PointsMessage;

use futures::TryStreamExt; // pulsar message迭代处理
use log::{error, info}; // 用于记录操作日志
use pulsar::{Consumer, SubType}; // pulsar consumer模块
use sqlx::types::chrono::NaiveDateTime; // 用于mysql datetime时间处理
use std::ops::DerefMut; // 用于解引用处理
use std::process; // 用于获取进程id
use std::sync::{mpsc, Arc}; // 用于线程发送者/接收者以及消息通信处理
use std::time::Duration;
use tokio::signal; // 用于平滑退出信号量处理
use tokio::sync::RwLock; // 用于消费者退出标识读写锁处理

// 定义项目相关的模块
mod config; // 用于配置文件读取以及mysql和pulsar初始化
mod entity; // 实体对象定义
mod infras; // 项目中基础设施层封装

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    env_logger::init(); // 初始化操作日志配置
    println!("points-svc job");
    println!("app_debug:{:?}", APP_CONFIG.app_debug);
    println!("current process pid:{}", process::id());

    // 平滑退出，stop用于消费者退出标识，它是一个引用计数且持有读写锁的bool类型的共享变量
    let stop = Arc::new(RwLock::new(false));
    let (send, recv) = mpsc::channel();
    tokio::spawn(async move {
        graceful_shutdown(send).await;
    });

    // 通过tokio::spawn异步执行消息实时消费
    let stop1 = stop.clone();
    tokio::spawn(async move {
        // 消费消息
        let reply = consumer_message(stop1).await;
        if let Err(err) = reply {
            error!("consumer pulsar message err:{}", err);
        }
    });

    // 当接收到退出信号量时候，更新stop为true
    println!("recv data:{:?}", recv.recv().unwrap());
    let mut stop = stop.write().await;
    *stop = true;
    println!("shutdown success");
    Ok(())
}

async fn consumer_message(exit: Arc<RwLock<bool>>) -> anyhow::Result<()> {
    // mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("mysql pool init failed");
    let pulsar_client = xpulsar::client(&APP_CONFIG.pulsar_conf)
        .await
        .expect("pulsar client init failed");

    // 通过arc引用计数的方式传递state
    let app_state = Arc::new(config::AppState {
        mysql_pool,    // 这里等价于mysql_pool: mysql_pool
        pulsar_client, // 这里等价于pulsar_client: pulsar_client
    });

    let topic = "points-topic";
    // 创建consumer
    let mut consumer: Consumer<PointsMessage, _> = app_state
        .pulsar_client
        .consumer()
        .with_topic(topic) // 设置topic
        .with_consumer_name("group-1") // 设置消费组名字
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("points-sys") // 订阅的名字
        .build()
        .await?;

    info!("consumer has run...");
    let mut counter: usize = 0;
    // 接收消息并消费
    while let Some(msg) = consumer.try_next().await? {
        let exit = exit.read().await;
        if *exit {
            // println!("recv shutdown signal,consumer will stop...");
            return Ok(());
        }

        println!("metadata:{:?}", msg.message_id());
        println!("id:{:?}", msg.message_id());
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(err) => {
                error!("could not deserialize message:{:?}", err);
                continue;
            }
        };

        // 消费消息逻辑,这里需要处理用户积分明细落地到数据库DB中，并更新用户的积分总数
        println!("got message data:{:?}", data);
        let openid = data.openid.clone();
        let reply = points_handler(data, &app_state.mysql_pool).await;
        if let Err(err) = reply {
            error!("current openid:{} consumer points err:{}", openid, err);
            continue;
        }

        // 提交消息ack确认
        consumer.ack(&msg).await?;
        println!("consumer openid:{} message data success", openid);

        counter += 1;
        info!("got message count:{}", counter);
    }

    Ok(())
}

// 积分添加/扣减业务逻辑
async fn points_handler(
    msg: PointsMessage,
    mysql_pool: &sqlx::MySqlPool,
) -> Result<(), sqlx::Error> {
    if msg.action == "add" {
        add_points(msg, mysql_pool).await?;
    } else {
        sub_points(msg, mysql_pool).await?;
    }

    Ok(())
}

// 增加积分操作
async fn add_points(msg: PointsMessage, mysql_pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let fmt = "%Y-%m-%d %H:%M:%S";
    let created_at = msg.created_at.unwrap();
    let sql = format!(
        r#"insert into {} (openid,points,action,reason,created_at) value (?,?,?,?,?);"#,
        PointsDetailsEntity::table_name(),
    );

    // 创建一个事务transaction
    let mut tx = mysql_pool.begin().await?;

    // 插入积分明细记录
    let affect_res = sqlx::query(&sql)
        .bind(&msg.openid)
        .bind(msg.points)
        .bind(msg.action)
        .bind(msg.reason)
        .bind(NaiveDateTime::parse_from_str(&created_at, fmt).unwrap())
        // 在sqlx 0.7版本以上，execute这里需要对tx进行解引用并获取内部DB的可变引用connection
        .execute(tx.deref_mut())
        .await?;
    info!(
        "insert points detail affect_rows:{}",
        affect_res.rows_affected()
    );

    // 更新用户积分总数
    let sql = format!(
        r#"update {} set points = points + ?,updated_at = ? where openid = ?"#,
        MembersEntity::table_name(),
    );
    let affect_res = sqlx::query(&sql)
        .bind(msg.points)
        .bind(NaiveDateTime::parse_from_str(&created_at, fmt).unwrap())
        .bind(&msg.openid)
        .execute(tx.deref_mut())
        .await?;
    info!("update points affect_rows:{}", affect_res.rows_affected());

    // 提交事务
    tx.commit().await?;
    Ok(())
}

// 积分扣减操作
async fn sub_points(msg: PointsMessage, mysql_pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    // 先查询用户积分
    let sql = "select * from members where openid = ?";
    // query_as将其映射到结构体MembersEntity中
    let member: MembersEntity = sqlx::query_as(sql)
        .bind(&msg.openid)
        .fetch_one(mysql_pool)
        .await?;
    if member.points == 0 {
        info!("current openid:{} points is zero", msg.openid);
        return Ok(());
    }

    let mut points = msg.points;
    // 先减去对应的积分，如果小于等于0就直接全部扣除
    let remain = member.points as i64 - msg.points as i64;
    if remain <= 0 {
        info!(
            "current openid:{} points:{} remain:{}",
            msg.openid, member.points, remain
        );
        points = member.points; // 实际扣减的积分数
    }

    let fmt = "%Y-%m-%d %H:%M:%S";
    let created_at = msg.created_at.unwrap();
    let sql = format!(
        r#"
        insert into {} (openid,points,action,reason,created_at) value(?,?,?,?,?);
        "#,
        PointsDetailsEntity::table_name()
    );

    // 创建一个事务transaction
    let mut tx = mysql_pool.begin().await?;
    // 插入积分明细记录
    let affect_res = sqlx::query(&sql)
        .bind(&msg.openid)
        .bind(points)
        .bind(msg.action)
        .bind(msg.reason)
        .bind(NaiveDateTime::parse_from_str(&created_at, fmt).unwrap())
        .execute(tx.deref_mut())
        .await?;
    info!(
        "insert points detail affect_rows:{}",
        affect_res.rows_affected()
    );

    // 更新用户积分总数
    let sql = format!(
        r#"update {} set points = points - ?,used_points = used_points + ?,updated_at = ? where openid = ?;"#,
        MembersEntity::table_name(),
    );
    let affect_res = sqlx::query(&sql)
        .bind(points)
        .bind(points)
        .bind(NaiveDateTime::parse_from_str(&created_at, fmt).unwrap())
        .bind(&msg.openid)
        .execute(tx.deref_mut())
        .await?;
    info!("update points affect_rows:{}", affect_res.rows_affected());

    // 提交事务
    tx.commit().await?;
    Ok(())
}

// graceful shutdown
async fn graceful_shutdown(sender: mpsc::Sender<&str>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(APP_CONFIG.graceful_wait_time)).await;
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(APP_CONFIG.graceful_wait_time)).await;
        },
    }

    println!("signal received,starting graceful shutdown");
    sender.send("shutdown").unwrap();
}
