mod garden;
// use crate::garden::vegetables::Asparagus;
use crate::garden::Asparagus;
use std::collections::HashMap;

fn deliver_order() {
    println!("Hi from deliver_order!");
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("order1: {:?}, order2: {:?}", order1, order2);
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        println!("Hi from cook_order!");
    }
}

fn main() {
    let a = Asparagus{
        msg : String::from("hello world"),
    };
    println!("msg: {}", a.msg);

    back_of_house::fix_incorrect_order();
    eat_at_restaurant();

    eat_at_restaurant2();

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map: {:?}", map);
}
