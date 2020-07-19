struct Solution;
use std::collections::HashMap;

impl Solution {
    fn count_triplets(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                *hm.entry(a[i] & a[j]).or_default() += 1;
            }
        }
        let mut res = 0;
        for i in 0..n {
            for (&k, &v) in hm.iter() {
                if a[i] & k == 0 {
                    res += v;
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![2, 1, 3];
    let res = 12;
    assert_eq!(Solution::count_triplets(a), res);
}
