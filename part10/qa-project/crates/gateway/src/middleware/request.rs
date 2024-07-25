// 引入axum相关的包
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::config::APP_CONFIG;
use crate::utils::get_header;
use http_body_util::BodyExt; // body output
use log::info;
use std::ops::Sub;
use std::time::Instant;
use uuid::Uuid;

/// request access log
pub async fn access_log(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let method = req.method();
    let uri = req.uri();
    let path = uri.path();
    let query = uri.query();
    let headers = req.headers();
    let ua = get_header(headers, header::USER_AGENT);
    let mut request_id = get_header(headers, "x-request-id");
    if request_id.is_empty() {
        request_id = Uuid::new_v4().to_string()
    }

    let start_time = Instant::now();

    // println!("request:{:?}", req);
    info!(
        "exec begin,method:{} uri:{} path:{} query:{:?} ua:{} request_id:{}",
        method, uri, path, query, ua, request_id,
    );

    // insert x-request-id into headers
    let (mut parts, body) = req.into_parts();

    let request_id = request_id.as_str();
    // print request body
    let bytes = buffer_and_print(request_id, "request", body).await?;

    // add request_id into header["x-request-id"]
    parts
        .headers
        .insert("x-request-id", HeaderValue::from_str(request_id).unwrap());

    // change request with new parts and body
    let req = Request::from_parts(parts, Body::from(bytes));

    // handler request
    let mut response = next.run(req).await;

    // exec request end
    let end_time = Instant::now();
    info!(
        "exec end,request_id:{},exec_time:{}ms",
        request_id,
        end_time.sub(start_time).as_millis(),
    );

    // set x-request-id to headers
    response
        .headers_mut()
        .insert("x-request-id", HeaderValue::from_str(request_id).unwrap());

    // output response body
    // 是否要输出response body根据实际情况决定
    if APP_CONFIG.app_debug {
        let (parts, body) = response.into_parts();
        let bytes = buffer_and_print(request_id, "response", body).await?;
        response = Response::from_parts(parts, Body::from(bytes));
    }

    Ok(response)
}

async fn buffer_and_print<B>(
    request_id: &str,
    prefix: &str,
    body: B,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!(
                    "request_id:{} failed to read {} body error: {}",
                    request_id, prefix, err
                ),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        info!("request_id:{} {} body = {:?}", request_id, prefix, body);
    }

    Ok(bytes)
}
