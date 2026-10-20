// 831. Masking Personal Information (Medium)
// https://leetcode.com/problems/masking-personal-information/
pub struct Solution;

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.starts_with(|c: char| c.is_ascii_alphabetic()) {
            let email: String = s.to_ascii_lowercase();
            let (name, domain) = email.split_once('@').unwrap();
            let first_name = name.chars().next().unwrap();
            let last_name = name.chars().next_back().unwrap();
            format!("{first_name}*****{last_name}@{domain}")
        } else {
            let mut numbers: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let country_code = numbers.len() - 10;
            let country_mask = match country_code {
                0 => "",
                1 => "+*-",
                2 => "+**-",
                3 => "+***-",
                _ => unreachable!(),
            };
            let number_last = numbers.split_off(numbers.len() - 4);
            format!("{country_mask}***-***-{number_last}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mask(s: &str) -> String {
        Solution::mask_pii(s.to_string())
    }

    // --- E-Mail ---

    #[test]
    fn example_1_email_mixed_case() {
        assert_eq!(mask("LeetCode@LeetCode.com"), "l*****e@leetcode.com");
    }

    #[test]
    fn example_2_email_two_letter_name() {
        assert_eq!(mask("AB@qq.com"), "a*****b@qq.com");
    }

    #[test]
    fn email_all_lowercase() {
        assert_eq!(mask("alice@example.org"), "a*****e@example.org");
    }

    #[test]
    fn email_all_uppercase() {
        assert_eq!(mask("JOHNDOE@MAIL.DE"), "j*****e@mail.de");
    }

    // --- Telefon ---

    #[test]
    fn example_3_phone_no_country_code() {
        assert_eq!(mask("1(234)567-890"), "***-***-7890");
    }

    #[test]
    fn phone_starts_with_parenthesis() {
        assert_eq!(mask("(12) 345-678-90"), "***-***-7890");
    }

    #[test]
    fn phone_country_code_1() {
        assert_eq!(mask("+1 (234) 567-8901"), "+*-***-***-8901");
    }

    #[test]
    fn phone_country_code_2() {
        assert_eq!(mask("+(12) 345-678-9012"), "+**-***-***-9012");
    }

    #[test]
    fn phone_country_code_3() {
        assert_eq!(mask("+86-(10)123456789"), "+***-***-***-6789");
    }

    #[test]
    fn phone_digits_only() {
        assert_eq!(mask("1234567890"), "***-***-7890");
    }

    #[test]
    fn phone_many_separators() {
        assert_eq!(mask("( 1 )-2 3 4-567-8 90"), "***-***-7890");
    }
}
