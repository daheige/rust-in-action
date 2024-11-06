// 定义utils模块，用于图片压缩、裁剪、旋转操作
mod utils;

// 引入structopt
use structopt::StructOpt;

// 定义CLI命令行参数
#[derive(StructOpt, Debug)]
struct Params {
    // 原图片文件路径
    #[structopt(
        short = "i",
        long,
        default_value = "test.jpeg",
        help = "original picture path"
    )]
    input_path: String,

    // 压缩后的图片路径
    #[structopt(
        short = "o",
        long,
        default_value = "output.png",
        help = "converted picture path"
    )]
    out_path: String,

    // 图片压缩、裁剪、旋转三种操作，分别对应0,1,2，默认压缩图片
    #[structopt(
        short = "a",
        long,
        default_value = "0",
        help = "executed action,eg:0=compress,1=crop,2=rotate90"
    )]
    action: u8,

    // 压缩百分比数字，例如:60
    #[structopt(
        short = "q",
        long,
        default_value = "60",
        help = "the picture compression percentage number,eg:60"
    )]
    quality: u8,

    // 图片裁剪的起始位置(x,y)，单位px
    #[structopt(
        short = "x",
        long,
        default_value = "0",
        help = "the starting coordinate point x of the picture crop"
    )]
    x: u32,

    #[structopt(
        short = "y",
        long,
        default_value = "0",
        help = "the starting coordinate point x of the picture crop"
    )]
    y: u32,

    // 图片裁剪的宽度和高度，单位px
    #[structopt(short = "w", long, default_value = "100", help = "crop width")]
    width: u32,

    #[structopt(short = "h", long, default_value = "100", help = "crop height")]
    height: u32,

    #[structopt(
        short = "r",
        long,
        default_value = "90",
        help = "the degrees of clockwise rotation,eg:90,180,270"
    )]
    rotate_degrees: u16,
}

fn main() {
    // 获取CLI命令行参数，并解析到结构体Params对象p中
    let p = Params::from_args();
    println!("params:{:#?}", p);
    match p.action {
        0 => {
            // 图片压缩
            println!("compress image begin");
            utils::compress_image(p.input_path, p.out_path, p.quality);
            println!("compress image end");
        }
        1 => {
            // 图片裁剪
            println!("crop image begin");
            utils::crop_image(p.input_path, p.out_path, (p.x, p.y), p.width, p.height);
            println!("crop image end");
        }
        2 => {
            // 图片顺时针旋转
            println!("rotate image begin");
            utils::rotate_image(p.input_path, p.out_path, p.rotate_degrees);
            println!("rotate image end");
        }
        _ => {
            // 不支持的action
            println!("action invalid");
        }
    }
}
