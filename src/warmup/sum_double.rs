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
