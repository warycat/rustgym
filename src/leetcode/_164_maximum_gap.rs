struct Solution;

impl Solution {
    fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let mut res = 0;
        for i in 1..n {
            res = res.max(nums[i] - nums[i - 1]);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![3, 6, 9, 1];
    let res = 3;
    assert_eq!(Solution::maximum_gap(nums), res);
}
