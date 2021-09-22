struct Solution;

impl Solution {
    fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        nums[n - 1] * nums[n - 2] - nums[0] * nums[1]
    }
}

#[test]
fn test() {
    let nums = vec![5, 6, 2, 7, 4];
    let res = 34;
    assert_eq!(Solution::max_product_difference(nums), res);
    let nums = vec![4, 2, 5, 9, 7, 4, 8];
    let res = 64;
    assert_eq!(Solution::max_product_difference(nums), res);
}
