/// # Newspapers
///
/// You have a stack of newspapers in a fixed order. Each newspaper has a read time.
/// Assign all newspapers to a group of at most `num_coworkers` workers. Each worker is
/// assigned a consecutive section of newspapers from the stack, and all workers read
/// their assigned sections in parallel.
///
/// You cannot reorder newspapers — each worker gets a consecutive block from the
/// original stack. Return the minimum time needed to read all newspapers (the time
/// taken by the slowest worker).
///
/// ## Examples
///
/// - `newspapers = [7, 2, 5, 10, 8]`, `num_coworkers = 2` → `18`
///   (split into `[7, 2, 5]` (14) and `[10, 8]` (18))
/// - `newspapers = [2, 3, 5, 7]`, `num_coworkers = 3` → `7`
///   (split into `[2, 3]`, `[5]`, `[7]`)
///
/// ## Constraints
///
/// - `1 <= newspapers_read_times.length <= 10^5`
/// - `1 <= newspapers_read_times[i] <= 10^5`
/// - `1 <= num_coworkers <= 10^5`
///
/// ## Solution Complexity
///
/// - **Time: O(n log S)** where `S` is the sum of read times — binary search on the answer.
/// - **Space: O(1)**.
pub fn newspapers(newspapers_read_times: Vec<i32>, num_coworkers: i32) -> i32 {
    let mut left = *newspapers_read_times.iter().max().unwrap_or(&0);
    let mut right = newspapers_read_times.iter().sum::<i32>();

    while left < right {
        let mid = left + (right - left) / 2;

        if feasible(&newspapers_read_times[..], num_coworkers, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn feasible(read_times: &[i32], num_coworkers: i32, limit: i32) -> bool {
    let (mut time, mut workers) = (0, 0);

    for read_time in read_times.iter() {
        if time + read_time > limit {
            workers += 1;
            time = 0;
        }

        time += read_time;
    }

    if time != 0 {
        workers += 1;
    }

    workers <= num_coworkers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_workers() {
        assert_eq!(newspapers(vec![7, 2, 5, 10, 8], 2), 18);
    }

    #[test]
    fn example_three_workers() {
        assert_eq!(newspapers(vec![2, 3, 5, 7], 3), 7);
    }

    #[test]
    fn single_worker() {
        assert_eq!(newspapers(vec![1, 2, 3, 4, 5], 1), 15);
    }

    #[test]
    fn workers_equal_to_newspapers() {
        assert_eq!(newspapers(vec![1, 2, 3, 4, 5], 5), 5);
    }

    #[test]
    fn more_workers_than_newspapers() {
        assert_eq!(newspapers(vec![3, 7, 2], 10), 7);
    }

    #[test]
    fn single_newspaper() {
        assert_eq!(newspapers(vec![42], 1), 42);
    }

    #[test]
    fn single_newspaper_many_workers() {
        assert_eq!(newspapers(vec![42], 5), 42);
    }

    #[test]
    fn all_equal() {
        assert_eq!(newspapers(vec![5, 5, 5, 5], 2), 10);
    }

    #[test]
    fn all_equal_four_workers() {
        assert_eq!(newspapers(vec![5, 5, 5, 5], 4), 5);
    }

    #[test]
    fn max_dominates() {
        assert_eq!(newspapers(vec![1, 1, 1, 100, 1, 1, 1], 3), 100);
    }

    #[test]
    fn two_newspapers_two_workers() {
        assert_eq!(newspapers(vec![10, 20], 2), 20);
    }

    #[test]
    fn two_newspapers_one_worker() {
        assert_eq!(newspapers(vec![10, 20], 1), 30);
    }

    #[test]
    fn split_balances() {
        assert_eq!(newspapers(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    }
}
