// 459. Repeated Substring Pattern (Easy)
// https://leetcode.com/problems/repeated-substring-pattern/
pub struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut ss: String = s.clone();
        ss.push_str(s.as_str());
        let teil: &str = &ss[1..ss.len() - 1];
        teil.contains(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_ab_twice() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn example_2_aba_is_not_repeated() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn example_3_abc_four_times() {
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }

    #[test]
    fn single_char_is_not_repeated() {
        assert!(!Solution::repeated_substring_pattern("a".to_string()));
    }

    #[test]
    fn prefix_matches_first_then_fails() {
        assert!(Solution::repeated_substring_pattern("aabaab".to_string()));
    }

    #[test]
    fn same_char_repeated() {
        assert!(Solution::repeated_substring_pattern("aaaa".to_string()));
    }

    #[test]
    fn no_period_at_all() {
        assert!(!Solution::repeated_substring_pattern("abac".to_string()));
    }
}
