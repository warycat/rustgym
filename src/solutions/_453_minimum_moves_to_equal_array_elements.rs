struct Solution;

impl Solution {
    fn min_moves(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut sum = 0;
        for &x in &nums {
            min = i32::min(x, min);
        }
        for &x in &nums {
            sum += x - min;
        }
        sum
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::min_moves(nums), 3);
}
