struct Solution;

impl Solution {
    fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prev = nums[0];
        let mut prev_sum = 0;
        let mut res = vec![0; n];
        for i in 0..n {
            let diff = nums[i] - prev;
            prev_sum += diff * i as i32;
            res[i] += prev_sum;
            prev = nums[i];
        }
        prev = nums[n - 1];
        prev_sum = 0;
        for i in (0..n).rev() {
            let diff = (nums[i] - prev).abs();
            prev_sum += diff * (n - 1 - i) as i32;
            res[i] += prev_sum;
            prev = nums[i];
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 5];
    let res = vec![4, 3, 5];
    assert_eq!(Solution::get_sum_absolute_differences(nums), res);
    let nums = vec![1, 4, 6, 8, 10];
    let res = vec![24, 15, 13, 15, 21];
    assert_eq!(Solution::get_sum_absolute_differences(nums), res);
}
