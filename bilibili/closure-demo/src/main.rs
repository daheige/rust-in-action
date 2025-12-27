use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let list = vec![1, 2, 3];
    println!("before list:{:?}", list);
    // 开启一个线程，并通过move关键字将list的所有权移动到闭包中
    thread::spawn(move || {
        println!("from thread list:{:?}", list);
    })
    .join()
    .unwrap();

    // 由于list已通过move将所有权移动到了闭包中，在线程中已经消耗了，所以这里不能在使用了
    // ^^^^ value borrowed here after move
    // println!("after thread list:{:?}", list);

    // 定义一个可变类型的list
    let mut list = [
        Rectangle {
            width: 20,
            height: 40,
        },
        Rectangle {
            width: 10,
            height: 30,
        },
        Rectangle {
            width: 30,
            height: 50,
        },
    ];

    // sort_by_key 被定义为接收一个 FnMut 闭包的原因是它会多次调用这个闭包：对 slice 中的每个元素调用一次。
    // 闭包 |r| r.width 不捕获、修改或将任何东西移出它的环境，所以它满足 trait bound 的要求。
    list.sort_by_key(|r| r.width); // 从小到大排序
    println!("sorted list:{:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    // list.sort_by_key(|r| {
    //     // 这里的闭包是一个FnOnce trait类型的闭包
    //     // 这个value所有权已经被消耗了，就不能再使用，因此报错
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    let mut num_sort_operations = 0; // 统计闭包执行排序的次数
    // sort_by_key 被定义为接收一个 FnMut 闭包
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("after list:{list:#?}, sorted in {num_sort_operations} operations");
}
