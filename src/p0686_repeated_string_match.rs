// 686. Repeated String Match (Medium)
// https://leetcode.com/problems/repeated-string-match/
pub struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let n = b.len().div_ceil(a.len());
        let mut aa = a.repeat(n);
        if aa.contains(&b) {
            return n as i32;
        }
        aa.push_str(a.as_str());
        if aa.contains(&b) {
            return n as i32 + 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::repeated_string_match("a".to_string(), "aa".to_string()),
            2
        );
    }

    #[test]
    fn b_starts_mid_a_needs_extra_copy() {
        assert_eq!(
            Solution::repeated_string_match("abc".to_string(), "cabca".to_string()),
            3
        );
    }

    #[test]
    fn impossible() {
        assert_eq!(
            Solution::repeated_string_match("abc".to_string(), "abd".to_string()),
            -1
        );
    }

    // TODO: eigene Randfälle ergänzen (b genau Vielfaches von a? b kürzer als a? ...)
}
