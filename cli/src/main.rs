use two_sum::Solution;

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;

    // println!("Hello, world!");
    let indicies = Solution::two_sum(nums, target);

    for x in &indicies {
        println!("{x}");
    }
}
