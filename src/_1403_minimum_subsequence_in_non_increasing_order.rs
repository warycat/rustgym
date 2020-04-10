struct Solution;

impl Solution {
    fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.sort_unstable();
        let sum: i32 = nums.iter().sum();
        let mut res = vec![];
        let mut cur = 0;
        for i in (0..n).rev() {
            if cur * 2 <= sum {
                res.push(nums[i]);
            }
            cur += nums[i];
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![4, 3, 10, 9, 8];
    let res = vec![10, 9];
    assert_eq!(Solution::min_subsequence(nums), res);
    let nums = vec![4, 4, 7, 6, 7];
    let res = vec![7, 7, 6];
    assert_eq!(Solution::min_subsequence(nums), res);
    let nums = vec![6];
    let res = vec![6];
    assert_eq!(Solution::min_subsequence(nums), res);
}
