struct Solution;

impl Solution {
    fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        [nums.clone(), nums].concat()
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1];
    let res = vec![1, 2, 1, 1, 2, 1];
    assert_eq!(Solution::get_concatenation(nums), res);
    let nums = vec![1, 3, 2, 1];
    let res = vec![1, 3, 2, 1, 1, 3, 2, 1];
    assert_eq!(Solution::get_concatenation(nums), res);
}
