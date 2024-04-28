use crate::config;
use crate::entity::points_details::PointsDetailsEntity;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello, World!"
}

// 查看用户积分明细
pub async fn user_points(
    Path(id): Path<String>,
    State(state): State<Arc<config::AppState>>,
) -> Response {
    if id.is_empty() {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1001,
                message: "id invalid".to_string(),
                data: Some(Vec::<super::EmptyArray>::new()),
            }),
        )
            .into_response();
    }

    // 查询用户的积分
    let sql = "select * from points_details where openid = ?";
    let records: Result<Vec<PointsDetailsEntity>, _> = sqlx::query_as(sql)
        .bind(id)
        .fetch_all(&state.mysql_pool)
        .await;
    if let Err(err) = records {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1002,
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
