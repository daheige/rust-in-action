use murmurhash32::murmurhash3;

fn main() {
    let url = "https://github.com/daheige?tab=repositories";

    println!("Hello, world!");
    println!("{}",murmurhash3(url.as_bytes()));

    let num = murmurhash3(url.as_bytes());
    // 转换为62进制的字符串
    let s = base62::encode(num);
    println!("num: {},base62 str:{}",num,s);
    println!("s decode to num:{}",base62::decode(s).unwrap());

    let url = "https://github.com/daheige?tab=repositories&a=1";
    let num = murmurhash3::murmurhash3_x86_32(url.as_bytes(),0);
    let s = base62::encode(num);
    println!("num: {},base62 str:{}",num,s);
    println!("s decode to num:{}",base62::decode(s).unwrap());

    let (_num,num2) = murmurhash3::murmurhash3_x64_128(url.as_bytes(),0);
    let s = base62::encode(num2);
    println!("num2: {},base62 str:{}",num2,s);
    println!("s decode to num:{}",base62::decode(s).unwrap());
}
