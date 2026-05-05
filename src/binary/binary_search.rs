use std::cmp::Ordering;

/// Recursive binary search over a sorted slice.
///
/// Returns the index of `target` in `items`, or `None` if absent.
/// If `items` contains duplicates of `target`, the returned index
/// may be any matching position.
///
/// `items` must be sorted in ascending order; otherwise the result
/// is unspecified.
///
/// Runs in `O(log n)` time, `O(log n)` stack space.
///
/// # Examples
///
/// ```
/// use leetcode::binary::binary_search::binary_search_recursive;
///
/// assert_eq!(binary_search_recursive(&[1, 3, 5, 7, 9], &5), Some(2));
/// assert_eq!(binary_search_recursive(&[1, 3, 5, 7, 9], &4), None);
/// ```
pub fn binary_search_recursive<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    binary_search_recursive_impl(items, 0, items.len(), target)
}

fn binary_search_recursive_impl<T: Ord>(
    items: &[T],
    lo: usize,
    hi: usize,
    target: &T,
) -> Option<usize> {
    if lo >= hi {
        return None;
    }

    let mid = lo + (hi - lo) / 2;

    match target.cmp(&items[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search_recursive_impl(items, lo, mid, target),
        Ordering::Greater => binary_search_recursive_impl(items, mid + 1, hi, target),
    }
}

/// Iterative binary search over a sorted slice.
///
/// Returns the index of `target` in `items`, or `None` if absent.
/// If `items` contains duplicates of `target`, the returned index
/// may be any matching position.
///
/// `items` must be sorted in ascending order; otherwise the result
/// is unspecified.
///
/// Runs in `O(log n)` time, `O(1)` space.
///
/// # Examples
///
/// ```
/// use leetcode::binary::binary_search::binary_search_iterative;
///
/// assert_eq!(binary_search_iterative(&[1, 3, 5, 7, 9], &5), Some(2));
/// assert_eq!(binary_search_iterative(&[1, 3, 5, 7, 9], &4), None);
/// ```
pub fn binary_search_iterative<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = items.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match target.cmp(&items[mid]) {
            Ordering::Equal => {
                return Some(mid);
            }
            Ordering::Less => {
                right = mid;
            }
            Ordering::Greater => {
                left = mid + 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_both<T: Ord>(items: &[T], target: &T, expected: Option<usize>) {
        assert_eq!(
            binary_search_recursive(items, target),
            expected,
            "recursive"
        );
        assert_eq!(
            binary_search_iterative(items, target),
            expected,
            "iterative"
        );
    }

    fn assert_both_in<T: Ord>(items: &[T], target: &T, allowed: &[usize]) {
        let r = binary_search_recursive(items, target);
        let i = binary_search_iterative(items, target);
        let r_idx = r.expect("recursive returned None");
        let i_idx = i.expect("iterative returned None");
        assert!(
            allowed.contains(&r_idx),
            "recursive idx {} not in {:?}",
            r_idx,
            allowed
        );
        assert!(
            allowed.contains(&i_idx),
            "iterative idx {} not in {:?}",
            i_idx,
            allowed
        );
    }

    #[test]
    fn finds_target_in_middle() {
        assert_both(&[1, 3, 5, 7, 9], &5, Some(2));
    }

    #[test]
    fn finds_first_element() {
        assert_both(&[1, 3, 5, 7, 9], &1, Some(0));
    }

    #[test]
    fn finds_last_element() {
        assert_both(&[1, 3, 5, 7, 9], &9, Some(4));
    }

    #[test]
    fn target_smaller_than_all() {
        assert_both(&[1, 3, 5, 7, 9], &0, None);
    }

    #[test]
    fn target_larger_than_all() {
        assert_both(&[1, 3, 5, 7, 9], &10, None);
    }

    #[test]
    fn target_between_elements() {
        assert_both(&[1, 3, 5, 7, 9], &4, None);
    }

    #[test]
    fn empty_slice() {
        assert_both::<i32>(&[], &1, None);
    }

    #[test]
    fn single_element_match() {
        assert_both(&[42], &42, Some(0));
    }

    #[test]
    fn single_element_no_match_smaller() {
        assert_both(&[42], &1, None);
    }

    #[test]
    fn single_element_no_match_larger() {
        assert_both(&[42], &100, None);
    }

    #[test]
    fn two_elements() {
        assert_both(&[1, 2], &1, Some(0));
        assert_both(&[1, 2], &2, Some(1));
        assert_both(&[1, 2], &0, None);
        assert_both(&[1, 2], &3, None);
    }

    #[test]
    fn even_length_slice() {
        assert_both(&[1, 3, 5, 7], &3, Some(1));
        assert_both(&[1, 3, 5, 7], &5, Some(2));
    }

    #[test]
    fn duplicates_returns_some_match() {
        assert_both_in(&[1, 2, 2, 2, 3], &2, &[1, 2, 3]);
    }

    #[test]
    fn works_with_strings() {
        let items = ["apple", "banana", "cherry", "date"];
        assert_both(&items, &"cherry", Some(2));
        assert_both(&items, &"fig", None);
    }
}
