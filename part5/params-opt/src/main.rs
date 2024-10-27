use std::path;
use structopt::StructOpt;

// 使用StructOpt派生宏自动生成参数获取和参数解析的代码
#[derive(StructOpt, Debug)]
struct ParamsOpt {
    // short属性可以设置单命名的方式，long设置长命名方式，
    // default_value可以设置默认值
    #[structopt(short = "n", long, default_value = "")]
    name: String,

    // 文件路径格式，例如/tmp/test.md
    #[structopt(short = "i", long, parse(from_os_str), default_value = "./")]
    input: path::PathBuf,

    #[structopt(short = "f", long, default_value = "0")]
    first_num: i32,

    #[structopt(short = "s", long, default_value = "0.0")]
    second_num: f64,
}

fn main() {
    let opt = ParamsOpt::from_args();
    println!("{:#?}", opt);
    println!(
        "name:{} first_num:{} second_num:{} filename:{:?}",
        opt.name, opt.first_num, opt.second_num, opt.input
    );
}
