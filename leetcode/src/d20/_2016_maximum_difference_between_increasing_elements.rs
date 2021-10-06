struct Solution;

impl Solution {
    fn maximum_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = -1;
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] < nums[j] {
                    res = res.max(nums[j] - nums[i]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![7, 1, 5, 4];
    let res = 4;
    assert_eq!(Solution::maximum_difference(nums), res);
    let nums = vec![9, 4, 3, 2];
    let res = -1;
    assert_eq!(Solution::maximum_difference(nums), res);
    let nums = vec![1, 5, 2, 10];
    let res = 9;
    assert_eq!(Solution::maximum_difference(nums), res);
}
