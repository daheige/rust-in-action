use chrono::Local;
fn main() {
    // 按照年-月-日 时分秒格式输出
    let fmt = "%Y-%m-%d %H:%M:%S";
    let current_time = Local::now().format(fmt).to_string();

    // 输出
    println!("current_time:{}", current_time);
}
