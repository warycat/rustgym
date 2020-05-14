struct Solution;
use std::i32;

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0;
        }
        let mut buy: Vec<i32> = vec![0; n];
        let mut sell: Vec<i32> = vec![0; n];
        let mut rest: Vec<i32> = vec![0; n];
        sell[0] = i32::MIN;
        buy[0] = -prices[0];
        for i in 1..n {
            buy[i] = i32::max(rest[i - 1] - prices[i], buy[i - 1]);
            sell[i] = i32::max(buy[i - 1] + prices[i], sell[i - 1]);
            rest[i] = i32::max(sell[i - 1], rest[i - 1]);
        }
        i32::max(sell[n - 1], rest[n - 1])
    }
}

#[test]
fn test() {
    let prices: Vec<i32> = vec![1, 2, 3, 0, 2];
    let res = 3;
    assert_eq!(Solution::max_profit(prices), res);
}
