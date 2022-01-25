struct Solution;

use std::collections::HashSet;

impl Solution {
    fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let s1: HashSet<i32> = nums1.into_iter().collect();
        let s2: HashSet<i32> = nums2.into_iter().collect();
        let s3: HashSet<i32> = nums3.into_iter().collect();
        let mut res: HashSet<i32> = HashSet::new();
        for &x in s1.intersection(&s2) {
            res.insert(x);
        }
        for &x in s2.intersection(&s3) {
            res.insert(x);
        }
        for &x in s3.intersection(&s1) {
            res.insert(x);
        }
        res.into_iter().collect()
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 1, 3, 2];
    let nums2 = vec![2, 3];
    let nums3 = vec![3];
    let mut res = vec![3, 2];
    let mut ans = Solution::two_out_of_three(nums1, nums2, nums3);
    res.sort_unstable();
    ans.sort_unstable();

    assert_eq!(res, ans);
    let nums1 = vec![3, 1];
    let nums2 = vec![2, 3];
    let nums3 = vec![1, 2];
    let mut res = vec![2, 3, 1];
    let mut ans = Solution::two_out_of_three(nums1, nums2, nums3);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(res, ans);

    let nums1 = vec![1, 2, 2];
    let nums2 = vec![4, 3, 3];
    let nums3 = vec![5];
    let mut res: Vec<i32> = vec![];
    let mut ans = Solution::two_out_of_three(nums1, nums2, nums3);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(res, ans);
}
