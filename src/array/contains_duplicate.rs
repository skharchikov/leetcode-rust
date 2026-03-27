/// Given an integer array nums, return true if any value appears at least twice
/// in the array, and return false if every element is distinct.
///
/// # Examples
///
/// Example 1:
/// - Input: nums = [1,2,3,1]
/// - Output: true
/// - Explanation: The element 1 occurs at the indices 0 and 3.
///
/// Example 2:
/// - Input: nums = [1,2,3,4]
/// - Output: false
/// - Explanation: All elements are distinct.
///
/// Example 3:
/// - Input: nums = [1,1,1,3,3,4,3,2,4,2]
/// - Output: true
///
/// # Constraints
/// - 1 <= nums.length <= 10^5
/// - -10^9 <= nums[i] <= 10^9
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = std::collections::HashSet::new();

    for i in nums.iter() {
        if seen.contains(i) {
            return true;
        }
        seen.insert(*i);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let cases = vec![
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];

        for (nums, expected) in cases {
            assert_eq!(contains_duplicate(nums), expected);
        }
    }
}
