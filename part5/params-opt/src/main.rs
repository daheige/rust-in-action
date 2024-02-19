use structopt::StructOpt;

// 使用StructOpt trait
#[derive(StructOpt, Debug)]
struct ParamsOpt {
    // short表示单命名的方式，long是长命名方式，default_value属性可设置默认值
    #[structopt(short, long, default_value = "")]
    name: String,
    #[structopt(short, long, default_value = "0")]
    first_num: i32,

    #[structopt(short, long, default_value = "0.0")]
    second_num: f64,
}

fn main() {
    let opt = ParamsOpt::from_args();
    println!("{:#?}", opt);
    println!(
        "name:{} first_num:{} second_num:{}",
        opt.name, opt.first_num, opt.second_num
    );
}
