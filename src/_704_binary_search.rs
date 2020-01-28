struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        while l <= r {
            let mid = (l + r) / 2;
            match nums[mid].cmp(&target) {
                Equal => {
                    return mid as i32;
                }
                Less => {
                    if mid + 1 > n - 1 {
                        break;
                    }
                    l = mid + 1;
                }
                Greater => {
                    if mid < 1 {
                        break;
                    }
                    r = mid - 1;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(Solution::search(nums, target), 4);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    assert_eq!(Solution::search(nums, target), -1);
}
