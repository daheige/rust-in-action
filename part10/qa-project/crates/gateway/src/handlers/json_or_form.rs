use axum::{
    async_trait,
    extract::{rejection::FormRejection, rejection::JsonRejection, FromRequest, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    Form, Json, RequestExt,
};
use serde::de::DeserializeOwned;
use validator::Validate;

// json or form handler
pub struct JsonOrForm<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + 'static,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        if content_type.starts_with("application/json") {
            let Json(payload) = req
                .extract_with_state(state)
                .await
                .map_err(IntoResponse::into_response)?;
            let res = payload.validate();
            if let Err(err) = res {
                let msg = format!("input validation error: [{}]", err).replace('\n', ", ");
                return Err((
                    StatusCode::OK,
                    Json(super::Reply {
                        code: 1001,
                        message: msg,
                        data: Some(super::EmptyObject {}),
                    }),
                )
                    .into_response());
            }

            return Ok(Self(payload));
        }

        if content_type.starts_with("application/x-www-form-urlencoded") {
            let Form(payload) = req
                .extract_with_state(state)
                .await
                .map_err(IntoResponse::into_response)?;
            let res = payload.validate();
            if let Err(err) = res {
                let msg = format!("input validation error: [{}]", err).replace('\n', ", ");
                return Err((
                    StatusCode::OK,
                    Json(super::Reply {
                        code: 1001,
                        message: msg,
                        data: Some(super::EmptyObject {}),
                    }),
                )
                    .into_response());
            }

            return Ok(Self(payload));
        }

        // invalid content-type
        Err((
            StatusCode::OK,
            Json(super::Reply {
                code: 500,
                message: format!("content-type:{} error", content_type),
                data: Some(super::EmptyObject {}),
            }),
        )
            .into_response())
    }
}
