// 定义模块feed
mod feed;

// 重新导出feed模块
pub use feed::edit;
pub use feed::show;

pub fn summary() {
    println!("called summary function");
}
