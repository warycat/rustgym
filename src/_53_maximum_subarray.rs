struct Solution;

impl Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut max = std::i32::MIN;
        let n = nums.len();
        for i in 0..n {
            prev = nums[i].max(prev + nums[i]);
            max = max.max(prev);
        }
        max
    }
}

#[test]
fn test() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);
    let nums = vec![-1];
    assert_eq!(Solution::max_sub_array(nums), -1);
    let nums = vec![1];
    assert_eq!(Solution::max_sub_array(nums), 1);
}
