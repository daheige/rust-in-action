use std::borrow::Cow;

fn shout(input: &str) -> Cow<'_, str> {
    if input.chars().any(|c| c.is_lowercase()) {
        // 需要修改 -> 克隆一份并转为大写（Owned）
        Cow::Owned(input.to_uppercase())
    } else {
        // 不需要修改 -> 直接借用（Borrowed）
        Cow::Borrowed(input)
    }
}

fn takes_str(s: &str) {
    println!("{s}");
}

fn main() {
    let a = shout("hello"); // Owned("HELLO")
    let b = shout("WORLD"); // Borrowed("WORLD")
    println!("{a} {b}"); // HELLO WORLD

    let c: Cow<str> = Cow::Borrowed("hi");
    takes_str(&c); // 自动 Deref
    println!("{}", c.len()); // 直接用 str 的方法
}
