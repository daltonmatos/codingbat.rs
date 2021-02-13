/* https://codingbat.com/prob/p187868 */
/*
   The parameter weekday is true if it is a weekday, and the parameter vacation is true if we are on vacation.
   We sleep in if it is not a weekday or we're on vacation. Return true if we sleep in.


   sleepIn(false, false) → true
   sleepIn(true, false) → false
   sleepIn(false, true) → true*
*/

fn sleep_in(weekday: bool, vacation: bool) -> bool {
    return vacation || (!weekday);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_if_is_vacation_and_not_weekday() {
        assert_eq!(sleep_in(false, true), true)
    }

    #[test]
    fn sleep_if_is_vacation_and_weekday() {
        assert_eq!(sleep_in(true, true), true)
    }

    #[test]
    fn sleep_if_is_weekday_not_vacation() {
        assert_eq!(sleep_in(true, false), false)
    }

    #[test]
    fn sleep_if_not_week_day_and_not_vacation() {
        assert_eq!(sleep_in(false, false), true)
    }
}
