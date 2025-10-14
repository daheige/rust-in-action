use image::ImageFormat;
use std::fmt::Debug;
use std::path::Path;

// 根据文件名检查文件格式是否是PNG,JPEG,GIF三种类型的图片
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

// 图片压缩
// 将输入的图片按照一定的百分比进行压缩
pub fn compress_image<P: AsRef<Path> + Debug>(input_path: P, out_path: P, quality: u8) {
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

// 图片裁剪
pub fn crop_image<P: AsRef<Path> + Debug>(
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
    if w >= src_width - x {
        w = src_width - x;
    }
    let mut h = height;
    if h >= src_height - y {
        h = src_height - y;
    }

    println!(
        "src_width:{} src_height:{} crop image point({},{}) width:{} height:{}",
        src_width, src_height, x, y, w, h
    );

    // 裁剪图片，并保存到out_path路径
    img.crop(x, y, w, h)
        .save(out_path)
        .expect("failed to crop image");
}

// 图片旋转
pub fn rotate_image<P: AsRef<Path> + Debug>(input_path: P, out_path: P, rotate_degrees: u16) {
    // 验证图片类型
    check_image_type(&input_path).expect("invalid image");

    // 打开图片
    let img = image::open(input_path).unwrap();

    // 顺时针旋转图片
    match rotate_degrees {
        90 => {
            let rotated = img.rotate90();
            rotated.save(out_path).expect("failed to rotate image"); // 保存图片
        }
        180 => {
            let rotated = img.rotate90();
            rotated.save(out_path).expect("failed to rotate image"); // 保存图片
        }
        270 => {
            let rotated = img.rotate90();
            rotated.save(out_path).expect("failed to rotate image"); // 保存图片
        }
        _ => {
            panic!("rotate_degrees invalid")
        }
    }

    println!("rotate90 success");
}
