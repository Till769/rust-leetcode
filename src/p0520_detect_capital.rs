// 520. Detect Capital (Easy)
// https://leetcode.com/problems/detect-capital/
pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word.chars().all(|c| c.is_uppercase()) || !word.chars().skip(1).any(|c| c.is_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_all_upper() {
        assert!(Solution::detect_capital_use("USA".to_string()));
    }

    #[test]
    fn example_2_mixed_is_invalid() {
        assert!(!Solution::detect_capital_use("FlaG".to_string()));
    }

    #[test]
    fn all_lower() {
        assert!(Solution::detect_capital_use("leetcode".to_string()));
    }

    #[test]
    fn only_first_upper() {
        assert!(Solution::detect_capital_use("Google".to_string()));
    }

    #[test]
    fn single_lower_letter() {
        assert!(Solution::detect_capital_use("g".to_string()));
    }

    #[test]
    fn single_upper_letter() {
        assert!(Solution::detect_capital_use("G".to_string()));
    }

    #[test]
    fn lower_first_then_upper_is_invalid() {
        assert!(!Solution::detect_capital_use("gOOGLE".to_string()));
    }

    #[test]
    fn upper_only_at_end_is_invalid() {
        assert!(!Solution::detect_capital_use("leetcodE".to_string()));
    }

    #[test]
    fn two_upper_then_lower_is_invalid() {
        assert!(!Solution::detect_capital_use("GOogle".to_string()));
    }
}
