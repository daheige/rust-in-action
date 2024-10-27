use image::ImageFormat;
use std::fmt::Debug;
use std::path::Path;
use std::u32;
use structopt::StructOpt;

// 定义CLI命令行参数
#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(short = "i", long, default_value = "test.jpeg")]
    input_path: String, // 原图片文件路径

    #[structopt(short = "o", long, default_value = "output.png")]
    out_path: String, // 压缩后的图片路径

    #[structopt(short = "c", long, default_value = "0")]
    action: u8, // 图片压缩、裁剪、翻转三种操作，分别对应0,1,1，默认压缩图片

    // 图片裁剪的坐标起始位置x,y
    #[structopt(short = "x", long, default_value = "0")]
    x: u32,

    #[structopt(short = "y", long, default_value = "0")]
    y: u32,

    #[structopt(short = "w", long, default_value = "100")]
    weight: u32,

    #[structopt(short = "h", long, default_value = "100")]
    height: u32,

    #[structopt(short = "q", long, default_value = "60")]
    quality: u8, // 压缩百分比数字，例如:60
}

fn main() {
    let p = Params::from_args();
    println!("params:{:#?}", p);
    if p.action == 0 {
        println!("compress image begin");
        compress_image(p.input_path, p.out_path, p.quality);
        println!("compress image finish");
    } else {
        println!("crop image begin");
        crop_image(p.input_path, p.out_path, (p.x, p.y), p.weight, p.height);
        println!("crop image finish");
    }
}

fn check_image_type<P: AsRef<Path> + Debug>(input_path: P) -> Result<(), String> {
    // 根据路径获取图片的类型
    let img_type = ImageFormat::from_path(&input_path).expect("failed to get image type");

    // 对图片格式进行过滤
    match img_type {
        ImageFormat::Jpeg | ImageFormat::Png | ImageFormat::Gif => {
            println!("input_path:{:?} image type:{:?}", input_path, img_type);
            Ok(())
        }
        _ => Err("unsupported image type".to_string()),
    }
}

// 将输入的图片按照一定的百分比进行压缩
fn compress_image<P: AsRef<Path> + Debug>(input_path: P, out_path: P, quality: u8) {
    // 验证图片类型
    check_image_type(&input_path).expect("invalid image");

    // 设置压缩比例因子，例如：0.3表示压缩30%
    let compress_factor = quality as f32 / 100.0;
    println!("compress_factor:{}", compress_factor);

    // 打开原图片
    let img = image::open(input_path).expect("failed to open image");

    // 计算压缩后的图片宽度和高度
    // 这里先将宽度和高度通过as关键字转换为f32类型，再乘以对应的压缩比例因子，最后再转换u32类型
    // 获取原图片的宽度和高度
    let src_width = img.width();
    let src_height = img.height();
    let compress_width = (src_width as f32 * compress_factor) as u32;
    let compress_height = (src_height as f32 * compress_factor) as u32;
    println!(
        "src_width:{} src_height:{} compress width:{} height:{}",
        src_width, src_height, compress_width, compress_height
    );

    // 调整图片大小，按照 Gaussian Filter 算法压缩图片
    let compress_img = img.resize(compress_width, compress_height, image::imageops::Gaussian);

    // 保存压缩后的图片
    compress_img.save(out_path).expect("failed to save image");
}

fn crop_image<P: AsRef<Path> + Debug>(
    input_path: P,
    out_path: P,
    point: (u32, u32),
    width: u32,
    height: u32,
) {
    // 验证图片类型
    check_image_type(&input_path).expect("invalid image");

    // 打开原图片
    let mut img = image::open(input_path).expect("failed to open image");

    // 获取原图片的宽度和高度
    let src_width = img.width();
    let src_height = img.height();

    // 设置裁剪的起开始坐标点(m,n)
    let mut x = point.0;
    if x >= src_width {
        x = src_width;
    }
    let mut y = point.1;
    if y >= src_height {
        y = src_height;
    }

    // 设置裁剪的宽度和高度
    let mut w = width;
    if w >= src_width {
        w = src_width;
    }
    let mut h = height;
    if h >= src_height {
        h = src_height;
    }

    println!(
        "src_width:{} src_height:{} crop image point({},{}) width:{} height:{}",
        src_width, src_height, x, y, w, h
    );

    // 裁剪图片
    img.crop(x, y, w, h)
        .save(out_path)
        .expect("failed to crop image");
}

// 将图片旋转90度
fn rotate_image<P: AsRef<Path> + Debug>(input_path: P, out_path: P) {
    // 验证图片类型
    check_image_type(&input_path).expect("invalid image");

    let img = image::open(input_path).unwrap();
    let rotated = img.rotate90(); // 图片旋转90度
    rotated.save(out_path).unwrap(); // 保存为原路径
    println!("rotate90 success");
}
