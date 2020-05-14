struct Solution;

impl Solution {
    fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let k = k as usize;
        let n = nums.len();
        for i in 0..k {
            sum += nums[i];
        }
        let mut max = sum;
        for i in k..n {
            sum += nums[i];
            sum -= nums[i - k];
            max = i32::max(sum, max);
        }
        max as f64 / k as f64
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    assert_approx_eq!(Solution::find_max_average(nums, k), 12.75);
}
