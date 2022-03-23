/*

https://codingbat.com/prob/p120546

We have two monkeys, a and b, and the parameters a_smile and b_smile indicate if each is smiling. We are in trouble if they are both smiling or if neither of them is smiling. Return True if we are in trouble.


monkey_trouble(True, True) → True
monkey_trouble(False, False) → True
monkey_trouble(True, False) → False
*/

pub fn monkey_trouble(a_smile: bool, b_smile: bool) -> bool {
    (a_smile && b_smile) || (!a_smile && !b_smile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false_false() {
        assert_eq!(true, monkey_trouble(false, false))
    }

    #[test]
    fn test_false_true() {
        assert_eq!(false, monkey_trouble(false, true))
    }

    #[test]
    fn test_true_false() {
        assert_eq!(false, monkey_trouble(true, false))
    }

    #[test]
    fn test_true_true() {
        assert_eq!(true, monkey_trouble(true, true))
    }
}
