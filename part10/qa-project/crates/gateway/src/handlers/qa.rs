use super::json_or_form::JsonOrForm;
use crate::config::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tonic::Request;

use crate::handlers::custom_validate::validate_required;
use autometrics::autometrics;
use pb::qa::{UserLoginRequest, UserLogoutReply, UserLogoutRequest};
use validator::{Validate, ValidationError};

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
    #[validate(custom(function = "validate_required", message = "username is empty"))]
    #[validate(length(max = 32, message = "username invalid"))]
    pub username: String,

    #[validate(custom(function = "validate_required", message = "password is empty"))]
    #[validate(length(min = 6, max = 32, message = "password invalid"))]
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
    info!("response:{:?}", response);
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(custom(function = "validate_required", message = "token is empty"))]
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LogoutReply {
    pub state: i64,
}

// 用户退出登录
pub async fn user_logout(
    State(state): State<Arc<AppState>>,
    JsonOrForm(payload): JsonOrForm<LogoutRequest>,
) -> impl IntoResponse {
    let req = Request::new(UserLogoutRequest {
        token: payload.token,
    });
    let response = state.grpc_client.clone().user_logout(req).await;
    info!("response:{:?}", response);
    match response {
        Ok(res) => {
            let res = res.into_inner();
            let reply = LogoutReply { state: res.state };
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
