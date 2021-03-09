struct Solution;

impl Solution {
    fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut new_nums = vec![1; n + 2];
        new_nums[1..n + 1].clone_from_slice(&nums);
        let n = new_nums.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![0; n]; n];
        Self::dp(0, n - 1, &mut memo, &new_nums)
    }

    fn dp(l: usize, r: usize, memo: &mut Vec<Vec<i32>>, nums: &[i32]) -> i32 {
        if l + 1 == r {
            return 0;
        }
        if memo[l][r] != 0 {
            return memo[l][r];
        }
        let mut res = 0;
        for i in l + 1..r {
            let sum = nums[l] * nums[i] * nums[r]
                + Self::dp(l, i, memo, nums)
                + Self::dp(i, r, memo, nums);
            res = res.max(sum);
        }
        memo[l][r] = res;
        res
    }
}

#[test]
fn test() {
    let nums = vec![3, 1, 5, 8];
    let res = 167;
    assert_eq!(Solution::max_coins(nums), res);
}
