// 328. Odd Even Linked List (Medium)
// https://leetcode.com/problems/odd-even-linked-list/
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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rest = head;
        let mut odd_head: Option<Box<ListNode>> = None;
        let mut odd_end: &mut Option<Box<ListNode>> = &mut odd_head;
        let mut even_head: Option<Box<ListNode>> = None;
        let mut even_end: &mut Option<Box<ListNode>> = &mut even_head;
        let mut odd = true;
        while let Some(mut node) = rest {
            rest = node.next.take();
            if odd {
                *odd_end = Some(node);
                odd_end = &mut odd_end.as_mut().unwrap().next;
            } else {
                *even_end = Some(node);
                even_end = &mut even_end.as_mut().unwrap().next;
            }
            odd = !odd
        }
        *odd_end = even_head;
        odd_head
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
        to_vec(Solution::odd_even_list(from_vec(input)))
    }

    #[test]
    fn example_1() {
        assert_eq!(run(vec![1, 2, 3, 4, 5]), vec![1, 3, 5, 2, 4]);
    }

    #[test]
    fn example_2() {
        assert_eq!(run(vec![2, 1, 3, 5, 6, 4, 7]), vec![2, 3, 6, 7, 1, 5, 4]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(run(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn single_node() {
        assert_eq!(run(vec![1]), vec![1]);
    }

    #[test]
    fn two_nodes() {
        assert_eq!(run(vec![1, 2]), vec![1, 2]);
    }

    #[test]
    fn even_length() {
        assert_eq!(run(vec![1, 2, 3, 4, 5, 6]), vec![1, 3, 5, 2, 4, 6]);
    }

    #[test]
    fn negatives() {
        assert_eq!(run(vec![-1, -2, -3]), vec![-1, -3, -2]);
    }
}
