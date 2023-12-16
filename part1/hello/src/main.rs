fn main() {
    println!("Hello, world!");
    println!("add(1,2) = {}", add(1, 2));
    println!("add(2,3) = {}", add(2, 3));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    let x = 1;
    let y = 2;
    assert_eq!(add(x, y), 3);
}
