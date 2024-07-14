use crate::config::AppState;
use crate::handlers;
use crate::middleware as ware;
use std::sync::Arc;

// 引入axum相关包
use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

// create api router
pub fn api_router(state: Arc<AppState>) -> Router {
    // set api group and not found handler for api/xxx
    let api_routers = Router::new()
        .route("/home", get(handlers::qa::root))
        .route("/hello", get(handlers::qa::hello))
        .route("/user/login", post(handlers::qa::user_login))
        .with_state(state)
        .fallback(api_not_found);

    let router = Router::new()
        .nest("/api", api_routers)
        .route("/", get(handlers::qa::root))
        .fallback(not_found_handler) // api router not found
        .layer(middleware::from_fn(ware::request::access_log))
        .layer(middleware::from_fn(ware::header::no_cache_header));

    router
}

// handler not found
async fn api_not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(handlers::Reply {
            code: 404,
            message: "api not found".to_string(),
            data: Some(handlers::EmptyObject {}),
        }),
    )
}

// handler not found for global router not found
async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "this page not found")
}
