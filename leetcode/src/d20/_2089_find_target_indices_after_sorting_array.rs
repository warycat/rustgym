struct Solution;

impl Solution {
    fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = vec![];
        for i in 0..n {
            if nums[i] == target {
                res.push(i as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 5, 2, 3];
    let target = 2;
    let res = vec![1, 2];
    assert_eq!(Solution::target_indices(nums, target), res);
    let nums = vec![1, 2, 5, 2, 3];
    let target = 3;
    let res = vec![3];
    assert_eq!(Solution::target_indices(nums, target), res);
    let nums = vec![1, 2, 5, 2, 3];
    let target = 5;
    let res = vec![4];
    assert_eq!(Solution::target_indices(nums, target), res);
}
