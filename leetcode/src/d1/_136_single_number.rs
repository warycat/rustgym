struct Solution;

impl Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0i32, |res, val| res ^ val)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}
