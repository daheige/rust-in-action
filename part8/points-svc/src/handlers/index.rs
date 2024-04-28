use crate::config;
use crate::entity::points_details::PointsDetailsEntity;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello, World!"
}

// 查看用户积分明细
pub async fn user_points(
    Path(openid): Path<String>,
    Query(pagination): Query<super::Pagination>,
    State(state): State<Arc<config::AppState>>,
) -> Response {
    if openid.is_empty() {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1001,
                message: "openid invalid".to_string(),
                data: Some(Vec::<super::EmptyArray>::new()),
            }),
        )
            .into_response();
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
    let sql = "select * from points_details where openid = ? order by id desc limit ? offset ?";
    let records: Result<Vec<PointsDetailsEntity>, _> = sqlx::query_as(sql)
        .bind(openid)
        .bind(limit)
        .bind((page - 1) * limit)
        .fetch_all(&state.mysql_pool)
        .await;
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
    Path(openid): Path<String>,
    State(state): State<Arc<config::AppState>>,
) -> Response {
    if openid.is_empty() {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1001,
                message: "openid invalid".to_string(),
                data: Some(super::EmptyObject {}),
            }),
        )
            .into_response();
    }

    // 将消息推送到Pulsar mq队列对应的topic主题中 todo

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
