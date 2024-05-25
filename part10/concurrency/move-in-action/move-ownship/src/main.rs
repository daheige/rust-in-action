fn main() {
    // 声明一个整数类型的向量
    let data = vec![1, 2, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        for i in data {
            println!("{}", i);
        }
    });

    // println!("data:{:?}", data);
    handle.join().unwrap();
}
