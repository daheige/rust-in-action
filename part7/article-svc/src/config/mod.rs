mod app;
pub mod mysql;
pub mod xredis;

// use重新导出
pub use app::{AppState, APP_CONFIG, REDIS_POOL};
