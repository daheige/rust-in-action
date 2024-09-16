use infras::{Config, ConfigTrait};
use once_cell::sync::Lazy;
use pb::qa::qa_service_client::QaServiceClient;
use serde::{Deserialize, Serialize};
use std::path::Path;

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

// config read and init app config
pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let config_dir = std::env::var("QA_CONFIG_DIR").unwrap_or("./".to_string());
    let filename = Path::new(config_dir.as_str()).join("app-gw.yaml");
    println!("filename:{:?}", filename);
    let c = Config::load(filename);

    // read config to struct
    let conf: AppConfig = serde_yaml::from_str(c.content()).unwrap();
    // 开发过程中，可以取消下面的注释
    if conf.app_debug {
        println!("conf:{:?}", conf);
    }

    conf
});
