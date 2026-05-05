/// Find the First `true` in a Sorted Boolean Array.
///
/// `arr` is partitioned into a (possibly empty) prefix of `false` values
/// followed by a (possibly empty) suffix of `true` values. Return the
/// index of the first `true`, or `-1` if no `true` exists.
///
/// Runs in `O(log n)` time, `O(1)` space.
///
/// # Examples
///
/// ```
/// use leetcode::binary::first_true_in_sorted_boolean_array::first_true;
///
/// assert_eq!(first_true(&[false, false, true, true, true]), 2);
/// assert_eq!(first_true(&[false, false, false]), -1);
/// assert_eq!(first_true(&[true, true, true]), 0);
/// assert_eq!(first_true(&[]), -1);
/// ```
pub fn first_true(arr: &[bool]) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] {
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
    fn example_case() {
        assert_eq!(first_true(&[false, false, true, true, true]), 2);
    }

    #[test]
    fn all_false() {
        assert_eq!(first_true(&[false, false, false]), -1);
    }

    #[test]
    fn all_true() {
        assert_eq!(first_true(&[true, true, true]), 0);
    }

    #[test]
    fn empty_array() {
        assert_eq!(first_true(&[]), -1);
    }

    #[test]
    fn single_false() {
        assert_eq!(first_true(&[false]), -1);
    }

    #[test]
    fn single_true() {
        assert_eq!(first_true(&[true]), 0);
    }

    #[test]
    fn boundary_at_end() {
        assert_eq!(first_true(&[false, false, false, true]), 3);
    }

    #[test]
    fn boundary_at_start() {
        assert_eq!(first_true(&[true, true, true, true]), 0);
    }

    #[test]
    fn two_elements_split() {
        assert_eq!(first_true(&[false, true]), 1);
    }
}
