use clap::Parser;

// 通过derive注解的方式实现参数获取与解析
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ParamsOpt {
    // short表示单命名的方式，long是长命名方式，default_value_t可指定默认值
    #[arg(short, long, default_value_t = String::from(""))]
    name: String,

    #[arg(short, long, default_value_t = 0)]
    first_num: i32,

    #[arg(short, long, default_value_t = 0.0)]
    second_num: f64,
}

fn main() {
    // 在ParamsOpt上面调用parse函数实现CLI命令行参数获取和参数解析
    let opt = ParamsOpt::parse();
    println!("{:#?}", opt);

    // 输出opt结构体字段
    println!(
        "name:{} first_num:{} second_num:{}",
        opt.name, opt.first_num, opt.second_num
    );
}
