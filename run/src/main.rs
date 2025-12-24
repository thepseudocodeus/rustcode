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

    // Create a linked list
    let mut l1 = to_list(vec![2, 5, 7]);

    while l1.is_some() {
        if let Some(node) = l1 {
            println!("Node Value: {}", node.val);
            l1 = node.next;
        }
    }
}

fn to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut cursor = &mut dummy.next;
    for &num in nums.iter() {
        *cursor = Some(Box::new(ListNode::new(num)));
        cursor = &mut cursor.as_mut().unwrap().next;
    }
    dummy.next
}

fn run_two_sum(numbers: &Vec<i32>, target: i32) -> Vec<i32> {
    Solution::two_sum(numbers.to_vec(), target)
}
