struct Solution;

impl Solution {
    fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut j = 0;
        for i in 1..=n - k {
            if nums[i] > nums[j] {
                j = i;
            }
        }
        nums[j..j + k].to_vec()
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 5, 2, 3];
    let k = 3;
    let res = vec![5, 2, 3];
    assert_eq!(Solution::largest_subarray(nums, k), res);
    let nums = vec![1, 4, 5, 2, 3];
    let k = 4;
    let res = vec![4, 5, 2, 3];
    assert_eq!(Solution::largest_subarray(nums, k), res);
    let nums = vec![1, 4, 5, 2, 3];
    let k = 1;
    let res = vec![5];
    assert_eq!(Solution::largest_subarray(nums, k), res);
}
