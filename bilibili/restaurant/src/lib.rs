pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            println!("Hi from waitlist!");
        }

        fn seat_at_table() {
            println!("Hi from seat_at_table!");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Hi from take_order!");
        }

        fn serve_order() {
            println!("Hi from serve_order!");
        }

        fn take_payment() {
            println!("Hi from take_payment!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn take_order_test(){
        front_of_house::serving::take_order();
    }
}
