use std::net::{IpAddr, SocketAddr};
use tonic::service::Interceptor;
use tonic::{Request, Status};

#[derive(Clone)]
pub struct LoggingInterceptor;
impl Interceptor for LoggingInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        println!("Request received: {:?}", request);
        let ip = get_client_ip(&request);
        let ua = get_ua(&request);

        println!("gRPC request ip: {} ua:{}", ip, ua);

        Ok(request)
    }
}

fn get_client_ip(req: &Request<()>) -> String {
    // 转换为IP字符串
    req.remote_addr() // 获取SocketAddr
        .map(|addr: SocketAddr| addr.ip().to_string())
        .unwrap_or(String::from("unknown"))
}

fn get_ua(req: &Request<()>) -> String {
    // 转换为ua字符串
    req.metadata()
        .get("user-agent")
        .map(|ua| ua.to_str().unwrap_or("unknown"))
        .unwrap_or("no agent")
        .to_string()
}
