struct Solution;

use std::collections::HashSet;

impl Solution {
    fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let h1: HashSet<i32> = nums1.into_iter().collect();
        let h2: HashSet<i32> = nums2.into_iter().collect();
        let bitand = &h1 & &h2;
        bitand.into_iter().collect()
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    assert_eq!(Solution::intersection(nums1, nums2), vec![2]);
}
