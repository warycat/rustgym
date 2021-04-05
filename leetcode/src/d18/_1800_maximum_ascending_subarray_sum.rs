struct Solution;

impl Solution {
    fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = nums[0];
        let mut sum = nums[0];
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
            max = max.max(sum);
        }
        max
    }
}

#[test]
fn test() {
    let nums = vec![10, 20, 30, 5, 10, 50];
    let res = 65;
    assert_eq!(Solution::max_ascending_sum(nums), res);
    let nums = vec![10, 20, 30, 40, 50];
    let res = 150;
    assert_eq!(Solution::max_ascending_sum(nums), res);
    let nums = vec![12, 17, 15, 13, 10, 11, 12];
    let res = 33;
    assert_eq!(Solution::max_ascending_sum(nums), res);
}
