/// Square Root Estimation.
///
/// Given a non-negative integer `n`, return the integer part of its
/// square root (i.e. floor(sqrt(n))) without using floating-point or
/// built-in square-root functions.
///
/// Runs in `O(log n)` time, `O(1)` space.
///
/// # Examples
///
/// ```
/// use leetcode::binary::square_root_estimation::square_root;
///
/// assert_eq!(square_root(16), 4);
/// assert_eq!(square_root(8), 2);
/// ```
pub fn square_root(n: i32) -> i32 {
    let mut left: i64 = 1;
    let mut right: i64 = n as i64;
    let n64 = n as i64;

    while left < right {
        let mid = left + (right - left) / 2;
        if mid * mid < n64 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    let ans = if left * left > n64 { left - 1 } else { left };
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_square_16() {
        assert_eq!(square_root(16), 4);
    }

    #[test]
    fn non_perfect_square_8() {
        assert_eq!(square_root(8), 2);
    }

    #[test]
    fn zero() {
        assert_eq!(square_root(0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(square_root(1), 1);
    }

    #[test]
    fn two() {
        assert_eq!(square_root(2), 1);
    }

    #[test]
    fn three() {
        assert_eq!(square_root(3), 1);
    }

    #[test]
    fn perfect_square_9() {
        assert_eq!(square_root(9), 3);
    }

    #[test]
    fn just_below_perfect_square() {
        assert_eq!(square_root(15), 3);
    }

    #[test]
    fn just_above_perfect_square() {
        assert_eq!(square_root(17), 4);
    }

    #[test]
    fn perfect_square_100() {
        assert_eq!(square_root(100), 10);
    }

    #[test]
    fn perfect_square_10000() {
        assert_eq!(square_root(10_000), 100);
    }

    #[test]
    fn large_value() {
        assert_eq!(square_root(2_147_395_599), 46_339);
    }

    #[test]
    fn i32_max() {
        assert_eq!(square_root(i32::MAX), 46_340);
    }
}
