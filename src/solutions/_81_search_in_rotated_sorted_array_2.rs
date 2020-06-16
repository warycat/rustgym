struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        if n == 0 {
            return false;
        }
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return true;
            }
            match nums[m].cmp(&nums[r]) {
                Equal => {
                    r -= 1;
                }
                Less => {
                    if nums[m] < target && nums[r] >= target {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                Greater => {
                    if nums[m] > target && nums[l] <= target {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
            }
        }
        nums[l] == target
    }
}

#[test]
fn test() {
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 0;
    let res = true;
    assert_eq!(Solution::search(nums, target), res);
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 3;
    let res = false;
    assert_eq!(Solution::search(nums, target), res);
    let nums = vec![1, 1];
    let target = 0;
    let res = false;
    assert_eq!(Solution::search(nums, target), res);
    let nums = vec![1, 3, 1, 1];
    let target = 3;
    let res = true;
    assert_eq!(Solution::search(nums, target), res);
}
