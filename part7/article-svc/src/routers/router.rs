use crate::{config, handlers};
use axum::{routing::get, Router};
use std::sync::Arc;

// api路由设置
pub fn api_router(state: Arc<config::AppState>) -> Router {
    let router = Router::new()
        .route("/", get(handlers::article::root))
        .route("/api/article/:id", get(handlers::article::show))
        .with_state(state);
    router
}
