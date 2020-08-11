struct Solution;

impl Solution {
    fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![];
        let mut prev = 0;
        for i in 0..n {
            prev += nums[i];
            res.push(prev);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = vec![1, 3, 6, 10];
    assert_eq!(Solution::running_sum(nums), res);
    let nums = vec![1, 1, 1, 1, 1];
    let res = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::running_sum(nums), res);
    let nums = vec![3, 1, 2, 10, 1];
    let res = vec![3, 4, 6, 16, 17];
    assert_eq!(Solution::running_sum(nums), res);
}
