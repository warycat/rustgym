struct Solution;
use std::collections::HashMap;

impl Solution {
    fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for &i in &a {
            for &j in &b {
                *hm.entry(i + j).or_default() += 1;
            }
        }
        let mut res = 0;
        for &i in &c {
            for &j in &d {
                if let Some(v) = hm.get(&(-i - j)) {
                    res += v;
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![1, 2];
    let b = vec![-2, -1];
    let c = vec![-1, 2];
    let d = vec![0, 2];
    let res = 2;
    assert_eq!(Solution::four_sum_count(a, b, c, d), res);
}
