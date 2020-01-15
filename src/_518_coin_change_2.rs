struct Solution;

impl Solution {
    fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let n = amount + 1;
        let mut dp: Vec<i32> = vec![0; n];
        dp[0] = 1;
        for coin in coins {
            let mut sum = coin as usize;
            while sum <= amount {
                dp[sum] += dp[sum - coin as usize];
                sum += 1;
            }
        }
        dp[amount]
    }
}

#[test]
fn test() {
    let amount = 5;
    let coins = vec![1, 2, 5];
    let res = 4;
    assert_eq!(Solution::change(amount, coins), res);
    let amount = 3;
    let coins = vec![2];
    let res = 0;
    assert_eq!(Solution::change(amount, coins), res);
    let amount = 10;
    let coins = vec![10];
    let res = 1;
    assert_eq!(Solution::change(amount, coins), res);
}
