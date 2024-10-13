fn main() {
    println!("Hello, world!");
    println!("add(1,2) = {}", add(1, 2));
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
