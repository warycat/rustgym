struct Solution;

impl Solution {
    fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut prev_left = 0;
        for i in 0..n {
            left[i] = prev_left;
            if nums[i] == 0 {
                prev_left = 0;
            } else {
                prev_left += 1;
            }
        }
        let mut prev_right = 0;
        for i in (0..n).rev() {
            right[i] = prev_right;
            if nums[i] == 0 {
                prev_right = 0;
            } else {
                prev_right += 1;
            }
        }
        let mut res = 0;
        for i in 0..n {
            res = res.max(left[i] + right[i]);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 0, 1];
    let res = 3;
    assert_eq!(Solution::longest_subarray(nums), res);
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let res = 5;
    assert_eq!(Solution::longest_subarray(nums), res);
    let nums = vec![1, 1, 1];
    let res = 2;
    assert_eq!(Solution::longest_subarray(nums), res);
    let nums = vec![1, 1, 0, 0, 1, 1, 1, 0, 1];
    let res = 4;
    assert_eq!(Solution::longest_subarray(nums), res);
    let nums = vec![0, 0, 0];
    let res = 0;
    assert_eq!(Solution::longest_subarray(nums), res);
}
