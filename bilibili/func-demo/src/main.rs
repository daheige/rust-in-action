fn main() {
    println!("Hello, world!");
    let y = plus_one(3);
    println!("{}", y);
    let z = plus_one(1);
    println!("{}", z);
    hello("daheige");
    let s = String::from("daheige");
    hello(&s);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


