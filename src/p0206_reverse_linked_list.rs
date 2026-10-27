// 206. Reverse Linked List (Easy)
// https://leetcode.com/problems/reverse-linked-list/
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rest = head;
        let mut vorherige: Option<Box<ListNode>> = None;
        while let Some(mut aktueller) = rest {
            rest = aktueller.next;
            aktueller.next = vorherige;
            vorherige = Some(aktueller);
        }
        vorherige
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
        to_vec(Solution::reverse_list(from_vec(input)))
    }

    #[test]
    fn example_1() {
        assert_eq!(run(vec![1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(run(vec![1, 2]), vec![2, 1]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(run(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn single_node() {
        assert_eq!(run(vec![7]), vec![7]);
    }

    #[test]
    fn negatives_and_duplicates() {
        assert_eq!(run(vec![-5000, 0, 0, 5000]), vec![5000, 0, 0, -5000]);
    }

    #[test]
    fn max_length() {
        let input: Vec<i32> = (0..5000).collect();
        let expected: Vec<i32> = (0..5000).rev().collect();
        assert_eq!(run(input), expected);
    }
}
