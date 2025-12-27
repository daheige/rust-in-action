#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        println!("{:?}", &self);
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // 匹配成功
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home kind:{:?}",home.kind);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("loopback kind:{:?}",loopback.kind);
    println!("loopback addr:{:?}",loopback.address);

    let r = IpAddr2::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddr2::V6(String::from("::1"));

    match r {
        IpAddr2::V4(ref addr) => println!("v4 addr string:{:?}",addr),
        IpAddr2::V6(ref addr) => println!("v6 addr string:{:?}",addr),
    }

    println!("r address:{:?}",r);
    println!("ipv6 addr:{:?}",ipv6);

    let m = Message::Write(String::from("hello"));
    // println!("m message:{:?}",m);
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    println!("some_char:{:?}",some_char);
    println!("some_number:{:?}",some_number);
    println!("absent_number:{:?}",absent_number);

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
    println!("x:{:?}",x);

    let c = Coin::Dime;
    let c_val = value_in_cents(c);
    println!("c_val:{}",c_val);

    let state = UsState::Alaska;
    let c = Coin::Quarter(state);
    let c_val = value_in_cents(c);
    println!("c_val:{}",c_val);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(10u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Dime;
    // let coin = Coin::Quarter(UsState::Alabama);
    // 如果匹配到是Quarter 类型就输出，否则就count+1
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("count {count}");

    let state = UsState::Alabama;
    let coin = Coin::Quarter(state);
    let r = describe_state_quarter(coin);
    println!("{:?}",r);
}
