/*
   #1: Two Sum

   Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

   You may assume that each input would have exactly one solution, and you may not use the same element twice.

   You can return the answer in any order.



   Example 1:

   Input: nums = [2,7,11,15], target = 9
   Output: [0,1]
   Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
   Example 2:

   Input: nums = [3,2,4], target = 6
   Output: [1,2]
   Example 3:

   Input: nums = [3,3], target = 6
   Output: [0,1]


   Constraints:

   2 <= nums.length <= 104
   -109 <= nums[i] <= 109
   -109 <= target <= 109
   Only one valid answer exists.


   Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
*/
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut history = HashMap::with_capacity(numbers.len());

        for (i, &number) in numbers.iter().enumerate() {
            // println!("index: {} = value: {}", i, num);
            let complement = target - number;

            // If possible solve problem now
            if let Some(&prior_index) = history.get(&complement) {
                return vec![prior_index as i32, i as i32];
            }

            // Add current number to history
            history.insert(number, i);
        }

        // There is a chance there is not a solution. Deal with this explicitly.
        panic!("No solution found in {:?} for {}.", numbers, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2, 7, 11, 15], 9, vec![0, 1])]
    #[case(vec![3, 2, 4], 6, vec![1, 2])]
    fn test_two_sum(#[case] numbers: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let mut result = Solution::two_sum(numbers, target);
        let mut expected = expected;
        // Ensure both are sorted so they match.
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
