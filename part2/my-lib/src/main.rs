use my_lib::max;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("args must be separated by space");
        return;
    }

    let mut nums = vec![];
    for val in args {
        let num: i32 = val.parse().unwrap_or(0);
        nums.push(num);
    }

    let max_num = max(&nums);
    println!("you input nums:{:?} max is {}", nums, max_num);
}
