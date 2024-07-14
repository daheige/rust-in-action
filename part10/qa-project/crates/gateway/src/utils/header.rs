use axum::http::{header::AsHeaderName, HeaderMap};

/// get header by key
pub fn get_header<K: AsHeaderName>(headers: &HeaderMap, key: K) -> String {
    let s = headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or("".to_string());
    s
}
