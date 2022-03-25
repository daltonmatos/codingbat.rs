/*

https://codingbat.com/prob/p124984

Given 2 ints, a and b, return True if one if them is 10 or if their sum is 10.


makes10(9, 10) → True
makes10(9, 9) → False
makes10(1, 9) → True

*/

pub fn makes10(a: i32, b: i32) -> bool {
    let numbers = vec![a, b];
    if numbers.contains(&10) || a + b == 10 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_is_10_test() {
        assert_eq!(true, makes10(10, 0));
        assert_eq!(true, makes10(0, 10));
        assert_eq!(true, makes10(20, -10));
        assert_eq!(true, makes10(5, 5));
        assert_eq!(true, makes10(-15, 25));
        assert_eq!(true, makes10(-3, 13));
    }

    #[test]
    fn a_is_10_test() {
        assert_eq!(true, makes10(10, 3));
    }

    #[test]
    fn b_is_10_test() {
        assert_eq!(true, makes10(3, 10));
    }
    #[test]
    fn both_are_10_test() {
        assert_eq!(true, makes10(10, 10));
    }
    #[test]
    fn none_is_10_not_sum_10_test() {
        assert_eq!(false, makes10(4, 9));
    }
}
