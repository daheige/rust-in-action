use super::json_or_form::JsonOrForm;
use crate::config::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tonic::Request;

// validate error
use autometrics::autometrics;
use pb::qa::UserLoginRequest;
use validator::Validate;

// basic handler that responds with a static string
#[autometrics]
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[autometrics]
pub async fn hello() -> &'static str {
    "Hello, qa-svc!"
}

// 用户登录请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "username invalid"))]
    pub username: String,
    #[validate(length(min = 1, max = 32, message = "password invalid"))]
    pub password: String,
}

// 用户登录返回结果
#[derive(Deserialize, Serialize, Debug)]
pub struct LoginReply {
    pub token: String,
}

// 将请求反序列化到UserLoginRequest，然后调用grpc service
#[autometrics]
pub async fn user_login(
    State(state): State<Arc<AppState>>,
    JsonOrForm(payload): JsonOrForm<LoginRequest>,
) -> impl IntoResponse {
    let req = Request::new(UserLoginRequest {
        username: payload.username,
        password: payload.password,
    });
    let response = state.grpc_client.clone().user_login(req).await;
    println!("response:{:?}", response);
    match response {
        Ok(res) => {
            let res = res.into_inner();
            let reply = LoginReply { token: res.token };
            (
                StatusCode::OK,
                Json(super::Reply {
                    code: 0,
                    message: "ok".to_string(),
                    data: Some(reply),
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
