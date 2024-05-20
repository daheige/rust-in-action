// 定义members会员信息、points_details积分明细、points_message消息明细等基本模块
pub mod members;
pub mod points_details;
mod points_message;

// 重新导出PointsMessage
pub use points_message::PointsMessage;
