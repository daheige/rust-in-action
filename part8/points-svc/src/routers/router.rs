use crate::{config, handlers};
use axum::routing::post;
use axum::{routing::get, Router};
use std::sync::Arc;

// 配置积分服务对应的api路由
pub fn api_router(state: Arc<config::AppState>) -> Router {
    let router = Router::new()
        .route("/", get(handlers::index::root))
        .route("/api/points/:openid", get(handlers::index::points))
        .route("/api/points/publish", post(handlers::index::publish))
        .with_state(state);
    router
}
