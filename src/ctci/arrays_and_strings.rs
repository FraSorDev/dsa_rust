/* =============================== Arrays and strings ===============================
Assumptions:
Assume all input string use standard ASCII characters.
If non-ASCII characters are used the functions with string input will return false.
*/

/* Function that returns true if a string has all unique characters and false if not. If an empty string
is given, the function returns true.
Time: O(N), Space: O(1)
*/
pub fn is_unique(string: &str) -> bool {
    if string.len() > 128 || !string.is_ascii() {
        return false;
    }
    let mut bit_vector = 0u128;
    for &byte in string.as_bytes() {
        let mask = 1u128 << byte;
        if bit_vector & mask != 0 {
            return false;
        }
        bit_vector |= mask;
    }
    true
}

/* Function that returns true if string s1 is permutation of s2 and false if not.
If both strings are empty, the function returns true.
Time: O(N), Space: O(1)
*/
pub fn check_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() || !s1.is_ascii() || !s2.is_ascii() {
        return false;
    }
    let mut counts = [0usize; 128];
    for &byte in s1.as_bytes() {
        counts[byte as usize] += 1;
    }
    for &byte in s2.as_bytes() {
        let index = byte as usize;
        if counts[index] == 0 {
            return false;
        }
        counts[index] -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        let test_cases = [
            ("", true),
            ("hello", false),
            ("world", true),
            ("s4fad", true),
            ("hb 627jh=j ()", false),
            ("aA", true),
            ("🚀💡", false),
            ("🚀🚀", false),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                is_unique(input),
                expected,
                "Test failed for input string: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_check_permutation() {
        let test_cases = [
            ("", "", true),
            ("abc", "bca", true),
            ("abc", "abcd", false),
            ("abc", "abd", false),
            ("aabb", "bbaa", true),
            ("hello", "olleh", true),
            ("hello", "ollhe!", false),
            ("aA", "Aa", true),
            ("🚀💡", "💡🚀", false),
            ("🚀🚀", "🚀", false),
        ];

        for (s1, s2, expected) in test_cases {
            assert_eq!(
                check_permutation(s1, s2),
                expected,
                "Failed for ('{}', '{}')",
                s1,
                s2
            );
        }
    }
}
