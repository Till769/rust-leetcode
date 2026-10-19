// 482. License Key Formatting (Easy)
// https://leetcode.com/problems/license-key-formatting/
pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let string2: String = s
            .chars()
            .filter(|&x| x != '-')
            .map(|x| x.to_ascii_uppercase())
            .collect();
        let mut ans: String = String::new();
        let k = k as usize;

        let rest = string2.len() % k;

        for (i, x) in string2.char_indices() {
            if i % k == rest && i > 0 {
                ans.push('-');
            }
            ans.push(x);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, k: i32) -> String {
        Solution::license_key_formatting(s.to_string(), k)
    }

    #[test]
    fn example_1() {
        assert_eq!(run("5F3Z-2e-9-w", 4), "5F3Z-2E9W");
    }

    #[test]
    fn example_2() {
        assert_eq!(run("2-5g-3-J", 2), "2-5G-3J");
    }

    #[test]
    fn only_dashes() {
        assert_eq!(run("---", 3), "");
    }

    #[test]
    fn k_is_one() {
        assert_eq!(run("a-b-c", 1), "A-B-C");
    }

    #[test]
    fn leading_and_trailing_dashes() {
        assert_eq!(run("-ab-cd-", 2), "AB-CD");
    }

    #[test]
    fn k_larger_than_key() {
        assert_eq!(run("a-b", 10), "AB");
    }
}
