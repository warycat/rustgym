struct Solution;

use std::collections::HashSet;

impl Solution {
    fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut prev: HashSet<i32> = HashSet::new();
        let n = arr.len();
        let mut res = std::i32::MAX;
        for i in 0..n {
            let mut cur = HashSet::new();
            for x in prev {
                cur.insert(x & arr[i]);
            }
            cur.insert(arr[i]);
            for &x in &cur {
                res = res.min((x - target).abs());
            }
            prev = cur;
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![9, 12, 3, 7, 15];
    let target = 5;
    let res = 2;
    assert_eq!(Solution::closest_to_target(arr, target), res);
    let arr = vec![1000000, 1000000, 1000000];
    let target = 1;
    let res = 999999;
    assert_eq!(Solution::closest_to_target(arr, target), res);
    let arr = vec![1, 2, 4, 8, 16];
    let target = 0;
    let res = 0;
    assert_eq!(Solution::closest_to_target(arr, target), res);
}
