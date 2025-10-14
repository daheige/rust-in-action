async fn hello_cat() {
    println!("hello, kitty!");
}

async fn say() {
    hello_cat();
}

fn main() {
    say();
}

