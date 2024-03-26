// 文章模块相关handlers
use crate::config;
use crate::entity::ArticleEntity;
use axum::extract::State;
use axum::response::Response;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use redis::Commands;
use std::sync::Arc;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn show(Path(id): Path<i64>, State(state): State<Arc<config::AppState>>) -> Response {
    if id <= 0 {
        return (
            StatusCode::OK,
            Json(super::Reply {
                code: 1001,
                message: "id invalid".to_string(),
                data: Some(super::EmptyObject {}),
            }),
        )
            .into_response();
    }

    // 查询article信息
    let sql = "select * from articles where id = ?";
    let article: ArticleEntity = sqlx::query_as(sql)
        .bind(1)
        .fetch_one(&state.mysql_pool)
        .await
        .expect("query article failed");

    // redis hash 的field是文章id，value是阅读数
    // 对文章阅读数加1操作，后续可以通过job定期处理，将阅读数同步到db即可 todo
    let hash_key = "article_sys:read_count:hash";
    let mut conn = config::REDIS_POOL
        .get()
        .expect("get redis connection failed");
    let num: i64 = conn
        .hincr(hash_key, id.to_string().as_str(), 1)
        .expect("redis hincr failed");

    println!("current article id:{} hincry result:{}", id, num);

    // 返回文章实体信息
    (
        StatusCode::OK,
        Json(super::Reply {
            code: 0,
            message: "ok".to_string(),
            data: Some(article),
        }),
    )
        .into_response()
}
