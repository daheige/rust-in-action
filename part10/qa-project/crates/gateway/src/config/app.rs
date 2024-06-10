use pb::qa::qa_service_client::QaServiceClient;
use serde::{Deserialize, Serialize};

// 定义传递给axum handlers的app_state，这里是通过引用计数的方式共享变量
// Sharing state with handlers
pub struct AppState {
    pub grpc_client: QaServiceClient<tonic::transport::Channel>,
}

// AppConfig gateway项目配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub app_debug: bool,         // 是否开启调试模式
    pub app_port: u16,           // http gateway port
    pub metrics_port: u16,       // prometheus metrics port
    pub grpc_address: String,    // grpc service运行地址
    pub graceful_wait_time: u64, // http service平滑退出等待时间，单位s
}
