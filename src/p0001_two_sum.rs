use std::collections::HashMap;

// 1. Two Sum (Easy)
// https://leetcode.com/problems/two-sum/
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let hashmap: HashMap<i32, usize> = nums
            .iter()
            .enumerate()
            .map(|(idx, &wert)| (wert, idx))
            .collect();
        for (idx, val) in nums.iter().enumerate() {
            let y = target - val;
            if let Some(jdx) = hashmap.get(&y) {
                if *jdx != idx {
                    return vec![idx as i32, *jdx as i32];
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reihenfolge der Antwort ist egal -> vor dem Vergleich sortieren
    fn check(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        let mut result = Solution::two_sum(nums, target);
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_1() {
        check(vec![2, 7, 11, 15], 9, vec![0, 1]);
    }

    #[test]
    fn example_2_no_self_match() {
        check(vec![3, 2, 4], 6, vec![1, 2]);
    }

    #[test]
    fn example_3_duplicates() {
        check(vec![3, 3], 6, vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        check(vec![-3, 4, 3, 90], 0, vec![0, 2]);
    }

    #[test]
    fn pair_at_the_end() {
        check(vec![1, 5, 4, 3, 7], 10, vec![3, 4]);
    }

    // TODO: eigene Randfälle ergänzen (Null? target negativ? ...)
}
