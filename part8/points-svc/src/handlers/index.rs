use crate::config;
use crate::entity::members::MembersEntity;
use crate::entity::points_details::PointsDetailsEntity;
use crate::entity::PointsMessage;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Local};
use pulsar::{producer, proto, Error as PulsarError, Pulsar, TokioExecutor};
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello, World!"
}

// 查看用户积分明细
pub async fn points(
    Path(openid): Path<String>,
    Query(pagination): Query<super::Pagination>,
    State(state): State<Arc<config::AppState>>,
) -> Response {
    // 判断用户是否存在
    let res = check_user(&openid, &state.mysql_pool).await;
    if let Err(err) = res {
        match err {
            sqlx::Error::RowNotFound => {
                return (
                    StatusCode::OK,
                    Json(super::Reply {
                        code: 1001,
                        message: "openid not found".to_string(),
                        data: Some(Vec::<super::EmptyArray>::new()),
                    }),
                )
                    .into_response();
            }
            other => {
                return (
                    StatusCode::OK,
                    Json(super::Reply {
                        code: 404,
                        message: format!("openid invalid,err:{}", other),
                        data: Some(Vec::<super::EmptyArray>::new()),
                    }),
                )
                    .into_response();
            }
        }
    }

    if pagination.page.is_none() {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1002,
                message: "missing page parameter".to_string(),
                data: Some(Vec::<super::EmptyArray>::new()),
            }),
        )
            .into_response();
    }

    let limit = pagination.limit.unwrap_or(20);
    let page = pagination.page.unwrap();
    println!("page:{} limit:{}", page, limit);
    // 查询用户的积分
    let records: Result<Vec<PointsDetailsEntity>, _> =
        query_points(&openid, limit, page, &state.mysql_pool).await;
    if let Err(err) = records {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1003,
                message: format!("get points detail failed,err:{}", err),
                data: Some(super::EmptyObject {}),
            }),
        )
            .into_response();
    }

    let records = records.unwrap();
    for row in &records {
        println!("current row = {:?}", row);
        println!("id = {} points = {}", row.id, row.points);
    }

    // 返回文章实体信息
    (
        StatusCode::OK,
        Json(super::Reply {
            code: 0,
            message: "ok".to_string(),
            data: Some(records),
        }),
    )
        .into_response()
}

pub async fn add(
    State(state): State<Arc<config::AppState>>,
    Json(message): Json<PointsMessage>,
) -> Response {
    // 判断用户是否存在
    let res = check_user(&message.openid, &state.mysql_pool).await;
    if let Err(err) = res {
        match err {
            sqlx::Error::RowNotFound => {
                return (
                    StatusCode::OK,
                    Json(super::Reply {
                        code: 1001,
                        message: "openid not found".to_string(),
                        data: Some(super::EmptyObject {}),
                    }),
                )
                    .into_response();
            }
            other => {
                return (
                    StatusCode::OK,
                    Json(super::Reply {
                        code: 404,
                        message: format!("openid invalid,err:{}", other),
                        data: Some(super::EmptyObject {}),
                    }),
                )
                    .into_response();
            }
        }
    }

    // 发送积分操作的消息
    let reply = publish_message(message, &state.pulsar_client).await;
    if let Err(err) = reply {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1002,
                message: format!("publish points message,err:{}", err),
                data: Some(super::EmptyObject {}),
            }),
        )
            .into_response();
    }

    // 发送消息成功后，返回结果
    (
        StatusCode::OK,
        Json(super::Reply {
            code: 0,
            message: "ok".to_string(),
            data: Some(super::EmptyObject {}),
        }),
    )
        .into_response()
}

// 发送积分操作的消息
async fn publish_message(
    message: PointsMessage,
    pulsar_client: &Pulsar<TokioExecutor>,
) -> Result<(), PulsarError> {
    // 消息主题topic
    let topic = "my-topic";
    // 创建producer
    let mut producer = pulsar_client
        .producer()
        .with_topic(topic)
        .with_name("points-sys")
        .with_options(producer::ProducerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            ..Default::default()
        })
        .build()
        .await?;

    // 验证producer connection是否生效
    producer.check_connection().await?;

    let local: DateTime<Local> = Local::now();
    let created_at = local.format("%Y-%m-%d %H:%M:%S").to_string();
    // 将消息推送到Pulsar mq队列对应的topic主题中
    let message = PointsMessage {
        created_at,
        ..message // 其他字段通过message结构体字段进行填充
    };
    producer.send(message).await?;
    Ok(())
}

// 检测用户是否存在
pub async fn check_user(openid: &str, mysql_pool: &sqlx::MySqlPool) -> Result<bool, sqlx::Error> {
    let sql = format!(
        "select * from {} where openid = ?",
        MembersEntity::table_name()
    );
    // query_as将其映射到结构体UserEntity中
    let user: MembersEntity = sqlx::query_as(&sql)
        .bind(openid)
        .fetch_one(mysql_pool)
        .await?;
    Ok(user.id > 0)
}

// 根据openid和limit,page查询用户的积分
pub async fn query_points(
    openid: &str,
    limit: u64,
    page: u64,
    mysql_pool: &sqlx::MySqlPool,
) -> Result<Vec<PointsDetailsEntity>, sqlx::Error> {
    let sql = format!(
        "select * from {} where openid = ? order by id desc limit ? offset ?",
        PointsDetailsEntity::table_name()
    );
    let records: Vec<PointsDetailsEntity> = sqlx::query_as(&sql)
        .bind(openid)
        .bind(limit)
        .bind((page - 1) * limit)
        .fetch_all(mysql_pool)
        .await?;
    Ok(records)
}
