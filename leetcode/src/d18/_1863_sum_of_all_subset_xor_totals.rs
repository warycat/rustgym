struct Solution;

impl Solution {
    fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut mask = 0;
        let n = nums.len();
        for i in 0..n {
            mask |= nums[i];
        }
        mask << (n - 1)
    }
}

#[test]
fn test() {
    let nums = vec![1, 3];
    let res = 6;
    assert_eq!(Solution::subset_xor_sum(nums), res);
    let nums = vec![5, 1, 6];
    let res = 28;
    assert_eq!(Solution::subset_xor_sum(nums), res);
}
