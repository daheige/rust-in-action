fn main() {
    // 声明一个整数类型的向量
    let data = vec![1, 2, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        for i in data {
            println!("{}", i);
        }
    });

    // println!("data:{:?}", data);
    // 调用join方法等待线程执行完毕
    handle.join().unwrap();
}
