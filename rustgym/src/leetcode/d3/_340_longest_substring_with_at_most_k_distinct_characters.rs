struct Solution;

use std::collections::HashMap;

impl Solution {
    fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();
        let mut start = 0;
        let mut end = 0;
        let s: Vec<char> = s.chars().collect();
        let mut count: HashMap<char, usize> = HashMap::new();
        let mut res = 0;
        while end < n {
            *count.entry(s[end]).or_default() += 1;
            end += 1;
            while count
                .values()
                .fold(0, |acc, &v| acc + if v > 0 { 1 } else { 0 })
                > k
            {
                *count.get_mut(&s[start]).unwrap() -= 1;
                start += 1;
            }
            res = res.max(end - start);
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "eceba".to_string();
    let k = 2;
    let res = 3;
    assert_eq!(Solution::length_of_longest_substring_k_distinct(s, k), res);
    let s = "aa".to_string();
    let k = 1;
    let res = 2;
    assert_eq!(Solution::length_of_longest_substring_k_distinct(s, k), res);
}
