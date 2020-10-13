struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = vec![];
        nums.sort_unstable();
        let mut i = 0;
        while i + 3 < n {
            if i > 0 && nums[i - 1] == nums[i] {
                i += 1;
                continue;
            }
            if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
                break;
            }
            if nums[n - 3] + nums[n - 2] + nums[n - 1] + nums[i] < target {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j + 2 < n {
                if j > i + 1 && nums[j - 1] == nums[j] {
                    j += 1;
                    continue;
                }
                if nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target {
                    break;
                }
                if nums[n - 2] + nums[n - 1] + nums[i] + nums[j] < target {
                    j += 1;
                    continue;
                }
                let sum2 = nums[i] + nums[j];
                let mut l = j + 1;
                let mut r = n - 1;
                while l < r {
                    let sum4 = sum2 + nums[l] + nums[r];
                    match sum4.cmp(&target) {
                        Less => {
                            l += 1;
                        }
                        Greater => {
                            r -= 1;
                        }
                        Equal => {
                            res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                            l += 1;
                            r -= 1;
                            while l < r && nums[l - 1] == nums[l] {
                                l += 1;
                            }
                            while l < r && nums[r] == nums[r + 1] {
                                r -= 1;
                            }
                        }
                    }
                }
                j += 1
            }
            i += 1
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let mut ans: Vec<Vec<i32>> = vec_vec_i32![[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]];
    let mut res = Solution::four_sum(nums, target);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}
