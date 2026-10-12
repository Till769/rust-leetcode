// 2073. Time Needed to Buy Tickets (Easy)
// https://leetcode.com/problems/time-needed-to-buy-tickets/
pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k_ticket: i32 = tickets[k as usize];
        tickets
            .iter()
            .enumerate()
            .map(|(idx, &ticket)| {
                if idx <= k as usize {
                    i32::min(ticket, k_ticket)
                } else {
                    i32::min(k_ticket - 1, ticket)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }

    #[test]
    fn behind_k_with_fewer_tickets() {
        assert_eq!(Solution::time_required_to_buy(vec![3, 1], 0), 4);
    }

    #[test]
    fn single_person() {
        assert_eq!(Solution::time_required_to_buy(vec![7], 0), 7);
    }
}
