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
mod tests;
