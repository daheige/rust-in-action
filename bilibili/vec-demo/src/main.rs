#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v:{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(val) => println!("The element is {val}"),
        None => println!("There is no element."),
    }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    //
    // println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v:{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row:{:?}", row);

    // {
    //     let v2 = vec![1, 2, 3, 4];
    //
    //     // 使用 v2
    // } // <- 在这里 v2 离开作用域并被释放
    // println!("v:{:?}", v2);
}
