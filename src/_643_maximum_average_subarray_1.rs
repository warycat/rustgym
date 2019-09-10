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
        f64::from(max) / k as f64
    }
}

#[test]
fn test() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    assert_eq!(
        Solution::find_max_average(nums, k) - 12.75 < std::f64::EPSILON,
        true
    );
}
