struct Solution;

impl Solution {
    fn min_start_value(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut start = 1;
        for i in 0..n {
            sum += nums[i];
            if start + sum < 1 {
                start = start.max(1 - sum);
            }
        }
        start
    }
}

#[test]
fn test() {
    let nums = vec![-3, 2, -3, 4, 2];
    let res = 5;
    assert_eq!(Solution::min_start_value(nums), res);
}
