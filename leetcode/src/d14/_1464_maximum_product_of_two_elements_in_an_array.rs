struct Solution;

impl Solution {
    fn max_product(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        (nums[n - 1] - 1) * (nums[n - 2] - 1)
    }
}

#[test]
fn test() {
    let nums = vec![3, 4, 5, 2];
    let res = 12;
    assert_eq!(Solution::max_product(nums), res);
    let nums = vec![1, 5, 4, 5];
    let res = 16;
    assert_eq!(Solution::max_product(nums), res);
    let nums = vec![3, 7];
    let res = 12;
    assert_eq!(Solution::max_product(nums), res);
}
