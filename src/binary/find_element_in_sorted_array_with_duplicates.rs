/// Find Element in Sorted Array with Duplicates.
///
/// Given `arr` sorted in non-decreasing order, return the index of the
/// first occurrence of `target`, or `-1` if absent.
///
/// Runs in `O(log n)` time, `O(1)` space.
///
/// # Examples
///
/// ```
/// use leetcode::binary::find_element_in_sorted_array_with_duplicates::find_first_occurrence;
///
/// assert_eq!(find_first_occurrence(&[1, 3, 3, 3, 3, 6, 10, 10, 10, 100], 3), 1);
/// assert_eq!(find_first_occurrence(&[2, 3, 5, 7, 11, 13, 17, 19], 6), -1);
/// ```
pub fn find_first_occurrence(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left == arr.len() || arr[left] != target {
        -1
    } else {
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case_with_duplicates() {
        assert_eq!(
            find_first_occurrence(&[1, 3, 3, 3, 3, 6, 10, 10, 10, 100], 3),
            1
        );
    }

    #[test]
    fn example_case_absent() {
        assert_eq!(find_first_occurrence(&[2, 3, 5, 7, 11, 13, 17, 19], 6), -1);
    }

    #[test]
    fn target_at_start() {
        assert_eq!(find_first_occurrence(&[3, 3, 3, 5, 7], 3), 0);
    }

    #[test]
    fn target_at_end() {
        assert_eq!(find_first_occurrence(&[1, 2, 3, 4, 5], 5), 4);
    }

    #[test]
    fn target_smaller_than_all() {
        assert_eq!(find_first_occurrence(&[5, 6, 7], 1), -1);
    }

    #[test]
    fn target_larger_than_all() {
        assert_eq!(find_first_occurrence(&[1, 2, 3], 100), -1);
    }

    #[test]
    fn target_in_gap() {
        assert_eq!(find_first_occurrence(&[1, 3, 5, 7], 4), -1);
    }

    #[test]
    fn empty_array() {
        assert_eq!(find_first_occurrence(&[], 1), -1);
    }

    #[test]
    fn single_element_match() {
        assert_eq!(find_first_occurrence(&[42], 42), 0);
    }

    #[test]
    fn single_element_no_match() {
        assert_eq!(find_first_occurrence(&[42], 1), -1);
    }

    #[test]
    fn all_elements_equal_target() {
        assert_eq!(find_first_occurrence(&[7, 7, 7, 7, 7], 7), 0);
    }

    #[test]
    fn duplicates_at_end() {
        assert_eq!(find_first_occurrence(&[1, 2, 3, 5, 5, 5], 5), 3);
    }
}
