struct Solution;
use std::collections::HashMap;

impl Solution {
    fn partition_labels(s: String) -> Vec<i32> {
        let mut res = vec![];
        let mut last: HashMap<char, usize> = HashMap::new();
        for (i, c) in s.char_indices() {
            last.insert(c, i);
        }
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.char_indices() {
            let j = last[&c];
            if j > end {
                end = j;
            }
            if i == end {
                res.push((end - start + 1) as i32);
                start = last[&c] + 1;
                end = last[&c] + 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let res = vec![9, 7, 8];
    assert_eq!(Solution::partition_labels(s), res);
    let s = "vhaagbqkaq".to_string();
    let res = vec![1, 1, 8];
    assert_eq!(Solution::partition_labels(s), res);
}
