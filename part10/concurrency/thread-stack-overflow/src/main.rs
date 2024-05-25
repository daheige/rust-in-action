use std::thread;

fn main() {
    // 过小的堆栈大小
    let builder2 = thread::Builder::new()
        .name("worker thread".to_string())
        .stack_size(4 * 1024); // 4kb大小
    let handler2 = builder2.spawn(|| {
        panic!("oops!");
    });
    let child_status = handler2.unwrap().join();
    println!("child status:{:?}", child_status);
}
