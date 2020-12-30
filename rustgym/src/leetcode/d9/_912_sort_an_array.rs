struct Solution;

impl Solution {
    fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums
    }
}

#[test]
fn test() {
    let nums = vec![5, 2, 3, 1];
    let res = vec![1, 2, 3, 5];
    assert_eq!(Solution::sort_array(nums), res);
    let nums = vec![5, 1, 1, 2, 0, 0];
    let res = vec![0, 0, 1, 1, 2, 5];
    assert_eq!(Solution::sort_array(nums), res);
}
