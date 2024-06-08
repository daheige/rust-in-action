use super::json_or_form::JsonOrForm;
use crate::config::AppState;
use axum::extract::State;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tonic::Request;

// validate error
use pb::qa::UserLoginRequest;
use validator::Validate;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn hello() -> &'static str {
    "Hello, qa-svc!"
}

// 将请求反序列化到UserLoginRequest，然后调用grpc service
pub async fn user_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserLoginRequest>,
) -> impl IntoResponse {
    let req = Request::new(payload);
    let response = state.grpc_client.clone().user_login(req).await;
    println!("res:{:?}", response);
    match response {
        Ok(res) => {
            let res = res.into_inner();
            (
                StatusCode::OK,
                Json(super::Reply {
                    code: 0,
                    message: "ok".to_string(),
                    data: Some(res),
                }),
            )
        }
        Err(err) => (
            StatusCode::OK,
            Json(super::Reply {
                code: 500,
                message: format!("request err:{}", err),
                data: None,
            }),
        ),
    }
}
