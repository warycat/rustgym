struct Solution;

use std::collections::BTreeSet;
use std::collections::HashSet;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let set1: HashSet<i32> = nums1.iter().copied().collect();
        let set2: HashSet<i32> = nums2.iter().copied().collect();
        let all: BTreeSet<i32> = nums1.iter().copied().chain(nums2.iter().copied()).collect();
        let mut prev1 = 0;
        let mut prev2 = 0;
        let mut res = 0;
        for x in all {
            if set1.contains(&x) {
                prev1 += x as i64;
            }
            if set2.contains(&x) {
                prev2 += x as i64;
            }
            if set1.contains(&x) && set2.contains(&x) {
                let max = prev1.max(prev2);
                prev1 = max;
                prev2 = max;
            }
            res = res.max(prev1);
            res = res.max(prev2);
        }
        res %= MOD;
        res as i32
    }
}

#[test]
fn test() {
    let nums1 = vec![2, 4, 5, 8, 10];
    let nums2 = vec![4, 6, 8, 9];
    let res = 30;
    assert_eq!(Solution::max_sum(nums1, nums2), res);
    let nums1 = vec![1, 3, 5, 7, 9];
    let nums2 = vec![3, 5, 100];
    let res = 109;
    assert_eq!(Solution::max_sum(nums1, nums2), res);
}
