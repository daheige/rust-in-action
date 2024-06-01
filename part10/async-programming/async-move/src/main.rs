use futures::Future;
use futures::executor::block_on;

// async fn 中使用async块
async fn blocks() {
    let greet = "hello,world".to_string();

    let future_one = async {
        println!("{greet}");
    };

    let future_two = async {
        println!("{greet}");
    };

    // 运行这2个futures等待它们执行完成，会连续输出两次"hello,world"
    let ((), ()) = futures::join!(future_one, future_two);
}

// async move块
fn move_block() -> impl Future<Output = ()> {
    let lang = "rust".to_string();
    // 在async块中使用move，会将lang的所有权转移到async块中
    // async move作为一个整体返回，类型是Future
    async move {
        println!("{}", lang);
    }
}

fn main() {
    // 运行async fn函数
    block_on(blocks());

    // move_block函数会返回一个Future，然后使用block_on执行future直到完成
    let future = move_block();
    block_on(future);
}
