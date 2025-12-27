pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod front_of_house;

pub fn eat_at_restaurant() {
    front_of_house::add_to_waitlist();
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
    fn it_works2() {
        eat_at_restaurant();
    }
}
