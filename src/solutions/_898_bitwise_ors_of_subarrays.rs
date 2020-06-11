struct Solution;
use std::collections::HashSet;

impl Solution {
    fn subarray_bitwise_o_rs(a: Vec<i32>) -> i32 {
        let mut res: HashSet<i32> = HashSet::new();
        let mut prev: HashSet<i32> = HashSet::new();
        for x in a {
            let mut cur: HashSet<i32> = HashSet::new();
            cur.insert(x);
            for y in prev {
                cur.insert(y | x);
            }
            for &x in &cur {
                res.insert(x);
            }
            prev = cur;
        }
        res.len() as i32
    }
}

#[test]
fn test() {
    let a = vec![0];
    let res = 1;
    assert_eq!(Solution::subarray_bitwise_o_rs(a), res);
    let a = vec![1, 1, 2];
    let res = 3;
    assert_eq!(Solution::subarray_bitwise_o_rs(a), res);
    let a = vec![1, 2, 4];
    let res = 6;
    assert_eq!(Solution::subarray_bitwise_o_rs(a), res);
}
