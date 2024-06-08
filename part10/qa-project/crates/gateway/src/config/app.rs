use pb::qa::qa_service_client::QaServiceClient;

// 定义传递给axum handlers的app_state，这里是通过引用计数的方式共享变量
// Sharing state with handlers
pub struct AppState {
    pub grpc_client: QaServiceClient<tonic::transport::Channel>,
}
