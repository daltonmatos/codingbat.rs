/*
https://codingbat.com/prob/p141905

Given two int values, return their sum. Unless the two values are the same, then return double their sum.


sum_double(1, 2) → 3
sum_double(3, 2) → 5
sum_double(2, 2) → 8
*/

pub fn sum_double(a: i32, b: i32) -> i32 {
    if a != b {
        a + b
    } else {
        (a + b) * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_two_zeroes() {
        assert_eq!(0, sum_double(0, 0));
    }

    #[test]
    fn sum_nao_igauis() {
        assert_eq!(5, sum_double(3, 2));
        assert_eq!(9, sum_double(4, 5));
    }

    #[test]
    fn sum_negativo_test() {
        assert_eq!(10, sum_double(15, -5))
    }

    #[test]
    fn sum_negativo_com_zero_test() {
        assert_eq!(-1, sum_double(-1, 0))
    }

    #[test]
    fn sum_negativo_com_positivo_iguais_test() {
        assert_eq!(0, sum_double(5, -5))
    }

    #[test]
    fn sum_iguais_test() {
        assert_eq!(12, sum_double(3, 3))
    }
}
