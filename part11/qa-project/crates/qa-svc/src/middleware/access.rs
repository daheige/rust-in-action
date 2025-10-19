use log::info;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tonic::Request;
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
pub struct MyMiddlewareLayer;

impl<S> Layer<S> for MyMiddlewareLayer {
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
        let ua = get_ua(&req);

        info!("gRPC request method:{} ua:{}", method, ua);
        let future = self.inner.call(req);

        // return future
        Box::pin(async move {
            // waiting for the future to complete
            let res = future.await?;
            let elapsed = start.elapsed().as_millis();
            info!("gRPC request method:{} elapsed:{:?}", method, elapsed);
            Ok(res)
        })
    }
}

fn get_ua<ReqBody: Send + 'static>(request: &http::Request<ReqBody>) -> String {
    // 转换为ua字符串
    request
        .headers()
        .get("user-agent")
        .map(|ua| ua.to_str().unwrap_or("unknown"))
        .unwrap_or("no agent")
        .to_string()
}
