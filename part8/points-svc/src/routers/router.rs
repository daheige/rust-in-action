use crate::{config, handlers};
use axum::routing::post;
use axum::{routing::get, Router};
use std::sync::Arc;

// api路由设置
pub fn api_router(state: Arc<config::AppState>) -> Router {
    let router = Router::new()
        .route("/", get(handlers::index::root))
        .route("/api/points/:openid", get(handlers::index::user_points))
        .route("/api/points/:openid/add", post(handlers::index::add))
        .with_state(state);
    router
}
