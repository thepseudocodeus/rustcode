use rstest::rstest;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Note: We were calculating this multiple times so did it once upfront.
        let num_len = nums.len();
        // Lesson: Rust uses usize by default for indexes
        let mut history = HashMap::new();
        let mut output = Vec::new();
        output.push(0 as i32);

        for (i, &num) in nums.iter().enumerate() {
            // println!("index: {} = value: {}", i, num);
            let complement = target - num;

            if let Some(&j) = history.get(&complement) {
                return vec![j as i32, i as i32]
            }

            history.insert(num, i);
        }

        // There is a chance there is not a solution. Deal with this explicitly.
        unreachable!("No solution found.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_two_sum() -> Vec<i32> {

    }

    #[rstest]
    #[case([2, 7, 11, 15], 9, [0, 1])]
    fn it_works(#[case] a:Vec<i32>, #[case] b: Vec<i32>, #[case] expected: u64) {
        let result = Solution::two_sum(a, b);
        assert_eq!(result, expected);
    }
}
