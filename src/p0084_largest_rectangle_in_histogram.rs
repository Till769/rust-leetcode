pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::with_capacity(heights.len());
        let mut largest: i32 = 0;
        for (idx, size) in heights
            .iter()
            .copied()
            .chain(std::iter::once(0))
            .enumerate()
        {
            while let Some(&last) = stack.last()
                && size <= heights[last]
            {
                stack.pop();
                let rectangle = match stack.last() {
                    Some(&left) => (idx - left - 1) as i32 * heights[last],
                    None => idx as i32 * heights[last],
                };
                largest = largest.max(rectangle);
            }
            stack.push(idx);
        }

        largest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn single_zero() {
        assert_eq!(Solution::largest_rectangle_area(vec![0]), 0);
    }

    #[test]
    fn single_bar() {
        assert_eq!(Solution::largest_rectangle_area(vec![7]), 7);
    }

    #[test]
    fn all_equal() {
        assert_eq!(Solution::largest_rectangle_area(vec![3, 3, 3]), 9);
    }

    #[test]
    fn strictly_increasing() {
        // Stack wird nie geleert -> alles passiert im Aufräumen am Ende
        assert_eq!(Solution::largest_rectangle_area(vec![1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(Solution::largest_rectangle_area(vec![5, 4, 3, 2, 1]), 9);
    }

    #[test]
    fn zero_in_the_middle() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 0, 2]), 2);
    }

    #[test]
    fn lowest_bar_spans_everything() {
        // linke Grenze -1 und rechte Grenze len() werden gebraucht
        assert_eq!(Solution::largest_rectangle_area(vec![4, 2, 4, 4]), 8);
    }
}
