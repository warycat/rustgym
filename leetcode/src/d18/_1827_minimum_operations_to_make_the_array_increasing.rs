struct Solution;

impl Solution {
    fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 1..n {
            let target = nums[i - 1] + 1;
            if nums[i] < target {
                res += target - nums[i];
                nums[i] = target;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1];
    let res = 3;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![1, 5, 2, 4, 1];
    let res = 14;
    assert_eq!(Solution::min_operations(nums), res);
    let nums = vec![8];
    let res = 0;
    assert_eq!(Solution::min_operations(nums), res);
}
