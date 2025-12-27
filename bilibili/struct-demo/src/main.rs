struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User2<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let u = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", u.username);
    println!("{}", u.email);
    println!("{}", u.sign_in_count);

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1000,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);

    let u = build_user( String::from("anotheremail@example.com"),String::from("someusername123"));
    println!("{}", u.email);
    println!("{}", u.sign_in_count);

    let u = User {
        email: String::from("another2@example.com"),
        ..user1
    };

    println!("{}", u.email);
    println!("{}", u.username);
    println!("{}", u.sign_in_count);

    let c = Color(10, 0, 102);
    println!("c0={},c1={}",c.0, c.1);

    let Point(x, y, z) = Point(10, 20, 30);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let u = User2{
        active: true,
        username: "someusername123",
        email:"someone@example.com",
        sign_in_count: 1000,
    };
    println!("{}", u.email);
    println!("{}", u.username);
    println!("{}", u.sign_in_count);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let r = Rectangle::square(20);
    println!(
        "The area of the rectangle is {} square pixels.",
        r.area()
    );

}
