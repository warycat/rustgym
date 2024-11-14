struct Solution;

fn lower_bound(nums: &[i32], bound: i32) -> i64 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut res = 0;
    while l < r {
        let sum = nums[l] + nums[r];
        if sum < bound {
            res += r - l;
            l += 1;
        } else {
            r -= 1;
        }
    }
    res as i64
}

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        lower_bound(&nums, upper + 1) - lower_bound(&nums, lower)
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 7, 4, 4, 5];
    let lower = 3;
    let upper = 6;
    let res = 6;
    assert_eq!(Solution::count_fair_pairs(nums, lower, upper), res);
    let nums = vec![1, 7, 9, 2, 5];
    let lower = 11;
    let upper = 11;
    let res = 1;
    assert_eq!(Solution::count_fair_pairs(nums, lower, upper), res);
    let nums = vec![0, 0, 0, 0, 0, 0];
    let lower = 0;
    let upper = 0;
    let res = 15;
    assert_eq!(Solution::count_fair_pairs(nums, lower, upper), res);
}
