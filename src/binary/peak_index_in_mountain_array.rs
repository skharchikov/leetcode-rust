/// # [Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array)
///
/// You are given an integer mountain array `arr` of length `n` where the values increase
/// to a peak element and then decrease. Return the index of the peak element.
///
/// ## Examples
///
/// - `arr = [0,1,0]` → `1`
/// - `arr = [0,2,1,0]` → `1`
/// - `arr = [0,10,5,2]` → `1`
///
/// ## Constraints
///
/// - `3 <= arr.length <= 10^5`
/// - `0 <= arr[i] <= 10^6`
/// - `arr` is guaranteed to be a mountain array.
///
/// ## Solution Complexity
///
/// - **Time: O(log n)** — binary search.
/// - **Space: O(1)**.
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] > arr[mid + 1] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    right as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_elements() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    }

    #[test]
    fn example_four_elements() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    }

    #[test]
    fn example_steep_drop() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    }

    #[test]
    fn peak_at_middle() {
        assert_eq!(
            peak_index_in_mountain_array(vec![1, 2, 3, 4, 5, 4, 3, 2, 1]),
            4
        );
    }

    #[test]
    fn peak_near_start() {
        assert_eq!(peak_index_in_mountain_array(vec![1, 5, 4, 3, 2, 1, 0]), 1);
    }

    #[test]
    fn peak_near_end() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 1, 2, 3, 4, 5, 1]), 5);
    }

    #[test]
    fn long_ascent_short_descent() {
        assert_eq!(
            peak_index_in_mountain_array(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5]),
            10
        );
    }

    #[test]
    fn short_ascent_long_descent() {
        assert_eq!(
            peak_index_in_mountain_array(vec![0, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            1
        );
    }

    #[test]
    fn symmetric_mountain() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 1, 2, 3, 2, 1, 0]), 3);
    }

    #[test]
    fn peak_before_last() {
        assert_eq!(peak_index_in_mountain_array(vec![0, 3, 5, 12, 2]), 3);
    }

    #[test]
    fn large_values() {
        assert_eq!(
            peak_index_in_mountain_array(vec![100_000, 999_999, 1_000_000, 500_000, 0]),
            2
        );
    }
}
