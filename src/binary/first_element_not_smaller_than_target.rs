/// First Element Not Smaller Than Target.
///
/// Given `arr` sorted in non-decreasing order and a `target`, return the
/// index of the first element greater than or equal to `target`.
///
/// Assumes such an element always exists.
///
/// Runs in `O(log n)` time, `O(1)` space.
///
/// # Examples
///
/// ```
/// use leetcode::binary::first_element_not_smaller_than_target::first_not_smaller;
///
/// assert_eq!(first_not_smaller(&[1, 3, 3, 5, 8, 8, 10], 2), 1);
/// assert_eq!(first_not_smaller(&[2, 3, 5, 7, 11, 13, 17, 19], 6), 3);
/// ```
pub fn first_not_smaller(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if left == arr.len() { -1 } else { left as i32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case_1() {
        assert_eq!(first_not_smaller(&[1, 3, 3, 5, 8, 8, 10], 2), 1);
    }

    #[test]
    fn example_case_2() {
        assert_eq!(first_not_smaller(&[2, 3, 5, 7, 11, 13, 17, 19], 6), 3);
    }

    #[test]
    fn target_equals_element() {
        assert_eq!(first_not_smaller(&[1, 3, 3, 5, 8, 8, 10], 3), 1);
    }

    #[test]
    fn target_equals_duplicate_picks_first() {
        assert_eq!(first_not_smaller(&[1, 3, 3, 5, 8, 8, 10], 8), 4);
    }

    #[test]
    fn target_smaller_than_all() {
        assert_eq!(first_not_smaller(&[5, 6, 7], 1), 0);
    }

    #[test]
    fn target_equals_first() {
        assert_eq!(first_not_smaller(&[5, 6, 7], 5), 0);
    }

    #[test]
    fn target_equals_last() {
        assert_eq!(first_not_smaller(&[1, 2, 3, 4, 5], 5), 4);
    }

    #[test]
    fn single_element_match() {
        assert_eq!(first_not_smaller(&[42], 42), 0);
    }

    #[test]
    fn single_element_target_smaller() {
        assert_eq!(first_not_smaller(&[42], 1), 0);
    }
}
