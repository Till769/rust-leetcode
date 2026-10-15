// 373. Find K Pairs with Smallest Sums (Medium)
// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );
    }

    #[test]
    fn nums1_has_only_one_element() {
        // Randfall: i darf nicht über das Ende von nums1 hinauslaufen
        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 3),
            vec![vec![1, 1], vec![1, 2], vec![1, 3]]
        );
    }

    #[test]
    fn k_equals_all_pairs() {
        // Randfall: k = nums1.len() * nums2.len(), alle Paare werden gebraucht
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 2),
            vec![vec![1, 3], vec![2, 3]]
        );
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![-10, -4, 0], vec![3, 5], 4),
            vec![vec![-10, 3], vec![-10, 5], vec![-4, 3], vec![-4, 5]]
        );
    }
}
