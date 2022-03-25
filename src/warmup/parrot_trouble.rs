/*
https://codingbat.com/prob/p166884

We have a loud talking parrot. The "hour" parameter is the current hour time in the range 0..23. We are in trouble if the parrot is talking and the hour is before 7 or after 20. Return True if we are in trouble.


parrot_trouble(True, 6) → True
parrot_trouble(True, 7) → False
parrot_trouble(False, 6) → False

*/

pub fn parrot_trouble(talking: bool, hour: u8) -> bool {
    if (hour < 7 || hour > 20) && talking == true {
        println!("{}", hour);
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn talking_before_7_test() {
        assert_eq!(true, parrot_trouble(true, 4));
    }

    #[test]
    fn talking_between_7_and_20_test() {
        assert_eq!(false, parrot_trouble(true, 12));
    }

    #[test]
    fn talking_after_20_test() {
        assert_eq!(true, parrot_trouble(true, 23));
    }

    #[test]
    fn talking_at_7_test() {
        assert_eq!(false, parrot_trouble(true, 7));
    }

    #[test]
    fn talking_at_20_test() {
        assert_eq!(false, parrot_trouble(true, 20));
    }

    #[test]
    fn not_talking_before_7_test() {
        assert_eq!(false, parrot_trouble(false, 12));
    }

    #[test]
    fn not_talking_between_7_and_20_test() {
        assert_eq!(false, parrot_trouble(false, 12));
    }

    #[test]
    fn not_talking_after_20_test() {
        assert_eq!(false, parrot_trouble(false, 22));
    }

    #[test]
    fn not_talking_at_7_test() {
        assert_eq!(false, parrot_trouble(false, 7));
    }

    #[test]
    fn not_talking_at_20_test() {
        assert_eq!(false, parrot_trouble(false, 20));
    }
}
