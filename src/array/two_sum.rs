use std::collections::HashMap;

/// # [Two Sum](https://leetcode.com/problems/two-sum)
///
/// Given an array of integers `nums` and an integer `target`, return indices of the two numbers
/// such that they add up to `target`.
///
/// You may assume that each input would have exactly one solution, and you may not use the same
/// element twice. You can return the answer in any order.
///
/// ## Examples
///
/// - `nums = [2,7,11,15], target = 9` → `[0,1]`
/// - `nums = [3,2,4], target = 6` → `[1,2]`
/// - `nums = [3,3], target = 6` → `[0,1]`
///
/// ## Constraints
///
/// - `2 <= nums.length <= 10^4`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `-10^9 <= target <= 10^9`
/// - Only one valid answer exists.
///
/// ## Solution Complexity
///
/// - **Time: O(n)** — single pass through the array; each HashMap lookup/insert is O(1) amortized.
/// - **Space: O(n)** — in the worst case, the HashMap stores all n elements before finding the pair.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, &n) in nums.iter().enumerate() {
        if let Some(idx) = map.get(&(target - n)) {
            return vec![*idx as i32, index as i32];
        }
        map.insert(n, index);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for (nums, target, expected) in nums {
            assert_eq!(two_sum(nums, target), expected);
        }
    }
}
