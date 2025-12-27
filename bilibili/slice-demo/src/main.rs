fn main() {
    let mut s = String::from("hello world rust");
    let word = first_word(&s);
    s.clear(); // 这清空了字符串，使其等于 ""

    println!("s usize:{}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}", hello, world);
    let slice = &s[..2];
    println!("slice:{}", slice);
    let slice = &s[3..];
    println!("slice:{}", slice);
    let slice = &s[..];
    println!("slice:{}", slice);

    let s = String::from("hello world rust");
    let world = first_word2(&s);
    println!("world:{world}");

    let w = first_word2("hello world"); // "hello world" 本身就是一个&str
    println!("w:{}", w);

    // let mut s = String::from("hello world");

    // let word = first_word2(&s);
    //
    // s.clear(); // 错误！
    //
    // println!("the first word is: {word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
