use crate::warmup::sleep_in::sleep_in;

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
