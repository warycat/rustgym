struct Solution;

use std::collections::HashSet;

impl Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        let n = nums.len();
        let k = k as usize;
        for i in 0..n {
            let x = nums[i];
            if hs.contains(&x) {
                return true;
            } else {
                hs.insert(x);
                if hs.len() > k {
                    hs.remove(&nums[i - k]);
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::contains_nearby_duplicate(nums, 3), true);
    let nums = vec![1, 0, 1, 1];
    assert_eq!(Solution::contains_nearby_duplicate(nums, 1), true);
    let nums = vec![1, 2, 3, 1, 2, 3];
    assert_eq!(Solution::contains_nearby_duplicate(nums, 2), false);
}
