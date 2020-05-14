struct Solution;
use std::collections::HashMap;

impl Solution {
    fn custom_sort_string(s: String, t: String) -> String {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.char_indices() {
            *hm.entry(c).or_default() = i;
        }
        let mut t: Vec<char> = t.chars().collect();
        t.sort_unstable_by_key(|c| hm.get(c).unwrap_or(&26));
        t.iter().collect()
    }
}

#[test]
fn test() {
    let s = "cba".to_string();
    let t = "abcd".to_string();
    let res = "cbad".to_string();
    assert_eq!(Solution::custom_sort_string(s, t), res);
}
