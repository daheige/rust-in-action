use http::{HeaderMap, HeaderValue};
use log::info;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tonic::Request;
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
pub struct LoggerMiddlewareLayer;

impl<S> Layer<S> for LoggerMiddlewareLayer {
    type Service = MyMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        MyMiddleware { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct MyMiddleware<S> {
    inner: S,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for MyMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<ReqBody>) -> Self::Future {
        // record basic request information
        let start = Instant::now();
        let method = req.uri().path().to_string();
        // let headers = req.headers();
        // let ua = get_header(&headers, "user-agent");
        // let ip = get_header(&headers, "x-forwarded-ip"); // 客户端原始IP地址

        // info!("gRPC request method:{} ua:{} ip:{}", method, ua, ip);
        // info!("gRPC request method:{} ua:{}", method, ua);

        info!("gRPC request method:{} begin", method);
        let future = self.inner.call(req);

        // return future
        Box::pin(async move {
            // waiting for the future to complete
            let res = future.await?;
            let elapsed = start.elapsed().as_millis();
            info!("gRPC request method:{} elapsed:{:?}ms", method, elapsed);
            Ok(res)
        })
    }
}

fn get_header(headers: &HeaderMap<HeaderValue>, key: &str) -> String {
    // 转换为字符串
    headers
        .get(key)
        .map(|val| val.to_str().unwrap_or("unknown"))
        .unwrap_or("")
        .to_string()
}
