use std::collections::BinaryHeap;

// 1046. Last Stone Weight (Easy)
// https://leetcode.com/problems/last-stone-weight/
pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
        while heap.len() > 1 {
            let val1 = heap.pop().unwrap();
            let val2 = heap.pop().unwrap();
            if val1 != val2 {
                heap.push(val1 - val2);
            }
        }
        heap.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn equal_stones_leave_nothing() {
        assert_eq!(Solution::last_stone_weight(vec![3, 3]), 0);
    }
}
