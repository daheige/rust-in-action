fn main() {
    let mut s = String::new();
    s.push_str("bar");
    println!("{}", s);
    let data = "initial contents";
    let s = data.to_string();
    println!("s:{}", s);
    let s = "initial contents".to_string();
    println!("s:{}", s);

    let mut s = String::from("hello,");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // println!("s1 is {}", s1);
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
    println!("s1:{} s2:{} s3:{}", s1, s2, s3);

    let hello = "Здравствуйте";

    // let s = &hello[0..1];
    for c in hello.chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut s = "hello,foo".to_string();
    s = s.replace("foo", "world");
    println!("{}", s);

    let b = s.contains("hello");
    println!("b is {}", b);
}
