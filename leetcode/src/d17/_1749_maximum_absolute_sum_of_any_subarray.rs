struct Solution;

impl Solution {
    fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut min = 0;
        let mut max = 0;
        let mut sub_min = 0;
        let mut sub_max = 0;
        let mut res = 0;
        for i in 0..n {
            sum += nums[i];
            min = min.min(sum);
            max = max.max(sum);
            sub_max = sub_max.max(sum - min);
            sub_min = sub_min.min(sum - max);
            res = res.max(sub_max.abs());
            res = res.max(sub_min.abs());
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, -3, 2, 3, -4];
    let res = 5;
    assert_eq!(Solution::max_absolute_sum(nums), res);
    let nums = vec![2, -5, 1, -4, 3, -2];
    let res = 8;
    assert_eq!(Solution::max_absolute_sum(nums), res);
}
