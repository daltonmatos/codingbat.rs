/*
https://codingbat.com/prob/p116624

Given an int n, return the absolute difference between n and 21,
except return double the absolute difference if n is over 21.


diff21(19) → 2
diff21(10) → 11
diff21(21) → 0
*/

pub fn diff_21(target: i32) -> i32 {
    let _diff: i32 = 21 - target;

    if target > 21 {
        return (_diff * 2).abs();
    }
    _diff.abs()
}

#[cfg(test)]
mod tests {
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
