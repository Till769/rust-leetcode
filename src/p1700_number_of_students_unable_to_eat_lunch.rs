pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut student: [i32; 2] = [0, 0];
        for stud in students {
            student[stud as usize] += 1;
        }
        for sand in sandwiches {
            if student[sand as usize] > 0 {
                student[sand as usize] -= 1;
            } else {
                break;
            }
        }
        student.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
