use codingbat::warmup::diff21::diff_21;

#[cfg(test)]
mod diff21 {
    use super::*;

    #[test]
    fn diff_under_21() {
        assert_eq!(diff_21(10), 11)
    }

    #[test]
    fn diff_exact_21() {
        assert_eq!(diff_21(21), 0)
    }

    #[test]
    fn diff_obove_21() {
        assert_eq!(diff_21(30), 18)
    }

    #[test]
    fn diff_negative() {
        assert_eq!(diff_21(-1), 22)
    }

    #[test]
    fn diff_much_above() {
        assert_eq!(diff_21(50), 58)
    }
}
