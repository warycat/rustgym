struct Solution;

use std::collections::HashMap;

impl Solution {
    fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut hs: HashMap<i32, Vec<i32>> = HashMap::new();
        let n = a.len();
        for (i, &x) in b.iter().enumerate() {
            let indexes = hs.entry(x).or_default();
            indexes.push(i as i32);
        }
        let mut res: Vec<i32> = vec![0; n];
        for (i, &x) in a.iter().enumerate() {
            let indexes = hs.entry(x).or_default();
            res[i] = indexes.pop().unwrap();
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![12, 28, 46, 32, 50];
    let b = vec![50, 12, 32, 46, 28];
    let res = vec![1, 4, 3, 2, 0];
    assert_eq!(Solution::anagram_mappings(a, b), res);
}
