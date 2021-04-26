struct Solution;

use std::collections::BTreeSet;
use std::iter::FromIterator;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let bts: BTreeSet<i32> = BTreeSet::from_iter(nums1.clone());
        let n = nums1.len();
        let mut max = 0;
        let mut sum: i64 = 0;
        for i in 0..n {
            let x = nums2[i];
            let y = nums1[i];
            let z = (x - y).abs();
            sum += z as i64;
            if let Some(left) = bts.range(..=x).next_back() {
                if (x - left) < z {
                    max = max.max(z - (x - left));
                }
            }
            if let Some(right) = bts.range(x..).next() {
                if (right - x) < z {
                    max = max.max(z - (right - x));
                }
            }
        }
        ((sum - max as i64) % MOD) as i32
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 7, 5];
    let nums2 = vec![2, 3, 5];
    let res = 3;
    assert_eq!(Solution::min_absolute_sum_diff(nums1, nums2), res);
    let nums1 = vec![2, 4, 6, 8, 10];
    let nums2 = vec![2, 4, 6, 8, 10];
    let res = 0;
    assert_eq!(Solution::min_absolute_sum_diff(nums1, nums2), res);
    let nums1 = vec![1, 10, 4, 4, 2, 7];
    let nums2 = vec![9, 3, 5, 1, 7, 4];
    let res = 20;
    assert_eq!(Solution::min_absolute_sum_diff(nums1, nums2), res);
}
