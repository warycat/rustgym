struct Solution;

impl Solution {
    fn array_sign(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(|x| x.signum()).product()
    }
}

#[test]
fn test() {
    let nums = vec![-1, -2, -3, -4, 3, 2, 1];
    let res = 1;
    assert_eq!(Solution::array_sign(nums), res);
    let nums = vec![1, 5, 0, 2, -3];
    let res = 0;
    assert_eq!(Solution::array_sign(nums), res);
    let nums = vec![-1, 1, -1, 1, -1];
    let res = -1;
    assert_eq!(Solution::array_sign(nums), res);
}
