# TWO SUM

## GIST
Find a specific sum in an unordered list

## GOAL
We are looking for a way to find two numbers from a given list that, when added together, exactly equal a specific number.

## Model

*Goal:*
Solve problem with < O(n^2) time complexity

  - Intuition:
    - Perform all necessary operations in a single pass of the list

*Givens:*
*Assumptions:*
*Input*
*Output*
*Constraints & Conditions*

## Pseudocode

```text
// Solution: TwoSum O(n)

1.  FUNCTION TwoSum(Array A, Target T):
2.      INITIALIZE HashMap Seen
3.
4.      FOR i FROM 0 TO LENGTH(A) - 1 DO:
5.          current_element ← A[i]
6.          complement      ← T - current_element
7.
8.          // Have seen? O(1) [Note: Average; Worst Case can be O(n) if all keys collide which is statistically unlikely]
9.          IF complement IN Seen THEN
10.             RETURN [i, Seen[complement]]
11.         END IF
12.
13.         // Store current element for future lookups
14.         Seen[current_element] ← i
15.     END FOR
16.
17.     RETURN NULL
```




## Original
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
