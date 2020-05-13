struct Solution;
use std::collections::BTreeSet;

impl Solution {
    fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut bts = BTreeSet::new();
        let k = k as usize;
        let n = nums.len();
        if t < 0 {
            return false;
        }
        for i in 0..n {
            let l = nums[i].checked_sub(t).unwrap_or(std::i32::MIN);
            let r = nums[i].checked_add(t).unwrap_or(std::i32::MAX);
            if bts.range(l..=r).next().is_none() {
                bts.insert(nums[i]);
            } else {
                return true;
            }
            if i >= k {
                bts.remove(&nums[i - k]);
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let t = 0;
    let res = true;
    assert_eq!(Solution::contains_nearby_almost_duplicate(nums, k, t), res);
    let nums = vec![1, 0, 1, 1];
    let k = 1;
    let t = 2;
    let res = true;
    assert_eq!(Solution::contains_nearby_almost_duplicate(nums, k, t), res);
    let nums = vec![1, 5, 9, 1, 5, 9];
    let k = 2;
    let t = 3;
    let res = false;
    assert_eq!(Solution::contains_nearby_almost_duplicate(nums, k, t), res);
    let nums = vec![-1, -1];
    let k = 1;
    let t = -1;
    let res = false;
    assert_eq!(Solution::contains_nearby_almost_duplicate(nums, k, t), res);
    let nums = vec![7, 1, 3];
    let k = 2;
    let t = 3;
    let res = true;
    assert_eq!(Solution::contains_nearby_almost_duplicate(nums, k, t), res);
    let nums = vec![0, 2_147_483_647];
    let k = 1;
    let t = 2_147_483_647;
    let res = true;
    assert_eq!(Solution::contains_nearby_almost_duplicate(nums, k, t), res);
}
