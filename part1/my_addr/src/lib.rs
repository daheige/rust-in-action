pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    let x = 1;
    let y = 2;
    assert_eq!(add(x, y), 3);
    println!("add2(1,2) = {}", add(x, y));
}
