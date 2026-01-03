/*
#2. Add Two Numbers

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.



Example 1:


Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.
Example 2:

Input: l1 = [0], l2 = [0]
Output: [0]
Example 3:

Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]


Constraints:

The number of nodes in each linked list is in the range [1, 100].
0 <= Node.val <= 9
It is guaranteed that the list represents a number that does not have leading zeros.
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Issue: had to transform to mutable for my algorithm
        let mut l1 = l1;
        let mut l2 = l2;

        // Use the dummy pattern
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy.next;
        let mut carry = 0;

        println!("dummy: {}", dummy.val);
        // IF list1 has nodes OR list2 has nodes OR carry NOT 0 LOOP
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            // START LIST1
            // ADD node value
            if let Some(node) = l1 {
                sum += node.val;
                // NEXT l1 node
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                // NEXT l2 node
                l2 = node.next;
            }

            carry = sum / 10;
            let new_value = sum % 10;

            *current = Some(Box::new(ListNode::new(new_value)));
            current = &mut current.as_mut().unwrap().next;

        }

        // Return dummy after head
        dummy.next
    }
}

pub fn to_list(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut cursor = &mut dummy.next;
    for &number in numbers.iter() {
        *cursor = Some(Box::new(ListNode::new(number)));
        cursor = &mut cursor.as_mut().unwrap().next;
    }
    dummy.next
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list() -> Option<Box<ListNode>> {
        None
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
