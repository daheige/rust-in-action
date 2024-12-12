fn main() {
    // 声明一个整数类型的向量
    let data = vec![1, 2, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        // 在闭包中获取data变量的不可变引用，然后遍历data每个元素
        for i in &data {
            println!("{}", i);
        }
    });

    // 在闭包外部继续使用data，程序无法正常运行，因为data所有权已经被移动到了闭包函数中
    println!("data:{:?}", data);
    handle.join().unwrap();
}
