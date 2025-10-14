fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    println!("r1: {}", r1);
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("block r1: {}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2: {}", r2);

    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题,存在不可变借用
    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    // 不可变引用和可变引用没有同时发生
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{r1} and {r2}");
    println!("{r1} and {r2}");

    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
