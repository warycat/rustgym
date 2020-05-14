struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hm1: HashMap<i32, i32> = HashMap::new();
        let hs1: HashSet<i32> = nums1.clone().into_iter().collect();
        nums1.iter().for_each(|&x| {
            *hm1.entry(x).or_default() += 1;
        });
        let mut hm2: HashMap<i32, i32> = HashMap::new();
        let hs2: HashSet<i32> = nums2.clone().into_iter().collect();
        nums2.iter().for_each(|&x| {
            *hm2.entry(x).or_default() += 1;
        });
        let bitand = &hs1 & &hs2;
        let mut res: Vec<i32> = vec![];
        bitand.iter().for_each(|&x| {
            let c1 = hm1[&x];
            let c2 = hm2[&x];
            let min = i32::min(c1, c2);
            (0..min).for_each(|_| {
                res.push(x);
            });
        });
        res
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    assert_eq!(Solution::intersect(nums1, nums2), vec![2, 2]);
}
