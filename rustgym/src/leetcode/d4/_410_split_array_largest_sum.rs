struct Solution;

impl Solution {
    fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut lo = *nums.iter().max().unwrap();
        let mut hi = nums.iter().sum();
        let n = nums.len();
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if Self::split(&nums, mid, n) <= m {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    fn split(nums: &[i32], max: i32, n: usize) -> i32 {
        let mut sum = 0;
        let mut res = 1;
        for i in 0..n {
            if nums[i] + sum > max {
                sum = nums[i];
                res += 1;
            } else {
                sum += nums[i];
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![7, 2, 5, 10, 8];
    let m = 2;
    let res = 18;
    assert_eq!(Solution::split_array(nums, m), res);
    let nums = vec![1, 2, 3, 4, 5];
    let m = 2;
    let res = 9;
    assert_eq!(Solution::split_array(nums, m), res);
    let nums = vec![1, 4, 4];
    let m = 3;
    let res = 4;
    assert_eq!(Solution::split_array(nums, m), res);
    let nums = vec![2, 3, 1, 2, 4, 3];
    let m = 5;
    let res = 4;
    assert_eq!(Solution::split_array(nums, m), res);
}
