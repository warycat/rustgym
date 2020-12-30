struct Solution;

impl Solution {
    fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        let mut total = 0;
        for i in 1..n {
            let a = nums[i - 1];
            let b = nums[i];
            total += (a - b).abs();
            res = res.max((b - nums[0]).abs() - (a - b).abs());
            res = res.max((a - nums[n - 1]).abs() - (a - b).abs());
            min = min.min(a.max(b));
            max = max.max(a.min(b));
        }
        total + res.max((max - min) * 2)
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 5, 4];
    let res = 10;
    assert_eq!(Solution::max_value_after_reverse(nums), res);
    let nums = vec![2, 4, 9, 24, 2, 1, 10];
    let res = 68;
    assert_eq!(Solution::max_value_after_reverse(nums), res);
}
