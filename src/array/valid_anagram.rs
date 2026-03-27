/// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
///
/// # Examples
///
/// Example 1:
/// - Input: s = "anagram", t = "nagaram"
/// - Output: true
///
/// Example 2:
/// - Input: s = "rat", t = "car"
/// - Output: false
///
/// # Constraints
/// - 1 <= s.length, t.length <= 5 * 10^4
/// - s and t consist of lowercase English letters.
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
///
/// # Follow up
/// What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
pub fn is_anagram(s: String, t: String) -> bool {
    let letters_s = s
        .chars()
        .fold(std::collections::HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
            acc
        });

    let letters_t = t
        .chars()
        .fold(std::collections::HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|count| *count += 1).or_insert(1);
            acc
        });

    letters_s == letters_t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        let cases = vec![
            ("anagram".to_string(), "nagaram".to_string(), true),
            ("rat".to_string(), "car".to_string(), false),
            ("tree".to_string(), "eetr".to_string(), true),
            ("tram".to_string(), "mart".to_string(), true),
            ("hello".to_string(), "world".to_string(), false),
        ];

        for (s, t, expected) in cases {
            let error_msg = format!("Error for inputs: s = {}, t = {}", &s, &t);
            assert_eq!(is_anagram(s, t), expected, "{}", error_msg);
        }
    }
}
