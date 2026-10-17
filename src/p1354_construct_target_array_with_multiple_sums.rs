use std::collections::BinaryHeap;

// 1354. Construct Target Array With Multiple Sums (Hard)
// https://leetcode.com/problems/construct-target-array-with-multiple-sums/
pub struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let target_i64: Vec<i64> = target.iter().map(|&x| x as i64).collect();
        let mut sum: i64 = target_i64.iter().sum();
        let mut heap: BinaryHeap<i64> = BinaryHeap::from(target_i64);
        loop {
            let max = heap.pop().unwrap();
            let rest = sum - max;
            if max == 1 || rest == 1 {
                return true;
            }
            if rest <= 0 || max < rest {
                return false;
            }
            let neu = max % rest;
            if neu == 0 {
                return false;
            }
            heap.push(neu);
            sum -= max - neu;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(Solution::is_possible(vec![9, 3, 5]));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::is_possible(vec![1, 1, 1, 2]));
    }

    #[test]
    fn example_3() {
        assert!(Solution::is_possible(vec![8, 5]));
    }

    #[test]
    fn rest_sum_one_is_always_possible() {
        assert!(Solution::is_possible(vec![1, 1_000_000_000]));
    }

    #[test]
    fn modulo_zero_with_rest_greater_one() {
        assert!(!Solution::is_possible(vec![2, 4]));
    }

    #[test]
    fn max_not_greater_than_rest() {
        assert!(!Solution::is_possible(vec![3, 3, 3]));
    }

    #[test]
    fn single_element_not_one() {
        assert!(!Solution::is_possible(vec![5]));
    }

    #[test]
    fn single_element_one() {
        assert!(Solution::is_possible(vec![1]));
    }

    #[test]
    fn already_all_ones() {
        assert!(Solution::is_possible(vec![1, 1, 1]));
    }

    #[test]
    fn large_values_sum_overflows_i32() {
        // Summe = 3 * 10^9, passt nicht in i32
        assert!(!Solution::is_possible(vec![1_000_000_000; 3]));
    }
}
