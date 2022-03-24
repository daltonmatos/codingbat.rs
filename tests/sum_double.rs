#[cfg(test)]
mod sum_double {
    use codingbat::warmup::sum_double::sum_double;

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
