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
use autometrics::objectives::{Objective, ObjectiveLatency, ObjectivePercentile};
use pb::qa::{QuestionDetailRequest, UserLoginRequest, UserLogoutRequest, UserRegisterRequest};
use validator::Validate;

// Add autometrics Service-Level Objectives (SLOs)
// https://docs.autometrics.dev/rust/adding-alerts-and-slos
// Define SLO service level targets for api requests, such as success rate, request time.
// We expect 99.9% of all requests to succeed.
// We expect 99% of all latencies to be below 1000ms.
// Autometrics raises an alert whenever any of the SLO objectives fail.
const API_SLO: Objective = Objective::new("gateway_api")
    .success_rate(ObjectivePercentile::P99_9)
    .latency(ObjectiveLatency::Ms1000, ObjectivePercentile::P99);

// basic handler that responds with a static string
#[autometrics]
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[autometrics]
pub async fn hello() -> &'static str {
    "Hello, qa-svc!"
}

// 用户注册请求实体
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(custom(function = "validate_required", message = "username is empty"))]
    #[validate(length(max = 32, message = "username invalid"))]
    pub username: String,

    #[validate(custom(function = "validate_required", message = "password is empty"))]
    #[validate(length(min = 6, max = 32, message = "password invalid"))]
    pub password: String,
}

// 用户注册返回结果
#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterReply {
    pub state: i64,
}

pub async fn user_register(
    State(state): State<Arc<AppState>>,
    JsonOrForm(payload): JsonOrForm<LoginRequest>,
) -> impl IntoResponse {
    let req = Request::new(UserRegisterRequest {
        username: payload.username,
        password: payload.password,
    });
    let response = state.grpc_client.clone().user_register(req).await;
    info!("response:{:?}", response);
    match response {
        Ok(res) => {
            let res = res.into_inner();
            let reply = RegisterReply { state: res.state };
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
#[autometrics(objective = API_SLO)]
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
#[autometrics]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct QuestionDetailReply {
    pub question: Option<QuestionEntity>,
}

// QuestionEntity 问题实体
#[derive(Deserialize, Serialize, Debug)]
pub struct QuestionEntity {
    /// 问题id
    pub id: u64,
    /// 问题标题
    pub title: String,
    /// 内容，这里只返回部分内容
    pub content: String,
    /// 创建者
    pub created_by: String,
    /// 问题阅读数
    pub read_count: u64,
    /// 回答数量
    pub reply_count: u64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QuestionRequest {
    #[validate(custom(function = "validate_required", message = "username is empty"))]
    #[validate(length(max = 32, message = "username invalid"))]
    pub username: String,

    #[validate(range(min = 1, message = "id invalid"))]
    pub id: u64,
}

// 问题详情接口
#[autometrics(objective = API_SLO)]
pub async fn question_detail(
    State(state): State<Arc<AppState>>,
    JsonOrForm(payload): JsonOrForm<QuestionRequest>,
) -> impl IntoResponse {
    let req = Request::new(QuestionDetailRequest {
        username: payload.username,
        id: payload.id,
    });
    let response = state.grpc_client.clone().question_detail(req).await;
    info!("response:{:?}", response);
    match response {
        Ok(res) => {
            let res = res.into_inner();
            let question = res.question.unwrap();
            let question = QuestionEntity {
                id: question.id,
                title: question.title,
                content: question.content,
                created_by: question.created_by,
                read_count: question.read_count,
                reply_count: question.reply_count,
            };

            let reply = QuestionDetailReply {
                question: Some(question),
            };
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

// 其他方法，可以参考proto/qa.proto和qa-project/crates/pb/src/qa.rs中的impl<T> QaServiceClient<T>
// 自行添加，这里就不一一列举
