struct Solution;

impl Solution {
    fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>() as usize;
        let mut dp = vec![false; sum + 1];
        dp[0] = true;
        let n = stones.len();
        for i in 0..n {
            for j in (1..=sum).rev() {
                if j >= stones[i] as usize && dp[j - stones[i] as usize] {
                    dp[j] = true;
                }
            }
        }
        let mut res = sum;
        for i in 0..=sum / 2 {
            if dp[i] {
                res = res.min(sum - 2 * i);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    let res = 1;
    assert_eq!(Solution::last_stone_weight_ii(stones), res);
}
