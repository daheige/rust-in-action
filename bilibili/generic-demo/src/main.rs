fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {:?}, p.y = {:?}", p.x, p.y);

    let f = Point { x: 1.0, y: 4.0 };
    println!("f.x = {:?}, f.y = {:?}", f.x, f.y);

    let f = Point2 { x: 1, y: 4.0 };
    println!("point2 f.x = {:?}, f.y = {:?}", f.x, f.y);

    let f = Point2 { x: 1, y: 4 };
    println!("Point2 f.x = {:?}, f.y = {:?}", f.x, f.y);

    let p = Point { x: 1, y: 4 };
    let x = p.x();
    println!("x = {}", x);
}
