struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let mut total = 0;
        for i in 0..n {
            total += nums[i];
            total %= p;
        }
        if total == 0 {
            0
        } else {
            let mut res = std::usize::MAX;
            let mut index: HashMap<i32, usize> = HashMap::new();
            index.insert(0, 0);
            let mut cur = 0;
            for i in 0..n {
                cur += nums[i];
                cur %= p;
                let comp = (cur + p - total) % p;
                if let Some(j) = index.get(&comp) {
                    res = res.min(i + 1 - j);
                }
                index.insert(cur, i + 1);
            }
            if res < n {
                res as i32
            } else {
                -1
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![3, 1, 4, 2];
    let p = 6;
    let res = 1;
    assert_eq!(Solution::min_subarray(nums, p), res);
    let nums = vec![6, 3, 5, 2];
    let p = 9;
    let res = 2;
    assert_eq!(Solution::min_subarray(nums, p), res);
    let nums = vec![1, 2, 3];
    let p = 3;
    let res = 0;
    assert_eq!(Solution::min_subarray(nums, p), res);
    let nums = vec![1, 2, 3];
    let p = 7;
    let res = -1;
    assert_eq!(Solution::min_subarray(nums, p), res);
    let nums = vec![1000000000, 1000000000, 1000000000];
    let p = 3;
    let res = 0;
    assert_eq!(Solution::min_subarray(nums, p), res);
}
