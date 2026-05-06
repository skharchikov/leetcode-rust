/// # [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array)
///
/// Suppose an array of length `n` sorted in ascending order is rotated between `1` and `n`
/// times. Given the sorted rotated array `nums` of unique elements, return the minimum element.
///
/// ## Examples
///
/// - `nums = [3,4,5,1,2]` → `1`
/// - `nums = [4,5,6,7,0,1,2]` → `0`
/// - `nums = [11,13,15,17]` → `11`
///
/// ## Constraints
///
/// - `n == nums.length`
/// - `1 <= n <= 5000`
/// - `-5000 <= nums[i] <= 5000`
/// - All integers of `nums` are unique.
/// - `nums` is sorted and rotated between `1` and `n` times.
///
/// ## Solution Complexity
///
/// - **Time: O(log n)** — binary search.
/// - **Space: O(1)**.
pub fn find_min(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len();
    let last = arr[arr.len() - 1];

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= last {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    arr[left]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_rotated_4() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn example_not_rotated() {
        assert_eq!(find_min(vec![0, 1, 2, 4, 5, 6, 7]), 0);
    }

    #[test]
    fn example_rotated_3() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn single_element() {
        assert_eq!(find_min(vec![1]), 1);
    }

    #[test]
    fn two_elements_rotated() {
        assert_eq!(find_min(vec![2, 1]), 1);
    }

    #[test]
    fn two_elements_sorted() {
        assert_eq!(find_min(vec![1, 2]), 1);
    }

    #[test]
    fn rotated_by_one() {
        assert_eq!(find_min(vec![7, 1, 2, 3, 4, 5, 6]), 1);
    }

    #[test]
    fn rotated_by_n_minus_one() {
        assert_eq!(find_min(vec![2, 3, 4, 5, 6, 7, 1]), 1);
    }

    #[test]
    fn negative_values() {
        assert_eq!(find_min(vec![-1, 0, 1, -3, -2]), -3);
    }

    #[test]
    fn all_negative_rotated() {
        assert_eq!(find_min(vec![-2, -1, -5, -4, -3]), -5);
    }

    #[test]
    fn min_at_middle() {
        assert_eq!(find_min(vec![5, 6, 7, 1, 2, 3, 4]), 1);
    }
}
