
use two_sum::Solution;

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
    };
}

fn run_two_sum(numbers: &Vec<i32>, target: i32) -> Vec<i32> {
    Solution::two_sum(numbers.to_vec(), target)
}
