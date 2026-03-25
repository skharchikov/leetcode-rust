use std::collections::HashMap;

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
