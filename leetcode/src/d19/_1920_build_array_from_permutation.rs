struct Solution;

impl Solution {
    fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = nums[nums[i] as usize];
        }
        res
    }
}
#[test]
fn test() {
    let nums = vec![0, 2, 1, 5, 3, 4];
    let res = vec![0, 1, 2, 4, 5, 3];
    assert_eq!(Solution::build_array(nums), res);
    let nums = vec![5, 0, 1, 2, 3, 4];
    let res = vec![4, 5, 0, 1, 2, 3];
    assert_eq!(Solution::build_array(nums), res);
}
