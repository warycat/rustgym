struct Solution;

impl Solution {
    fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        Self::rob_slice(&nums[0..n - 1]).max(Self::rob_slice(&nums[1..n]))
    }
    fn rob_slice(v: &[i32]) -> i32 {
        let n = v.len();
        let mut prev = 0;
        let mut curr = 0;
        for i in 0..n {
            let temp = curr.max(v[i] + prev);
            prev = curr;
            curr = temp;
        }
        curr
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 2];
    let res = 3;
    assert_eq!(Solution::rob(nums), res);
    let nums = vec![1, 2, 3, 1];
    let res = 4;
    assert_eq!(Solution::rob(nums), res);
    let nums = vec![0];
    let res = 0;
    assert_eq!(Solution::rob(nums), res);
    let nums = vec![];
    let res = 0;
    assert_eq!(Solution::rob(nums), res);
    let nums = vec![1];
    let res = 1;
    assert_eq!(Solution::rob(nums), res);
}
