use add_two_numbers::to_list;
use two_sum::Solution;

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

fn main() {
    // Input variables
    let input_vec = vec![2, 7, 11, 15];
    let target = 13;
    // Get solution
    let result = run_two_sum(&input_vec, target);
    // Display the results
    for &index in &result {
        let idx = index as usize;
        if let Some(val) = input_vec.get(idx) {
            println!("Index: {} = Value: {}", idx, val);
        }
    }

    // Create linked lists
    let mut l1 = to_list(vec![2, 4, 3]);
    let mut l2 = to_list(vec![5, 6, 4]);
    let mut dummy = ListNode::new(0);
    let mut cursor = &mut dummy.next;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        // Add l1 value
        // Advance to l1.next
        if let Some(node) = l1 {
            println!("Current Value: {}", node.val);
            sum += node.val;
            println!("Current Sum: {}", sum);
            l1 = node.next;
        }

        // Add l2 value
        // Advance to l2.next
        if let Some(node) = l2 {
            println!("Current Value: {}", node.val);
            sum += node.val;
            println!("Current Sum: {}", sum);
            l2 = node.next;
        }

        // Update carry
        carry = sum / 10;
        println!("Current Carry: {}", carry);

        // [] TODO: updated current cursor
        *cursor = Some(Box::new(ListNode::new(sum % 10)));
        // Move reference to next
        cursor = &mut cursor.as_mut().unwrap().next;
    }

    // for &node in dummy.next.as_ref().iter() {
    //     println!("Current Value: {}", node.val);
    // }

    // show_llist(dummy.next);
    let result = to_vec(dummy.next);
    println!("Result: {:?}", result);
}

fn show_llist(head: Option<Box<ListNode>>) {
    println!("Printing linked list");
    let mut cursor = &head;
    while let Some(node) = cursor {
        println!("Current Value: {}", node.val);
        cursor = &node.next;
    }
}

fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut cursor = &head;

    while let Some(node) = cursor {
        result.push(node.val);
        cursor = &node.next;
    }

    result

}

// fn to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
//     let mut dummy = ListNode::new(0);
//     let mut cursor = &mut dummy.next;
//     for &num in nums.iter() {
//         *cursor = Some(Box::new(ListNode::new(num)));
//         cursor = &mut cursor.as_mut().unwrap().next;
//     }
//     dummy.next
// }

// fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) {}

fn run_two_sum(numbers: &Vec<i32>, target: i32) -> Vec<i32> {
    Solution::two_sum(numbers.to_vec(), target)
}
