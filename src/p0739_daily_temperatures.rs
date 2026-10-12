pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::with_capacity(temperatures.len());
        let mut answer: Vec<i32> = vec![0; temperatures.len()];
        for (idx, &temp) in temperatures.iter().enumerate() {
            while let Some(&last) = stack.last()
                && temp > temperatures[last]
            {
                answer[last] = (idx - last) as i32;
                stack.pop();
            }
            stack.push(idx);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
