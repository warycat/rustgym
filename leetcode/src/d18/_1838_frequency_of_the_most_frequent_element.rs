struct Solution;

impl Solution {
    fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        let mut res = 0;
        while j < n {
            sum += nums[j];
            j += 1;
            while (j - i) as i32 * nums[j - 1] - sum > k {
                sum -= nums[i];
                i += 1;
            }
            res = res.max(j - i);
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 4];
    let k = 5;
    let res = 3;
    assert_eq!(Solution::max_frequency(nums, k), res);
    let nums = vec![1, 4, 8, 13];
    let k = 5;
    let res = 2;
    assert_eq!(Solution::max_frequency(nums, k), res);
    let nums = vec![3, 9, 6];
    let k = 2;
    let res = 1;
    assert_eq!(Solution::max_frequency(nums, k), res);
}
