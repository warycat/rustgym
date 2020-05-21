struct Solution;

impl Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);
        nums1.append(nums2);
        nums1.sort_unstable();
    }
}

#[test]
fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
