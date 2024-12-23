use std::{thread, time};

fn main() {
    let handler1 = thread::spawn(|| {
        for c in 'a'..='z' {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", c);
        }

        println!("");
    });

    let handler2 = thread::spawn(|| {
        for i in 1..=9 {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", i);
        }

        println!("");
    });

    // 通过join方法等待线程执行完毕
    let _ = handler1.join();
    let _ = handler2.join();
    println!("the two threads are finished");
    println!("main thread will exit");
}
