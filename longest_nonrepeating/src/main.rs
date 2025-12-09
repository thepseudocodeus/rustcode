struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0 as i32;
        let mut store = Vec::new();

        for c in s.chars() {
            // println!("char: {}", c);
            // println!("current store: {}", store.iter().collect::<String>());
            // println!("current max length: {}", max_length);
            while store.contains(&c) {
                store.remove(0);
            }
            store.push(c);
            // println!("new store: {}", store.iter().collect::<String>());
            max_length = max_length.max(store.len() as i32);
            // println!("new max length: {}", max_length);
        }
        max_length
    }
}

// #[rstest]
// #[case(String::from("abcabcbb"), 3)]
// #[case(String::from("bbbbb"), 1)]
// #[case(String::from("pwwkew"), 3)]
// #[case(String::from(""), 0)]
// #[case(String::from(" "), 1)]
// #[case(String::from("au"), 2)]
// #[case(String::from("dvdf"), 3)]
// fn test_length_of_longest_substring(#[case] s: String, #[case] expected: i32) {
//     let result = Solution::length_of_longest_substring(s);
//     assert_eq!(result, expected);
// }

fn main() {
    let s = String::from("abcabcbb");
    println!("Input: {}", s);
    let result = Solution::length_of_longest_substring(s);
    println!("Output: {}", result);
}
