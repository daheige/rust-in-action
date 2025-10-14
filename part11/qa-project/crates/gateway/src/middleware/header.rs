// 引入axum相关的包
use axum::{extract::Request, http::HeaderValue, middleware::Next, response::IntoResponse};
use std::collections::HashMap;

// api without http cache
pub async fn no_cache_header(req: Request, next: Next) -> impl IntoResponse {
    // handler request
    let mut response = next.run(req).await;
    let mut m = HashMap::new();
    m.insert("Cache-Control", "no-cache,must-revalidate,no-store");
    m.insert("Pragma", "no-cache");
    m.insert("Expires", "-1");

    // set no cache header
    for (key, value) in m {
        response
            .headers_mut()
            .insert(key, HeaderValue::from_str(value).unwrap());
    }

    response
}
