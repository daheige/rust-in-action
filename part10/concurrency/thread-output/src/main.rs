use std::{thread, time};

fn main() {
    thread::spawn(|| {
        for c in 'a'..='z' {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", c);
        }

        println!("");
    });
    thread::spawn(|| {
        for i in 1..=9 {
            thread::sleep(time::Duration::from_millis(100));
            print!("{} ", i);
        }

        println!("");
    });

    // 停顿2s
    thread::sleep(time::Duration::from_secs(2));
}
