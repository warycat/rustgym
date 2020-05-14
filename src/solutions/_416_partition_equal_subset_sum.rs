struct Solution;

impl Solution {
    fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let half = (sum / 2) as usize;
        let mut dp: Vec<bool> = vec![false; half as usize + 1];
        dp[0] = true;
        let mut max = 0;
        for x in nums {
            let j = x as usize;
            for i in (j..=half.min(max + j)).rev() {
                if dp[i - j] {
                    dp[i] = true;
                    max = max.max(i);
                }
            }
            if dp[half] {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 11, 5];
    let res = true;
    assert_eq!(Solution::can_partition(nums), res);
    let nums = vec![1, 2, 5];
    let res = false;
    assert_eq!(Solution::can_partition(nums), res);
}
