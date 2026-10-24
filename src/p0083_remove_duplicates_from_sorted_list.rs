// 83. Remove Duplicates from Sorted List (Easy)
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
pub struct Solution;

// Definition for singly-linked list (wie von LeetCode vorgegeben).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut now) = head.as_mut() {
            while now.next.is_some() {
                if now.val == now.next.as_ref().unwrap().val {
                    now.next = now.next.as_mut().unwrap().next.take();
                } else {
                    now = now.next.as_mut().unwrap();
                }
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hilfsfunktionen nur für die Tests: Vec <-> verkettete Liste
    fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in values.iter().rev() {
            let mut node = Box::new(ListNode::new(v));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(node) = list {
            out.push(node.val);
            list = node.next;
        }
        out
    }

    fn run(input: Vec<i32>) -> Vec<i32> {
        to_vec(Solution::delete_duplicates(from_vec(input)))
    }

    #[test]
    fn example_1() {
        assert_eq!(run(vec![1, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(run(vec![1, 1, 2, 3, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(run(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn single_node() {
        assert_eq!(run(vec![5]), vec![5]);
    }

    #[test]
    fn triple_duplicate() {
        assert_eq!(run(vec![1, 1, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn all_equal() {
        assert_eq!(run(vec![7, 7, 7, 7]), vec![7]);
    }

    #[test]
    fn no_duplicates_with_negatives() {
        assert_eq!(run(vec![-100, -3, 0, 100]), vec![-100, -3, 0, 100]);
    }
}
