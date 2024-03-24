use crate::handlers;
use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

// api路由设置
pub fn api_router() -> Router {
    let router = Router::new().route("/", get(handlers::article::root));
    router
}
