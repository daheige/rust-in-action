// 引入my_addr包
use my_addr;

#[test]
fn test_add() {
    assert_eq!(my_addr::add(1, 1), 2);
    println!("add2(2,2) = {}", my_addr::add(2, 2));
    println!("add2(2,3) = {}", my_addr::add(2, 3));
}
