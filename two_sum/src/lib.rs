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

    #[rstest]
    #[case(1, 3, 4)]
    #[case(7, 3, 10)]
    #[case(17, 21, 38)]
    fn it_works(#[case] a:u64, #[case] b: u64, #[case] expected: u64) {
        let result = TwoSum::add(a, b);
        assert_eq!(result, expected);
    }
}
