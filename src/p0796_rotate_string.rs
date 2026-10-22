// 796. Rotate String (Easy)
// https://leetcode.com/problems/rotate-string/
pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let ss = s.repeat(2);
        ss.contains(goal.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_rotation() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
    }

    #[test]
    fn example_2_no_rotation() {
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
    }

    #[test]
    fn substring_but_different_length() {
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "cde".to_string()
        ));
    }

    // TODO: eigene Randfälle ergänzen (gleiche Strings? ein Zeichen? ...)
}
