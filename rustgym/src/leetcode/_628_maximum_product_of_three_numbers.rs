struct Solution;

impl Solution {
    fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        i32::max(
            nums[0] * nums[1] * nums[n - 1],
            nums[n - 1] * nums[n - 2] * nums[n - 3],
        )
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::maximum_product(nums), 6);
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::maximum_product(nums), 24);
}
